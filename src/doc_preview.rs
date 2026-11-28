//! In-app document preview: extract readable text from office/PDF attachments.
//!
//! Session-only (no disk cache): the worker downloads bytes (bounded 10 MiB),
//! [`extract_text`] pulls plain/structured text, and the GUI keeps the result in RAM.

use std::io::{Cursor, Read};

/// Max preview text kept in memory / shown in the full viewer.
pub const MAX_PREVIEW_CHARS: usize = 20_000;
/// Max download size for a preview fetch (10 MiB).
pub const MAX_PREVIEW_BYTES: usize = 10 * 1024 * 1024;
/// Max spreadsheet rows extracted per sheet.
const MAX_SHEET_ROWS: usize = 100;
/// Max spreadsheet columns extracted per row.
pub const MAX_SHEET_COLS: usize = 30;
/// Header marker wrapping each worksheet section inside extracted spreadsheet text.
const SHEET_HEADER_PREFIX: &str = "=== [Sheet: ";
const SHEET_HEADER_SUFFIX: &str = "] ===";

/// Parsed worksheet data rendered by the spreadsheet grid viewer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SheetPreview {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}

/// Whether `ext` (without leading dot, case-insensitive) should render as a table grid.
pub fn is_spreadsheet_ext(ext: &str) -> bool {
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "xls" | "xlsx" | "xlsm" | "xlsb" | "ods" | "csv" | "tsv"
    )
}

/// Extract readable text from raw file bytes by extension (no leading dot).
///
/// Supported: `pdf`, `doc`, `docx`, `xls`, `xlsx`, `xlsm`, `xlsb`, `ods`,
/// `ppt`, `pptx`, `txt`, `csv`, `tsv`, `md`, `log`, `json`, `xml`, `yaml`,
/// `yml`, `ini`, `toml`, `rtf`.
pub fn extract_text(bytes: &[u8], ext: &str) -> Result<String, String> {
    let ext = ext.to_ascii_lowercase();
    let raw = match ext.as_str() {
        "pdf" => extract_pdf(bytes)?,
        "docx" => extract_docx(bytes)?,
        "doc" => extract_doc_ole(bytes)?,
        "xls" | "xlsx" | "xlsm" | "xlsb" | "ods" => extract_spreadsheet(bytes, &ext)?,
        "pptx" => extract_pptx(bytes)?,
        "ppt" => extract_ppt_ole(bytes)?,
        "txt" | "csv" | "tsv" | "md" | "log" | "json" | "xml" | "yaml" | "yml" | "ini" | "toml" => {
            decode_lossy(bytes)
        }
        "rtf" => strip_rtf(&decode_lossy(bytes)),
        _ => return Err(format!("Không xem trước được .{ext}, hãy mở ngoài.")),
    };
    Ok(truncate_chars(&raw, MAX_PREVIEW_CHARS))
}

/// First `n` lines of `text` for the inline bubble snippet.
pub fn snippet_lines(text: &str, n: usize) -> String {
    text.lines().take(n).collect::<Vec<_>>().join("\n")
}

/// Parse extracted workbook / CSV / TSV preview text into per-sheet rows.
pub fn parse_sheets_from_text(text: &str, ext: &str) -> Vec<SheetPreview> {
    let ext = ext.to_ascii_lowercase();
    if ext == "csv" || ext == "tsv" {
        let delim = if ext == "tsv" { '\t' } else { ',' };
        let rows: Vec<Vec<String>> = text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .take(MAX_SHEET_ROWS)
            .map(|l| split_delimited_line(l, delim))
            .collect();
        return vec![SheetPreview {
            name: ext.to_ascii_uppercase(),
            rows,
        }];
    }

    let mut sheets: Vec<SheetPreview> = Vec::new();
    let mut current: Option<SheetPreview> = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix(SHEET_HEADER_PREFIX) {
            if let Some(name) = rest.strip_suffix(SHEET_HEADER_SUFFIX) {
                if let Some(prev) = current.take() {
                    if !prev.rows.is_empty() {
                        sheets.push(prev);
                    }
                }
                current = Some(SheetPreview {
                    name: name.to_string(),
                    rows: Vec::new(),
                });
                continue;
            }
        }
        if line.trim().is_empty() {
            continue;
        }
        let target = current.get_or_insert_with(|| SheetPreview {
            name: "Sheet1".to_string(),
            rows: Vec::new(),
        });
        target
            .rows
            .push(line.split('\t').map(str::to_string).collect());
    }
    if let Some(last) = current {
        if !last.rows.is_empty() {
            sheets.push(last);
        }
    }
    sheets
}

fn split_delimited_line(line: &str, delim: char) -> Vec<String> {
    if delim == '\t' {
        return line.split('\t').map(str::to_string).collect();
    }
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    in_quotes = !in_quotes;
                }
            }
            c if c == delim && !in_quotes => {
                cells.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    cells.push(cur.trim().to_string());
    cells
}

fn decode_lossy(bytes: &[u8]) -> String {
    // Strip UTF-8 BOM if present, then lossy-decode.
    let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
    String::from_utf8_lossy(bytes).into_owned()
}

fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let head: String = s.chars().take(max).collect();
    format!("{head}…")
}

fn extract_pdf(bytes: &[u8]) -> Result<String, String> {
    pdf_extract::extract_text_from_mem(bytes).map_err(|e| format!("Không đọc được PDF: {e}"))
}

fn read_zip_entry(bytes: &[u8], name: &str) -> Result<String, String> {
    let cursor = Cursor::new(bytes);
    let mut archive =
        zip::ZipArchive::new(cursor).map_err(|e| format!("Không mở được file Office: {e}"))?;
    let mut file = archive
        .by_name(name)
        .map_err(|_| format!("Thiếu {name} trong file Office."))?;
    let mut text = String::new();
    file.read_to_string(&mut text)
        .map_err(|e| format!("Không đọc được {name}: {e}"))?;
    Ok(text)
}

fn local_name_eq(e: &quick_xml::events::BytesStart<'_>, tag: &str) -> bool {
    e.local_name().into_inner() == tag
}

fn end_name_eq(e: &quick_xml::events::BytesEnd<'_>, tag: &str) -> bool {
    e.local_name().into_inner() == tag
}

/// Decoded text content of a `BytesText` event (entity-unescaped).
fn text_content(e: &quick_xml::events::BytesText<'_>) -> String {
    quick_xml::escape::unescape(&e.xml10_content())
        .map(|s| s.into_owned())
        .unwrap_or_else(|_| e.xml10_content().into_owned())
}

/// Normalize blank lines (at most 1 consecutive empty line) and trim outer whitespace.
fn normalize_multiline(s: &str) -> String {
    let mut collapsed = String::with_capacity(s.len());
    let mut blank_run = 0;
    for line in s.split('\n') {
        let t = line.trim_end();
        if t.trim().is_empty() {
            blank_run += 1;
            if blank_run <= 1 && !collapsed.is_empty() {
                collapsed.push('\n');
            }
        } else {
            blank_run = 0;
            collapsed.push_str(t);
            collapsed.push('\n');
        }
    }
    collapsed.trim().to_string()
}

/// Extract structured paragraphs, breaks, tabs, and table rows from Word/slide XML.
fn collect_structured_xml_text(xml: &str) -> String {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    let mut out = String::new();
    let mut in_t = false;
    let mut in_tc = false;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if local_name_eq(&e, "t") {
                    in_t = true;
                } else if local_name_eq(&e, "tc") {
                    in_tc = true;
                } else if local_name_eq(&e, "tab") {
                    out.push('\t');
                } else if local_name_eq(&e, "br") || local_name_eq(&e, "cr") {
                    out.push('\n');
                }
            }
            Ok(Event::Empty(e)) => {
                if local_name_eq(&e, "tab") {
                    out.push('\t');
                } else if local_name_eq(&e, "br") || local_name_eq(&e, "cr") {
                    out.push('\n');
                }
            }
            Ok(Event::Text(e)) => {
                if in_t {
                    out.push_str(&text_content(&e));
                }
            }
            Ok(Event::End(e)) => {
                if end_name_eq(&e, "t") {
                    in_t = false;
                } else if end_name_eq(&e, "tc") {
                    in_tc = false;
                    while out.ends_with(' ') {
                        out.pop();
                    }
                    if !out.ends_with('\t') && !out.ends_with('\n') {
                        out.push('\t');
                    }
                } else if end_name_eq(&e, "tr") {
                    while out.ends_with('\t') {
                        out.pop();
                    }
                    if !out.ends_with('\n') {
                        out.push('\n');
                    }
                } else if end_name_eq(&e, "p") {
                    if in_tc {
                        if !out.ends_with(' ') && !out.ends_with('\t') && !out.ends_with('\n') {
                            out.push(' ');
                        }
                    } else if !out.ends_with('\n') {
                        out.push('\n');
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    normalize_multiline(&out)
}

fn extract_docx(bytes: &[u8]) -> Result<String, String> {
    let xml = read_zip_entry(bytes, "word/document.xml")?;
    Ok(collect_structured_xml_text(&xml))
}

/// Parse `xl/sharedStrings.xml` into an ordered list of strings.
fn parse_shared_strings(xml: &str) -> Vec<String> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut out = Vec::new();
    let mut current = String::new();
    let mut in_si = false;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if local_name_eq(&e, "si") {
                    in_si = true;
                    current.clear();
                }
            }
            Ok(Event::Text(e)) => {
                if in_si {
                    current.push_str(&text_content(&e));
                }
            }
            Ok(Event::End(e)) => {
                if end_name_eq(&e, "si") {
                    in_si = false;
                    out.push(std::mem::take(&mut current));
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    out
}

/// Render one worksheet XML as tab-separated rows (capped at [`MAX_SHEET_ROWS`]).
fn render_sheet(xml: &str, shared: &[String]) -> Vec<String> {
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut rows: Vec<String> = Vec::new();
    let mut cells: Vec<String> = Vec::new();
    let flush_row = |cells: &mut Vec<String>, rows: &mut Vec<String>| {
        if !cells.iter().all(|c| c.trim().is_empty()) {
            rows.push(cells.join("\t"));
        }
        cells.clear();
    };
    let mut in_value = false;
    let mut cell_is_shared = false;
    let mut value = String::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                if local_name_eq(&e, "row") {
                    if rows.len() >= MAX_SHEET_ROWS {
                        break;
                    }
                    if !cells.is_empty() {
                        flush_row(&mut cells, &mut rows);
                    }
                } else if local_name_eq(&e, "c") {
                    cell_is_shared = e.attributes().filter_map(|a| a.ok()).any(|a| {
                        a.key.as_ref() == "t" && a.value.as_ref() == "s"
                    });
                    value.clear();
                } else if local_name_eq(&e, "v") || local_name_eq(&e, "t") {
                    in_value = true;
                }
            }
            Ok(Event::Text(e)) => {
                if in_value {
                    value.push_str(&text_content(&e));
                }
            }
            Ok(Event::End(e)) => {
                if end_name_eq(&e, "v") || end_name_eq(&e, "t") {
                    in_value = false;
                } else if end_name_eq(&e, "c") {
                    let text = if cell_is_shared {
                        value
                            .trim()
                            .parse::<usize>()
                            .ok()
                            .and_then(|i| shared.get(i))
                            .cloned()
                            .unwrap_or_default()
                    } else {
                        value.clone()
                    };
                    cells.push(text);
                    cell_is_shared = false;
                    value.clear();
                } else if end_name_eq(&e, "row") {
                    flush_row(&mut cells, &mut rows);
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    if !cells.is_empty() && rows.len() < MAX_SHEET_ROWS {
        flush_row(&mut cells, &mut rows);
    }
    rows
}

fn list_zip_names(bytes: &[u8]) -> Result<Vec<String>, String> {
    let cursor = std::io::Cursor::new(bytes);
    let mut archive =
        zip::ZipArchive::new(cursor).map_err(|e| format!("Không mở được file Office: {e}"))?;
    let mut names = Vec::new();
    for i in 0..archive.len() {
        if let Ok(f) = archive.by_index(i) {
            if let Ok(name) = f.name() {
                names.push(name.into_owned());
            }
        }
    }
    Ok(names)
}

fn format_calamine_cell(cell: &calamine::Data) -> String {
    use calamine::Data;
    match cell {
        Data::Empty => String::new(),
        Data::String(s) => s.replace(['\t', '\r', '\n'], " "),
        Data::Int(i) => i.to_string(),
        Data::Float(f) => {
            if f.is_finite() && f.fract() == 0.0 && f.abs() < 1e15 {
                format!("{f:.0}")
            } else {
                format!("{f}")
            }
        }
        Data::Bool(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
        Data::DateTime(dt) => {
            let (y, m, d, hh, mm, ss, _) = dt.to_ymd_hms_milli();
            if y == 1899 && m == 12 && d == 31 {
                format!("{hh:02}:{mm:02}:{ss:02}")
            } else if hh == 0 && mm == 0 && ss == 0 {
                format!("{y:04}-{m:02}-{d:02}")
            } else {
                format!("{y:04}-{m:02}-{d:02} {hh:02}:{mm:02}:{ss:02}")
            }
        }
        Data::DateTimeIso(s) | Data::DurationIso(s) => s.clone(),
        Data::Error(e) => format!("{e}"),
    }
}

fn render_calamine_workbook<'a, R>(wb: &mut R) -> Result<String, String>
where
    R: calamine::Reader<Cursor<&'a [u8]>>,
{
    let names = wb.sheet_names();
    if names.is_empty() {
        return Err("Không tìm thấy sheet nào trong file bảng tính.".to_string());
    }
    let mut sections = Vec::new();
    for name in names {
        let Ok(range) = wb.worksheet_range(&name) else {
            continue;
        };
        let mut sheet_rows = Vec::new();
        for row in range.rows() {
            if sheet_rows.len() >= MAX_SHEET_ROWS {
                break;
            }
            let mut cells: Vec<String> = row
                .iter()
                .take(MAX_SHEET_COLS)
                .map(format_calamine_cell)
                .collect();
            while cells.last().is_some_and(|c| c.trim().is_empty()) {
                cells.pop();
            }
            if !cells.is_empty() {
                sheet_rows.push(cells.join("\t"));
            }
        }
        if !sheet_rows.is_empty() {
            sections.push(format!(
                "{SHEET_HEADER_PREFIX}{name}{SHEET_HEADER_SUFFIX}\n{}",
                sheet_rows.join("\n")
            ));
        }
    }
    if sections.is_empty() {
        return Err("File bảng tính không có dữ liệu text.".to_string());
    }
    Ok(sections.join("\n\n"))
}

fn extract_spreadsheet(bytes: &[u8], ext: &str) -> Result<String, String> {
    use calamine::{Ods, Xls, Xlsb, Xlsx, open_workbook_auto_from_rs, open_workbook_from_rs};
    let res = match ext {
        "xls" => open_workbook_from_rs::<Xls<_>, _>(Cursor::new(bytes))
            .map_err(|e| e.to_string())
            .and_then(|mut w| render_calamine_workbook(&mut w)),
        "xlsb" => open_workbook_from_rs::<Xlsb<_>, _>(Cursor::new(bytes))
            .map_err(|e| e.to_string())
            .and_then(|mut w| render_calamine_workbook(&mut w)),
        "ods" => open_workbook_from_rs::<Ods<_>, _>(Cursor::new(bytes))
            .map_err(|e| e.to_string())
            .and_then(|mut w| render_calamine_workbook(&mut w)),
        _ => open_workbook_from_rs::<Xlsx<_>, _>(Cursor::new(bytes))
            .map_err(|e| e.to_string())
            .and_then(|mut w| render_calamine_workbook(&mut w)),
    };
    if res.is_ok() {
        return res;
    }
    if let Ok(mut auto) = open_workbook_auto_from_rs(Cursor::new(bytes)) {
        if let Ok(text) = render_calamine_workbook(&mut auto) {
            return Ok(text);
        }
    }
    if matches!(ext, "xlsx" | "xlsm") {
        if let Ok(text) = extract_xlsx_raw_xml(bytes) {
            return Ok(text);
        }
    }
    res.map_err(|e| format!("Không đọc được bảng tính (.{ext}): {e}"))
}

fn extract_xlsx_raw_xml(bytes: &[u8]) -> Result<String, String> {
    let names = list_zip_names(bytes)?;
    let shared = names
        .iter()
        .any(|n| n == "xl/sharedStrings.xml")
        .then(|| read_zip_entry(bytes, "xl/sharedStrings.xml").map(|x| parse_shared_strings(&x)))
        .transpose()?
        .unwrap_or_default();
    let mut sheets: Vec<String> = names
        .iter()
        .filter(|n| n.starts_with("xl/worksheets/sheet") && n.ends_with(".xml"))
        .cloned()
        .collect();
    sheets.sort_by_key(|s| natural_num_key(s));
    if sheets.is_empty() {
        return Err("Không tìm thấy sheet nào trong file Excel.".to_string());
    }
    let mut sections = Vec::new();
    for (idx, sheet_path) in sheets.iter().enumerate() {
        let xml = read_zip_entry(bytes, sheet_path)?;
        let rows = render_sheet(&xml, &shared);
        if rows.is_empty() {
            continue;
        }
        sections.push(format!(
            "{SHEET_HEADER_PREFIX}Sheet{}{SHEET_HEADER_SUFFIX}\n{}",
            idx + 1,
            rows.join("\n")
        ));
    }
    if sections.is_empty() {
        return Err("File Excel không có dữ liệu text.".to_string());
    }
    Ok(sections.join("\n\n"))
}

fn natural_num_key(path: &str) -> (u32, String) {
    let digits: String = path.chars().filter(|c| c.is_ascii_digit()).collect();
    let num = digits.parse::<u32>().unwrap_or(u32::MAX);
    (num, path.to_string())
}

fn extract_pptx(bytes: &[u8]) -> Result<String, String> {
    let names = list_zip_names(bytes)?;
    let mut slides: Vec<String> = names
        .iter()
        .filter(|n| n.starts_with("ppt/slides/slide") && n.ends_with(".xml"))
        .cloned()
        .collect();
    slides.sort_by_key(|s| natural_num_key(s));
    if slides.is_empty() {
        return Err("Không tìm thấy slide nào trong file PowerPoint.".to_string());
    }
    let mut out = Vec::new();
    for (idx, slide_name) in slides.iter().enumerate() {
        let xml = read_zip_entry(bytes, slide_name)?;
        let text = collect_structured_xml_text(&xml);
        if !text.trim().is_empty() {
            out.push(format!("--- Slide {} ---\n{}", idx + 1, text));
        }
    }
    if out.is_empty() {
        return Err("File PowerPoint không có dữ liệu text.".to_string());
    }
    Ok(out.join("\n\n"))
}

fn read_cfb_stream<R: std::io::Read + std::io::Seek>(
    comp: &mut cfb::CompoundFile<R>,
    name: &str,
) -> Option<Vec<u8>> {
    let mut stream = comp.open_stream(name).ok()?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).ok()?;
    Some(buf)
}

fn read_u16_le(b: &[u8], off: usize) -> Option<u16> {
    let s = b.get(off..off + 2)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}

fn read_i16_le(b: &[u8], off: usize) -> Option<i16> {
    let s = b.get(off..off + 2)?;
    Some(i16::from_le_bytes([s[0], s[1]]))
}

fn read_u32_le(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

fn read_i32_le(b: &[u8], off: usize) -> Option<i32> {
    let s = b.get(off..off + 4)?;
    Some(i32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

/// Decode single-byte Windows-1252 buffer into UTF-8.
fn decode_cp1252(bytes: &[u8]) -> String {
    const MAP_80_9F: [char; 32] = [
        '€', '\u{0081}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{008D}',
        'Ž', '\u{008F}', '\u{0090}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›',
        'œ', '\u{009D}', 'ž', 'Ÿ',
    ];
    let mut out = String::with_capacity(bytes.len());
    for &b in bytes {
        let ch = match b {
            0x80..=0x9F => MAP_80_9F[(b - 0x80) as usize],
            _ => char::from(b),
        };
        out.push(ch);
    }
    out
}

fn decode_utf16_le(bytes: &[u8]) -> String {
    let units = bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]));
    char::decode_utf16(units)
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect()
}

/// Strip Word binary field codes/control characters and normalize line/cell breaks.
fn clean_word_text(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut field_depth: usize = 0;
    let mut in_field_code = false;
    for ch in raw.chars() {
        match ch {
            '\u{0013}' => {
                field_depth += 1;
                in_field_code = true;
            }
            '\u{0014}' => {
                in_field_code = false;
            }
            '\u{0015}' => {
                field_depth = field_depth.saturating_sub(1);
                if field_depth == 0 {
                    in_field_code = false;
                }
            }
            _ if in_field_code => {}
            '\r' | '\u{000B}' | '\u{000C}' => out.push('\n'),
            '\u{0007}' => out.push('\t'),
            '\n' | '\t' => out.push(ch),
            c if c < ' ' => {}
            c => out.push(c),
        }
    }
    normalize_multiline(&out)
}

fn parse_doc_piece_table<R: std::io::Read + std::io::Seek>(
    comp: &mut cfb::CompoundFile<R>,
    word: &[u8],
) -> Option<String> {
    if word.len() < 0x44 {
        return None;
    }
    let flags = read_u16_le(word, 0x0A)?;
    let mut pos = 0x20usize;
    let csw = read_u16_le(word, pos)? as usize;
    pos = pos.checked_add(2 + csw * 2)?;
    let cslw = read_u16_le(word, pos)? as usize;
    let rg_lw_pos = pos + 2;
    let ccp_text = if cslw >= 4 {
        read_i32_le(word, rg_lw_pos + 12).map(|v| v.max(0) as u32).unwrap_or(0)
    } else {
        0
    };
    pos = rg_lw_pos.checked_add(cslw * 4)?;
    let cb_rg_fc_lcb = read_u16_le(word, pos)? as usize;
    if cb_rg_fc_lcb <= 33 {
        return None;
    }
    let fc_clx_pos = pos + 2 + 33 * 8;
    let fc_clx = read_u32_le(word, fc_clx_pos)? as usize;
    let lcb_clx = read_u32_le(word, fc_clx_pos + 4)? as usize;
    if lcb_clx == 0 {
        return None;
    }
    let table_name = if (flags & 0x0200) != 0 { "/1Table" } else { "/0Table" };
    let table = read_cfb_stream(comp, table_name)?;
    let clx = table.get(fc_clx..fc_clx.checked_add(lcb_clx)?)?;

    let mut i = 0usize;
    let pcd_slice = loop {
        let clxt = *clx.get(i)?;
        if clxt == 0x01 {
            let cb = read_i16_le(clx, i + 1)?;
            if cb < 0 {
                return None;
            }
            i = i.checked_add(3 + cb as usize)?;
        } else if clxt == 0x02 {
            let lcb_pcd = read_u32_le(clx, i + 1)? as usize;
            break clx.get(i + 5..i + 5 + lcb_pcd)?;
        } else {
            return None;
        }
    };
    if pcd_slice.len() < 16 {
        return None;
    }
    let n = (pcd_slice.len() - 4) / 12;
    let pcd_base = (n + 1) * 4;
    if pcd_base + n * 8 > pcd_slice.len() {
        return None;
    }
    let mut raw = String::new();
    for k in 0..n {
        let cp0 = read_u32_le(pcd_slice, k * 4)?;
        let cp1 = read_u32_le(pcd_slice, (k + 1) * 4)?;
        if ccp_text > 0 && cp0 >= ccp_text {
            break;
        }
        let mut char_count = cp1.saturating_sub(cp0);
        if ccp_text > 0 {
            char_count = char_count.min(ccp_text - cp0);
        }
        let fc_raw = read_u32_le(pcd_slice, pcd_base + k * 8 + 2)?;
        if (fc_raw & 0x4000_0000) != 0 {
            let byte_off = ((fc_raw & 0x3FFF_FFFF) / 2) as usize;
            let len = char_count as usize;
            if let Some(chunk) = word.get(byte_off..byte_off.saturating_add(len)) {
                raw.push_str(&decode_cp1252(chunk));
            }
        } else {
            let byte_off = (fc_raw & 0x3FFF_FFFF) as usize;
            let len = (char_count as usize).saturating_mul(2);
            if let Some(chunk) = word.get(byte_off..byte_off.saturating_add(len)) {
                raw.push_str(&decode_utf16_le(chunk));
            }
        }
    }
    let cleaned = clean_word_text(&raw);
    (!cleaned.is_empty()).then_some(cleaned)
}

/// Fallback printable-text scan inside an OLE binary stream (UTF-16LE + UTF-8/ASCII runs).
fn extract_ole_readable_text(data: &[u8]) -> String {
    let mut chunks = Vec::new();
    let mut cur_utf16 = Vec::new();
    let mut i = 0usize;
    while i + 1 < data.len() {
        let code = u16::from_le_bytes([data[i], data[i + 1]]);
        let ch_opt = char::from_u32(code as u32);
        let is_readable = ch_opt.is_some_and(|c| {
            c.is_alphanumeric()
                || matches!(c, ' ' | '\t' | '\n' | '\r' | '.' | ',' | ':' | ';' | '-' | '/' | '(' | ')' | '%' | '!' | '?')
        });
        if is_readable && code >= 0x20 || matches!(code, 0x0D | 0x0A | 0x09) {
            cur_utf16.push(code);
            i += 2;
        } else {
            if cur_utf16.len() >= 6 {
                let s = String::from_utf16_lossy(&cur_utf16).replace('\r', "\n");
                if s.chars().any(|c| c.is_alphabetic()) {
                    chunks.push(s.trim().to_string());
                }
            }
            cur_utf16.clear();
            i += 2;
        }
    }
    if cur_utf16.len() >= 6 {
        let s = String::from_utf16_lossy(&cur_utf16).replace('\r', "\n");
        if s.chars().any(|c| c.is_alphabetic()) {
            chunks.push(s.trim().to_string());
        }
    }
    if !chunks.is_empty() {
        return normalize_multiline(&chunks.join("\n"));
    }

    // ASCII / CP1252 printable fallback.
    let mut cur_ascii = Vec::new();
    for &b in data {
        if matches!(b, 0x20..=0x7E | b'\r' | b'\n' | b'\t') {
            cur_ascii.push(b);
        } else {
            if cur_ascii.len() >= 8 {
                let s = String::from_utf8_lossy(&cur_ascii).replace('\r', "\n");
                if s.chars().filter(|c| c.is_alphabetic()).count() >= 4 {
                    chunks.push(s.trim().to_string());
                }
            }
            cur_ascii.clear();
        }
    }
    if cur_ascii.len() >= 8 {
        let s = String::from_utf8_lossy(&cur_ascii).replace('\r', "\n");
        if s.chars().filter(|c| c.is_alphabetic()).count() >= 4 {
            chunks.push(s.trim().to_string());
        }
    }
    normalize_multiline(&chunks.join("\n"))
}

fn extract_doc_ole(bytes: &[u8]) -> Result<String, String> {
    let mut comp = cfb::CompoundFile::open(Cursor::new(bytes))
        .map_err(|e| format!("Không mở được file Word (.doc): {e}"))?;
    let word = read_cfb_stream(&mut comp, "/WordDocument")
        .ok_or_else(|| "Thiếu dữ liệu WordDocument trong file .doc.".to_string())?;
    if let Some(text) = parse_doc_piece_table(&mut comp, &word) {
        return Ok(text);
    }
    let fallback = extract_ole_readable_text(&word);
    if fallback.trim().is_empty() {
        return Err("File Word (.doc) không có dữ liệu text đọc được.".to_string());
    }
    Ok(fallback)
}

fn walk_ppt_records(
    slice: &[u8],
    in_slide: bool,
    current_slide: &mut Vec<String>,
    slides: &mut Vec<Vec<String>>,
    loose_atoms: &mut Vec<String>,
    depth: usize,
) {
    if depth > 32 {
        return;
    }
    let mut pos = 0usize;
    while pos + 8 <= slice.len() {
        let ver_inst = u16::from_le_bytes([slice[pos], slice[pos + 1]]);
        let rec_type = u16::from_le_bytes([slice[pos + 2], slice[pos + 3]]);
        let rec_len = u32::from_le_bytes([
            slice[pos + 4],
            slice[pos + 5],
            slice[pos + 6],
            slice[pos + 7],
        ]) as usize;
        pos += 8;
        let Some(end) = pos.checked_add(rec_len).filter(|&e| e <= slice.len()) else {
            break;
        };
        let payload = &slice[pos..end];
        let rec_ver = ver_inst & 0x0F;
        if rec_ver == 0x0F {
            // Skip MainMaster (0x03F0) boilerplate like "Click to edit Master title style".
            if rec_type == 0x03F0 {
                pos = end;
                continue;
            }
            if rec_type == 0x03EE {
                let mut slide_lines = Vec::new();
                walk_ppt_records(payload, true, &mut slide_lines, slides, loose_atoms, depth + 1);
                if !slide_lines.is_empty() {
                    slides.push(slide_lines);
                }
            } else {
                walk_ppt_records(payload, in_slide, current_slide, slides, loose_atoms, depth + 1);
            }
        } else if rec_type == 0x0FA0 {
            let text = normalize_multiline(&decode_utf16_le(payload).replace('\r', "\n"));
            if !text.is_empty() {
                if in_slide {
                    current_slide.push(text);
                } else {
                    loose_atoms.push(text);
                }
            }
        } else if rec_type == 0x0FA8 {
            let text = normalize_multiline(&decode_cp1252(payload).replace('\r', "\n"));
            if !text.is_empty() {
                if in_slide {
                    current_slide.push(text);
                } else {
                    loose_atoms.push(text);
                }
            }
        }
        pos = end;
    }
}

fn extract_ppt_ole(bytes: &[u8]) -> Result<String, String> {
    let mut comp = cfb::CompoundFile::open(Cursor::new(bytes))
        .map_err(|e| format!("Không mở được file PowerPoint (.ppt): {e}"))?;
    let ppt = read_cfb_stream(&mut comp, "/PowerPoint Document")
        .ok_or_else(|| "Thiếu dữ liệu PowerPoint Document trong file .ppt.".to_string())?;
    let mut slides: Vec<Vec<String>> = Vec::new();
    let mut scratch = Vec::new();
    let mut loose_atoms = Vec::new();
    walk_ppt_records(&ppt, false, &mut scratch, &mut slides, &mut loose_atoms, 0);
    if !slides.is_empty() {
        let rendered: Vec<String> = slides
            .into_iter()
            .enumerate()
            .map(|(i, lines)| format!("--- Slide {} ---\n{}", i + 1, lines.join("\n")))
            .collect();
        return Ok(rendered.join("\n\n"));
    }
    if !loose_atoms.is_empty() {
        return Ok(loose_atoms.join("\n\n"));
    }
    let fallback = extract_ole_readable_text(&ppt);
    if fallback.trim().is_empty() {
        return Err("File PowerPoint (.ppt) không có dữ liệu text đọc được.".to_string());
    }
    Ok(fallback)
}

/// Best-effort RTF strip: drop `\controlword`, keep grouped text.
fn strip_rtf(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' | '}' => {}
            '\\' => {
                // Control word: letters, optional numeric param, optional trailing space.
                let mut word = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_alphabetic() {
                        word.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_digit() || nc == '-' {
                        chars.next();
                    } else {
                        break;
                    }
                }
                if matches!(word.as_str(), "par" | "line" | "tab") {
                    out.push(if word == "tab" { '\t' } else { '\n' });
                } else if word.is_empty() {
                    // Escaped literal char (e.g. \\ \{ \}).
                    if let Some(nc) = chars.next() {
                        if nc != ' ' {
                            out.push(nc);
                        }
                    }
                } else if chars.peek() == Some(&' ') {
                    chars.next();
                }
            }
            _ => out.push(c),
        }
    }
    // Collapse 3+ newlines left by stripped groups.
    let mut collapsed = String::with_capacity(out.len());
    let mut blank_run = 0;
    for line in out.split('\n') {
        let t = line.trim();
        if t.is_empty() {
            blank_run += 1;
            if blank_run <= 1 {
                collapsed.push('\n');
            }
        } else {
            blank_run = 0;
            collapsed.push_str(t);
            collapsed.push('\n');
        }
    }
    collapsed.trim().to_string()
}
