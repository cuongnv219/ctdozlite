## Tóm tắt & Nguyên nhân gốc (Root Cause)
<!-- Mô tả ngắn gọn lỗi/tính năng và nguyên nhân gốc. KHÔNG dùng comment HACK/WORKAROUND để biện minh trong code. -->

## Kiểm tra bằng mã đã thêm (Code-Enforced Invariant)
- [ ] Đã thêm/cập nhật rule trong `scripts/check_invariants.py`, type/clippy lint, hoặc unit/property test để agent/dev không lặp lại lỗi này
- [ ] Thông báo lỗi khi vi phạm chỉ rõ cách sửa nguyên nhân gốc

## Bằng chứng kiểm chứng (Evidence)
<!-- Bắt buộc đính kèm output test/lint thực tế vừa chạy. Nếu sửa UI/UX, BẮT BUỘC đính kèm ảnh chụp màn hình hoặc video quay lại app chạy thật theo docs/FEATURE_CHECKLIST.md. -->

```text
python scripts/check_invariants.py
cargo test --lib -j 1 property_invariants
```

- Screenshot / Video (nếu chạm vào `src/gui*`):
  - Before / After: *(kéo thả ảnh hoặc clip vào đây)*

## Checklist tuân thủ
- [ ] Không chứa `.unwrap()`/`.expect()` mới trong `src/gui/`
- [ ] Không chứa `.clamp(min, max)` động chưa guard `min <= max`
- [ ] Không import chéo `crate::gui` từ tầng protocol/storage (`chat`, `store`, `qr`, `session`...)
- [ ] Không thêm comment `//` biện minh vá tạm hoặc `Instant::now()` thủ công mới trong GUI
- [ ] Không commit `session.json`, `accounts/`, `*.db`, `crash.log`
