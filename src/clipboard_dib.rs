//! Checked DIB decoding independent of borrowed Win32 clipboard handles.
use anyhow::{ensure, Context, Result};

const MAX_PIXELS: usize = 16 * 1024 * 1024;
/// Maximum bytes accepted from one borrowed clipboard allocation.
pub(crate) const MAX_RAW: usize = 80 * 1024 * 1024;
fn word(bytes: &[u8], offset: usize) -> Result<u32> {
    let b = bytes
        .get(offset..offset + 4)
        .context("Clipboard header bị cắt")?;
    Ok(u32::from_le_bytes(b.try_into()?))
}
fn channel(value: u32, mask: u32) -> u8 {
    if mask == 0 {
        return 255;
    }
    let shift = mask.trailing_zeros();
    let maximum = mask >> shift;
    ((((value & mask) >> shift) as u64 * 255 + maximum as u64 / 2) / maximum as u64) as u8
}
#[derive(Clone, Copy)]
struct Layout {
    width: usize,
    height: usize,
    signed_height: i32,
    masks: [u32; 4],
    offset: usize,
    bpp: usize,
    stride: usize,
    rgb: bool,
}

fn layout(bytes: &[u8]) -> Result<Layout> {
    ensure!(
        bytes.len() <= MAX_RAW && bytes.len() >= 40,
        "Clipboard bitmap quá lớn hoặc không hợp lệ"
    );
    let header = word(bytes, 0)? as usize;
    ensure!(
        matches!(header, 40 | 52 | 56 | 108 | 124) && header <= bytes.len(),
        "Không hỗ trợ header clipboard"
    );
    let width = word(bytes, 4)? as i32;
    let signed_height = word(bytes, 8)? as i32;
    ensure!(
        width > 0 && width <= 8192 && signed_height != 0 && signed_height.unsigned_abs() <= 8192,
        "Kích thước clipboard không hợp lệ"
    );
    let width = width as usize;
    let height = signed_height.unsigned_abs() as usize;
    ensure!(
        width.checked_mul(height).is_some_and(|n| n <= MAX_PIXELS),
        "Ảnh clipboard quá nhiều điểm ảnh"
    );
    let planes = u16::from_le_bytes(bytes[12..14].try_into()?);
    let bits = u16::from_le_bytes(bytes[14..16].try_into()?);
    let compression = word(bytes, 16)?;
    ensure!(
        planes == 1 && matches!(bits, 24 | 32) && matches!(compression, 0 | 3 | 6),
        "Không hỗ trợ định dạng clipboard"
    );
    ensure!(bits == 32 || compression == 0, "Không hỗ trợ bitmap nén");
    let masks = if compression == 0 {
        [
            0xff0000,
            0xff00,
            0xff,
            if bits == 32 { 0xff000000 } else { 0 },
        ]
    } else {
        [
            word(bytes, 40)?,
            word(bytes, 44)?,
            word(bytes, 48)?,
            if compression == 6 || header >= 56 {
                word(bytes, 52)?
            } else {
                0
            },
        ]
    };
    for (i, mask) in masks.iter().enumerate() {
        ensure!(
            (i == 3 || *mask != 0)
                && (*mask == 0
                    || (mask >> mask.trailing_zeros())
                        .wrapping_add(1)
                        .is_power_of_two()),
            "Mặt nạ màu clipboard không hợp lệ"
        );
        for other in masks.iter().take(i) {
            ensure!(mask & other == 0, "Mặt nạ màu clipboard chồng nhau");
        }
    }
    ensure!(
        compression != 6 || header == 40 || header >= 56,
        "Header clipboard thiếu alpha mask"
    );
    let offset = header
        + if header == 40 && compression != 0 {
            if compression == 6 {
                16
            } else {
                12
            }
        } else {
            0
        };
    let palette_bytes = (word(bytes, 32)? as usize)
        .checked_mul(4)
        .context("Bảng màu clipboard tràn")?;
    let offset = offset
        .checked_add(palette_bytes)
        .context("Vị trí pixel clipboard tràn")?;
    let bpp = bits as usize / 8;
    let stride = width
        .checked_mul(bpp)
        .and_then(|n| n.checked_add(3))
        .context("Clipboard stride tràn")?
        & !3;
    let end = stride
        .checked_mul(height)
        .and_then(|n| n.checked_add(offset))
        .context("Clipboard bitmap tràn")?;
    ensure!(end <= bytes.len(), "Clipboard bitmap bị cắt");
    Ok(Layout {
        width,
        height,
        signed_height,
        masks,
        offset,
        bpp,
        stride,
        rgb: compression == 0,
    })
}

/// Validate the bounded layout before copying a borrowed Win32 clipboard buffer.
/// # Errors
/// Rejects truncated, oversized or unsupported DIB headers, masks and pixel data.
pub(crate) fn validate_dib(bytes: &[u8]) -> Result<()> {
    layout(bytes).map(|_| ())
}

/// Decode a packed 24/32-bit DIB after the clipboard handle has been unlocked.
/// # Errors
/// Rejects invalid dimensions, compression, masks or missing pixel bytes before allocating pixels.
pub(crate) fn decode_dib(bytes: &[u8]) -> Result<image::RgbaImage> {
    let Layout {
        width,
        height,
        signed_height,
        masks,
        offset,
        bpp,
        stride,
        rgb,
    } = layout(bytes)?;
    let opaque_rgb = rgb
        && (bpp == 3
            || !(0..height)
                .any(|y| (0..width).any(|x| bytes[offset + y * stride + x * bpp + 3] != 0)));
    let mut image = image::RgbaImage::new(width as u32, height as u32);
    for y in 0..height {
        let source = if signed_height < 0 { y } else { height - 1 - y };
        for x in 0..width {
            let p = offset + source * stride + x * bpp;
            let value = if bpp == 4 {
                word(bytes, p)?
            } else {
                u32::from_le_bytes([bytes[p], bytes[p + 1], bytes[p + 2], 0])
            };
            image.put_pixel(
                x as u32,
                y as u32,
                image::Rgba([
                    channel(value, masks[0]),
                    channel(value, masks[1]),
                    channel(value, masks[2]),
                    if opaque_rgb {
                        255
                    } else {
                        channel(value, masks[3])
                    },
                ]),
            );
        }
    }
    Ok(image)
}
