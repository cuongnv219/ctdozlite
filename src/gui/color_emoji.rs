//! Colour emoji drawn over egui text.
//!
//! egui only draws single-colour font glyphs. After the frame is laid out, [`ColorEmoji::paint`]
//! finds emoji in every text shape, hides their monochrome glyphs and paints the matching
//! Twemoji image in the same place. The text itself is unchanged, so wrapping, selection and
//! copy keep working.

use eframe::egui;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};

use egui::epaint::{Galley, Shape, TextShape};
use egui::{Color32, LayerId, Pos2, Rect, TextureHandle, TextureId, TextureOptions};

/// Twemoji 72x72 PNGs, built by `scripts/build_twemoji_pack.py`.
const PACK: &[u8] = include_bytes!("../assets/emoji/twemoji72.pack");
const PACK_MAGIC: &[u8] = b"TWEMOJI1";
const VS16: char = '\u{fe0f}';
/// Image side relative to the font row height; about 1.15 em for the bundled fonts.
const IMAGE_SCALE: f32 = 0.92;

struct EmojiIndex {
    images: HashMap<&'static str, &'static [u8]>,
    /// First characters of all keys, to skip plain text quickly.
    starts: HashSet<char>,
    /// Longest key, in characters (U+FE0F excluded).
    max_chars: usize,
}

fn index() -> &'static EmojiIndex {
    static INDEX: OnceLock<EmojiIndex> = OnceLock::new();
    INDEX.get_or_init(|| {
        parse_pack(PACK).unwrap_or_else(|| EmojiIndex {
            images: HashMap::new(),
            starts: HashSet::new(),
            max_chars: 0,
        })
    })
}

fn parse_pack(pack: &'static [u8]) -> Option<EmojiIndex> {
    let rest = pack.strip_prefix(PACK_MAGIC)?;
    let read_u32 = |b: &[u8], at: usize| -> Option<u32> {
        Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
    };
    let count = read_u32(rest, 0)? as usize;
    let mut at = 4;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let len = *rest.get(at)? as usize;
        let key = std::str::from_utf8(rest.get(at + 1..at + 1 + len)?).ok()?;
        at += 1 + len;
        let offset = read_u32(rest, at)? as usize;
        let size = read_u32(rest, at + 4)? as usize;
        at += 8;
        entries.push((key, offset, size));
    }
    let data = rest.get(at..)?;
    let mut index = EmojiIndex {
        images: HashMap::with_capacity(count),
        starts: HashSet::new(),
        max_chars: 0,
    };
    for (key, offset, size) in entries {
        let png = data.get(offset..offset + size)?;
        index.starts.extend(key.chars().next());
        index.max_chars = index.max_chars.max(key.chars().count());
        index.images.insert(key, png);
    }
    Some(index)
}

/// An emoji sequence in a run of characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EmojiSpan {
    start: usize,
    len: usize,
    key: &'static str,
}

/// Characters that read as text unless U+FE0F asks for emoji: Latin-1 (©, ®) and
/// U+2000..U+25FF (punctuation, arrows, maths, geometric shapes, including egui's `◻`).
fn text_by_default(c: char) -> bool {
    (c as u32) < 0x2000 || (0x2000..=0x25FF).contains(&(c as u32))
}

/// Find emoji sequences in `chars`, longest match first. U+FE0F is ignored when matching and
/// consumed as part of the sequence it follows.
fn find_emoji(index: &EmojiIndex, chars: &[char]) -> Vec<EmojiSpan> {
    let mut spans = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if !index.starts.contains(&chars[i]) {
            i += 1;
            continue;
        }
        let mut key = String::new();
        let mut key_chars = 0;
        let mut best: Option<EmojiSpan> = None;
        for (j, &c) in chars.iter().enumerate().skip(i) {
            if c != VS16 {
                key_chars += 1;
                if key_chars > index.max_chars {
                    break;
                }
                key.push(c);
            }
            if let Some((&k, _)) = index.images.get_key_value(key.as_str()) {
                best = Some(EmojiSpan {
                    start: i,
                    len: j + 1 - i,
                    key: k,
                });
            }
        }
        match best {
            Some(span) if span.len == 1 && text_by_default(chars[i]) => i += 1,
            Some(span) => {
                spans.push(span);
                i += span.len;
            }
            None => i += 1,
        }
    }
    spans
}

/// One emoji to paint: where, and which glyphs of the galley to hide.
struct Placement {
    row: usize,
    glyphs: std::ops::Range<usize>,
    key: &'static str,
}

struct TextHit {
    layer: LayerId,
    shape: usize,
    clip: Rect,
    placements: Vec<Placement>,
}

fn galley_placements(index: &EmojiIndex, galley: &Galley) -> Vec<Placement> {
    let text = galley.job.text.as_str();
    if !text.chars().any(|c| index.starts.contains(&c)) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (row_idx, row) in galley.rows.iter().enumerate() {
        let chars: Vec<char> = row.glyphs.iter().map(|g| g.chr).collect();
        for span in find_emoji(index, &chars) {
            out.push(Placement {
                row: row_idx,
                glyphs: span.start..span.start + span.len,
                key: span.key,
            });
        }
    }
    out
}

/// Collapse the quads of `glyphs` so the monochrome glyphs draw nothing.
fn hide_glyphs(galley: &mut Galley, row: usize, glyphs: std::ops::Range<usize>) {
    let Some(row) = galley.rows.get_mut(row) else {
        return;
    };
    // Each visible glyph adds one quad (4 vertices) to the row mesh, in glyph order
    // (`epaint::text::text_layout::tessellate_glyphs`); invisible glyphs add none.
    let mut vertex = row.visuals.glyph_vertex_range.start;
    for (i, glyph) in row.glyphs.iter().enumerate() {
        if glyph.uv_rect.is_nothing() {
            continue;
        }
        if glyphs.contains(&i) {
            if let Some(quad) = row.visuals.mesh.vertices.get_mut(vertex..vertex + 4) {
                let anchor = quad[0].pos;
                for v in quad {
                    v.pos = anchor;
                }
            }
        }
        vertex += 4;
    }
}

fn image_rect(galley: &Galley, origin: Pos2, placement: &Placement) -> Option<Rect> {
    let row = galley.rows.get(placement.row)?;
    let glyphs = row.glyphs.get(placement.glyphs.clone())?;
    let first = glyphs.first()?;
    let span = glyphs
        .iter()
        .map(|g| g.logical_rect())
        .reduce(|a, b| a.union(b))?;
    let side = first.font_height * IMAGE_SCALE;
    Some(Rect::from_center_size(span.center(), egui::vec2(side, side)).translate(origin.to_vec2()))
}

#[derive(Default)]
pub(super) struct ColorEmoji {
    /// `None` marks an image that failed to decode, so it is not retried every frame.
    textures: HashMap<&'static str, Option<TextureHandle>>,
}

impl ColorEmoji {
    /// Replace monochrome emoji glyphs painted so far this frame with colour images.
    /// Call once, after all UI for the frame has been added.
    pub(super) fn paint(&mut self, ctx: &egui::Context) {
        let index = index();
        if index.images.is_empty() {
            return;
        }
        let mut layers: Vec<LayerId> = ctx.memory(|m| m.layer_ids().collect());
        if !layers.contains(&LayerId::background()) {
            layers.push(LayerId::background());
        }

        let hits = ctx.graphics_mut(|g| {
            let mut hits = Vec::new();
            for &layer in &layers {
                let Some(list) = g.get(layer) else { continue };
                for (shape, clipped) in list.all_entries().enumerate() {
                    let Shape::Text(text) = &clipped.shape else {
                        continue;
                    };
                    if text.angle != 0.0 {
                        continue;
                    }
                    let placements = galley_placements(index, &text.galley);
                    if !placements.is_empty() {
                        hits.push(TextHit {
                            layer,
                            shape,
                            clip: clipped.clip_rect,
                            placements,
                        });
                    }
                }
            }
            hits
        });
        if hits.is_empty() {
            return;
        }

        // Textures must be created outside `graphics_mut`, which holds the context lock.
        let mut ids: HashMap<&'static str, TextureId> = HashMap::new();
        for placement in hits.iter().flat_map(|h| &h.placements) {
            if let Some(id) = self.texture(ctx, index, placement.key) {
                ids.insert(placement.key, id);
            }
        }

        ctx.graphics_mut(|g| {
            for hit in hits {
                let Some(list) = g.get_mut(hit.layer) else {
                    continue;
                };
                let mut images = Vec::new();
                list.mutate_shape(egui::layers::ShapeIdx(hit.shape), |clipped| {
                    let Shape::Text(text) = &mut clipped.shape else {
                        return;
                    };
                    paint_hit(text, &hit.placements, &ids, &mut images);
                });
                list.extend(hit.clip, images);
            }
        });
    }

    fn texture(
        &mut self,
        ctx: &egui::Context,
        index: &EmojiIndex,
        key: &'static str,
    ) -> Option<TextureId> {
        self.textures
            .entry(key)
            .or_insert_with(|| {
                let png = index.images.get(key)?;
                let rgba = image::load_from_memory(png).ok()?.into_rgba8();
                let (w, h) = rgba.dimensions();
                let img = egui::ColorImage::from_rgba_unmultiplied(
                    [w as usize, h as usize],
                    rgba.as_raw(),
                );
                let options =
                    TextureOptions::LINEAR.with_mipmap_mode(Some(egui::TextureFilter::Linear));
                Some(ctx.load_texture(format!("twemoji_{key}"), img, options))
            })
            .as_ref()
            .map(TextureHandle::id)
    }
}

fn paint_hit(
    text: &mut TextShape,
    placements: &[Placement],
    ids: &HashMap<&'static str, TextureId>,
    images: &mut Vec<Shape>,
) {
    let tint = Color32::WHITE.gamma_multiply(text.opacity_factor);
    let mut galley: Galley = (*text.galley).clone();
    for placement in placements {
        let (Some(&id), Some(rect)) = (
            ids.get(placement.key),
            image_rect(&galley, text.pos, placement),
        ) else {
            continue;
        };
        hide_glyphs(&mut galley, placement.row, placement.glyphs.clone());
        images.push(Shape::image(
            id,
            rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            tint,
        ));
    }
    text.galley = Arc::new(galley);
}
