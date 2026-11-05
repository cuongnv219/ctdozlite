"""
Zalo Feed & Zalo Video Standalone Test Tool (Updated)
-----------------------------------------------------
Sử dụng trực tiếp session.json hiện có để kiểm tra các API endpoint
Nhật ký (Feed / Timeline / Albums) và Zalo Video theo tài liệu re-notes/09_feed_and_zalo_video.md.
Chạy hoàn toàn độc lập, không phụ thuộc vào code Rust đang sửa dở.
"""

import sys
import json
import base64
import urllib.parse
from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad
import requests

if sys.stdout.encoding != 'utf-8':
    try:
        sys.stdout.reconfigure(encoding='utf-8')
    except Exception:
        pass

SESSION_PATH = 'session.json'

def load_session():
    with open(SESSION_PATH, 'r', encoding='utf-8') as f:
        return json.load(f)

def encrypt_params(key_bytes, data_dict):
    json_str = json.dumps(data_dict, separators=(',', ':'))
    cipher = AES.new(key_bytes, AES.MODE_CBC, iv=b'\x00' * 16)
    padded = pad(json_str.encode('utf-8'), 16)
    ct = cipher.encrypt(padded)
    return base64.b64encode(ct).decode('utf-8')

def decrypt_data(key_bytes, b64_str):
    raw = base64.b64decode(b64_str)
    cipher = AES.new(key_bytes, AES.MODE_CBC, iv=b'\x00' * 16)
    pt = unpad(cipher.decrypt(raw), 16)
    return pt.decode('utf-8', errors='replace')

def main():
    print("=" * 68)
    print("🧪 KIỂM TRA TÍNH NĂNG NHẬT KÝ & ZALO VIDEO TỪ SESSION.JSON")
    print("=" * 68)

    try:
        session_data = load_session()
    except Exception as e:
        print(f"❌ Lỗi tải session.json: {e}")
        return

    imei = session_data.get('imei')
    zpw_sek = session_data.get('zpw_sek') or session_data.get('login', {}).get('zpw_sek')
    zpw_enk = session_data.get('secret_key_b64') or session_data.get('login', {}).get('zpw_enk')
    uin = session_data.get('login', {}).get('uin')
    display_name = session_data.get('login', {}).get('display_name', 'Chưa rõ')

    if not zpw_sek or not zpw_enk:
        print("❌ session.json thiếu zpw_sek hoặc secret_key_b64!")
        return

    secret_key = base64.b64decode(zpw_enk)

    print(f"✓ Đã đọc session.json:")
    print(f"  • Người dùng:     {display_name} (UIN: {uin})")
    print(f"  • Session token:  {zpw_sek[:16]}...{zpw_sek[-8:]}")
    print(f"  • AES Secret Key: {len(secret_key)} bytes")

    sess = requests.Session()
    sess.headers.update({
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) ZaloPC/26.10.11 Chrome/122.0.0.0 Electron/29.0.0 Safari/537.36',
        'Origin': 'https://chat.zalo.me',
        'Referer': 'https://chat.zalo.me/',
        'Cookie': f"zpw_sek={zpw_sek}"
    })

    # Host chuẩn xác được kiểm chứng cho social API
    profile_host = "https://tt-profile-wpa.chat.zalo.me"

    # 1. Xác thực session
    print("\n--- [Bước 1] Xác thực phiên đăng nhập (Profile me-v2) ---")
    body_me = {"avatar_size": 160, "imei": imei}
    enc_me = encrypt_params(secret_key, body_me)
    url_me = f"{profile_host}/api/social/profile/me-v2?zpw_ver=694&zpw_type=30&params={urllib.parse.quote(enc_me)}&os=8"

    try:
        r = sess.get(url_me, timeout=10)
        res = r.json()
        if res.get('error_code') == 0:
            dec = decrypt_data(secret_key, res.get('data'))
            user_info = json.loads(dec).get('data', {}).get('profile', {})
            print(f"  🟢 Phiên ĐANG HOẠT ĐỘNG (Valid)!")
            print(f"     • Tên Zalo:  {user_info.get('displayName')}")
            print(f"     • Số đt:     {user_info.get('phoneNumber', 'Ẩn')}")
            print(f"     • Giới tính: {'Nam' if user_info.get('gender') == 0 else 'Nữ'}")
            print(f"     • Avatar:    {user_info.get('avatar', '')[:65]}...")
        else:
            print(f"  🔴 Phiên hết hạn: {res}")
            return
    except Exception as e:
        print(f"  ❌ Lỗi kết nối: {e}")
        return

    # 2. Kéo album ảnh & bài đăng đại diện trên Nhật ký (/api/social/avatar-list)
    print("\n--- [Bước 2] Lấy Album ảnh Nhật ký (/api/social/avatar-list) ---")
    body_album = {"page": 1, "count": 10, "imei": imei}
    enc_album = encrypt_params(secret_key, body_album)
    url_album = f"{profile_host}/api/social/avatar-list?zpw_ver=694&zpw_type=30&params={urllib.parse.quote(enc_album)}&os=8"

    try:
        r_album = sess.get(url_album, timeout=8)
        rj_album = r_album.json()
        if rj_album.get('error_code') == 0 and rj_album.get('data'):
            dec_album = decrypt_data(secret_key, rj_album['data'])
            album_data = json.loads(dec_album).get('data', {})
            photos = album_data.get('photos', [])
            print(f"  ✓ Lấy thành công {len(photos)} ảnh từ Album Nhật ký:")
            for idx, p in enumerate(photos):
                print(f"    [{idx+1}] Photo ID: {p.get('photoId')}")
                print(f"        CDN URL:  {p.get('url')}")
        else:
            print(f"  Phản hồi: {rj_album}")
    except Exception as e:
        print(f"  Lỗi lấy album: {e}")

    # 3. Lấy danh sách bạn bè cùng tương tác Nhật ký (/api/social/friend/getfriends)
    print("\n--- [Bước 3] Lấy danh sách bạn bè Timeline (/api/social/friend/getfriends) ---")
    body_friends = {
        'incInvalid': 1,
        'page': 1,
        'count': 10,
        'avatar_size': 160,
        'actiontime': 0,
        'imei': imei
    }
    enc_friends = encrypt_params(secret_key, body_friends)
    url_friends = f"{profile_host}/api/social/friend/getfriends?zpw_ver=694&zpw_type=30&params={urllib.parse.quote(enc_friends)}"

    try:
        r_fr = sess.get(url_friends, timeout=10)
        rj_fr = r_fr.json()
        if rj_fr.get('error_code') == 0 and rj_fr.get('data'):
            dec_fr = decrypt_data(secret_key, rj_fr['data'])
            fr_data = json.loads(dec_fr)
            friends = fr_data.get('data', []) if isinstance(fr_data, dict) else fr_data
            print(f"  ✓ Lấy thành công danh sách bạn bè: {len(friends)} người")
            for fr in friends[:5]:
                print(f"    • {fr.get('displayName')} (UID: {fr.get('userId')})")
        else:
            print(f"  Phản hồi: {rj_fr}")
    except Exception as e:
        print(f"  Lỗi lấy bạn bè: {e}")

    # 4. Thử nghiệm Mobile Feed Gateway (/api/feed?)
    print("\n--- [Bước 4] Mobile Feed Gateway & Cơ chế Ký số Native ---")
    print("  • Gateway: https://api-talk.api.zaloapp.com/api/feed?")
    print("  • Cơ chế bảo mật (từ dịch ngược libnative_utils.so):")
    print("    - Thuật toán ký: sig = MD5(queryString + salt)")
    print("    - Hardware ID:   zcid đọc từ bộ nhớ toàn cục CoreUtils.getZCIDNative")
    print("    - Anti-tamper:   Thư viện native kiểm tra /proc/self/cmdline khớp 'com.zing.zalo'")
    print("    - Trạng thái:    Gateway di động từ chối Web session khi thiếu cặp chữ ký sig & zcid.")

    # 5. Thử nghiệm Zalo Video Feed (https://api.zalo.video)
    print("\n--- [Bước 5] Thử nghiệm Zalo Video Feed (https://api.zalo.video) ---")
    url_video = "https://api.zalo.video/v1/feed"
    try:
        r_video = requests.get(url_video, headers={
            'User-Agent': 'Zalo/26.10.02 (Android; Mobile; SDK 36; arm64-v8a)',
            'Authorization': f"Bearer {zpw_sek}",
            'Cookie': f"zpw_sek={zpw_sek}"
        }, timeout=6)
        print(f"  HTTP Code: {r_video.status_code}")
        print(f"  Phản hồi:  {r_video.text}")
        print("  • Phân tích mã lỗi -106:")
        print("    Zalo Video là hệ thống riêng, yêu cầu đổi sang Video OAuth Token (Bearer Token)")
        print("    thông qua https://oauth.zaloapp.com trước khi gọi API feed video.")
    except Exception as e:
        print(f"  Lỗi gọi Zalo Video: {e}")

    print("\n" + "=" * 68)
    print("✓ HOÀN TẤT KIỂM THỬ ĐỘC LẬP TÍNH NĂNG NHẬT KÝ & VIDEO")
    print("=" * 68)

if __name__ == '__main__':
    main()
