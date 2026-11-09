// Sync V2 helper: sinh key + giải mã metadata/message bằng WASM libzproto của Zalo.
// Chạy qua Node từ Rust (history_sync.rs). Yêu cầu: Node.js + file WASM lấy từ
// bản cài Zalo PC trên máy (KHÔNG đóng gói WASM theo repo — chính sách HistoryCodec.swift:365).
//
// Luồng đúng theo HistoryCodec.swift (JSCore mẫu):
//   batch = [u32 LE len][chunk]... → decrypt TỪNG chunk → unpack (zstd/zlib/gzip
//   tự nhận dạng) từng plaintext → protobuf SyncChunk.

const fs = require('fs');
const os = require('os');
const path = require('path');
const zlib = require('zlib');
const zproto = require('./zproto_lib.js');

const TEMP_DIR = os.tmpdir();
const KEYS_FILE = path.join(TEMP_DIR, "sync_v2_keys.json");
const SESSION_FILE = path.join(TEMP_DIR, "sync_v2_session.bin");
const MESSAGE_FILE = path.join(TEMP_DIR, "sync_v2_message.bin");

// Tìm WASM libzproto theo thứ tự ưu tiên:
//   1. CTDOZLITE_ZPROTO_WASM (đường dẫn tuyệt đối do user/dev chỉ định)
//   2. Zalo PC đã cài trên máy (glob hash bất kỳ — hash đổi mỗi release Zalo)
//   3. Thư mục assets cạnh script (dev tự copy vào, không commit)
function findWasm() {
    const override = process.env.CTDOZLITE_ZPROTO_WASM;
    if (override && fs.existsSync(override)) return override;
    const candidates = [];
    if (process.platform === 'darwin') {
        candidates.push('/Applications/Zalo.app/Contents/Resources/app.asar.unpacked');
        candidates.push(path.join(os.homedir(), 'Applications/Zalo.app/Contents/Resources/app.asar.unpacked'));
    } else if (process.platform === 'win32') {
        const local = process.env.LOCALAPPDATA || '';
        const prog = process.env.ProgramFiles || '';
        candidates.push(path.join(local, 'Programs', 'Zalo'));
        candidates.push(path.join(prog, 'Zalo'));
    }
    // Dev: copy wasm vào scripts/assets/ (gitignored)
    candidates.push(path.join(__dirname, 'assets'));
    for (const root of candidates) {
        if (!root || !fs.existsSync(root)) continue;
        const hit = globWasm(root, 4);
        if (hit) return hit;
    }
    return null;
}

function globWasm(dir, depth) {
    let entries;
    try {
        entries = fs.readdirSync(dir, { withFileTypes: true });
    } catch (_) {
        return null;
    }
    for (const e of entries) {
        const full = path.join(dir, e.name);
        if (e.isFile() && /^libzproto_wasm_bg\..*\.wasm$/.test(e.name)) return full;
    }
    if (depth <= 0) return null;
    for (const e of entries) {
        if (e.isDirectory() && e.name !== 'node_modules') {
            const hit = globWasm(full(e, dir), depth - 1);
            if (hit) return hit;
        }
    }
    return null;
    function full(e, dir) { return path.join(dir, e.name); }
}

let wasmReady = false;

async function initWasm() {
    if (wasmReady) return;
    const wasmPath = findWasm();
    if (!wasmPath) {
        throw new Error(
            'Không tìm thấy libzproto_wasm_bg.*.wasm. Cài Zalo PC hoặc đặt CTDOZLITE_ZPROTO_WASM trỏ tới file WASM.'
        );
    }
    const wasmBuf = fs.readFileSync(wasmPath);
    await zproto.b(wasmBuf);
    wasmReady = true;
}

async function generateV2Keys() {
    await initWasm();
    const ik = zproto.e();
    const ek = zproto.e();
    const res = {
        ik_pub: Buffer.from(ik.publicKey).toString('base64'),
        ik_priv: Buffer.from(ik.privateKey).toString('hex'),
        ek_pub: Buffer.from(ek.publicKey).toString('base64'),
        ek_priv: Buffer.from(ek.privateKey).toString('hex')
    };
    ik.free();
    ek.free();
    fs.writeFileSync(KEYS_FILE, JSON.stringify(res, null, 2));
    return res;
}

// Tách batch thành các chunk [u32 LE len][bytes] (HistoryCodec _split).
function splitFrames(blob) {
    const chunks = [];
    let pos = 0;
    while (pos < blob.length) {
        if (blob.length - pos < 4) throw new Error('Truncated frame header');
        const len = blob.readUInt32LE(pos);
        pos += 4;
        if (!len || len > blob.length - pos) throw new Error('Invalid frame length');
        chunks.push(blob.subarray(pos, pos + len));
        pos += len;
    }
    if (!chunks.length) throw new Error('Empty batch');
    return chunks;
}

// Giải nén tự nhận dạng: zstd (28 b5 2f fd) / gzip (1f 8b) / zlib (78 xx) / raw.
function unpack(buf) {
    if (buf.length >= 4 && buf[0] === 0x28 && buf[1] === 0xb5 && buf[2] === 0x2f && buf[3] === 0xfd) {
        if (typeof zlib.zstdDecompressSync === 'function') {
            return zlib.zstdDecompressSync(buf);
        }
        throw new Error('Chunk nén zstd nhưng Node.js quá cũ (cần >= 22 để có zstdDecompressSync)');
    }
    if (buf.length >= 2 && buf[0] === 0x1f && buf[1] === 0x8b) {
        return zlib.gunzipSync(buf);
    }
    if (buf.length >= 2 && buf[0] === 0x78) {
        try {
            return zlib.inflateSync(buf);
        } catch (_) {
            return buf;
        }
    }
    return buf;
}

async function decryptMeta(metaFilePath) {
    await initWasm();
    if (!fs.existsSync(KEYS_FILE)) {
        throw new Error('sync_v2_keys.json not found — chạy gen-keys trước');
    }
    const keys = JSON.parse(fs.readFileSync(KEYS_FILE, 'utf8'));
    const metaBytes = fs.readFileSync(metaFilePath);
    const ik_pub = Buffer.from(keys.ik_pub, 'base64');
    const ik_priv = Buffer.from(keys.ik_priv, 'hex');
    const ek_priv = Buffer.from(keys.ek_priv, 'hex');
    // Meta thường 1 chunk nhưng vẫn split cho đúng framing.
    const chunks = splitFrames(metaBytes);
    let last = null;
    for (const chunk of chunks) {
        const res = zproto.j(chunk, ik_pub, ik_priv, ek_priv, null);
        last = {
            sessionId: res.sessionId ? String(res.sessionId) : '',
            sessionRecord: Buffer.from(res.sessionRecord),
            plaintext: Buffer.from(res.plaintext),
            senderIdentity: res.senderIdentity ? String(res.senderIdentity) : ''
        };
        res.free();
    }
    fs.writeFileSync(SESSION_FILE, last.sessionRecord);
    return {
        sessionId: last.sessionId,
        senderIdentity: last.senderIdentity,
        sessionRecordHex: last.sessionRecord.toString('hex'),
        plainLen: last.plaintext.length
    };
}

async function decryptMessage(msgFilePath) {
    await initWasm();
    if (!fs.existsSync(SESSION_FILE)) {
        throw new Error('sync_v2_session.bin not found — chạy decrypt-meta trước');
    }
    const sessionRecord = fs.readFileSync(SESSION_FILE);
    const msgBytes = fs.readFileSync(msgFilePath);

    // Multi-chunk: decrypt + unpack từng chunk, nối protobuf bytes lại.
    const chunks = splitFrames(msgBytes);
    const parts = [];
    for (const chunk of chunks) {
        const res = zproto.i(chunk, sessionRecord);
        const plaintext = Buffer.from(res.plaintext);
        res.free();
        parts.push(unpack(plaintext));
    }
    const out = Buffer.concat(parts);
    fs.writeFileSync(MESSAGE_FILE, out);
    return {
        chunks: chunks.length,
        plainLen: out.length
    };
}

if (require.main === module) {
const cmd = process.argv[2];
if (cmd === 'gen-keys') {
    generateV2Keys().then(keys => {
        console.log(JSON.stringify(keys));
        process.exit(0);
    }).catch(e => {
        console.error(e && e.message ? e.message : e);
        process.exit(1);
    });
} else if (cmd === 'decrypt-meta') {
    const metaPath = process.argv[3];
    decryptMeta(metaPath).then(res => {
        console.log(JSON.stringify(res));
        process.exit(0);
    }).catch(e => {
        console.error(e && e.message ? e.message : e);
        process.exit(1);
    });
} else if (cmd === 'decrypt-message') {
    const msgPath = process.argv[3];
    decryptMessage(msgPath).then(res => {
        console.log(JSON.stringify(res));
        process.exit(0);
    }).catch(e => {
        console.error(e && e.message ? e.message : e);
        process.exit(1);
    });
} else {
    console.error('Usage: node sync_v2_helper.js <gen-keys|decrypt-meta|decrypt-message> [file]');
    process.exit(2);
}
} // require.main guard

module.exports = { generateV2Keys, decryptMeta, decryptMessage, initWasm, splitFrames, unpack, findWasm, zproto };
