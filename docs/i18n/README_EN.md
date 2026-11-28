# ctdozlite

**[Tiếng Việt](../../README.md)** · **[English](README_EN.md)** · **[한국어](README_KO.md)** · **[日本語](README_JA.md)** · **[简体中文](README_ZH.md)**

A pure Rust Zalo Desktop client (`eframe`/`egui` + `ratatui`) communicating directly with Zalo Web/PC APIs and WebSocket services without Electron or Chromium.

- **Project Homepage (Website & Download):** [https://ctdozlite.ctdo.net/](https://ctdozlite.ctdo.net/)
- **Prebuilt Packages (Releases):** [https://github.com/ctdoteam1110/ctdozlite/releases](https://github.com/ctdoteam1110/ctdozlite/releases)
- **Feature Matrix & Guides:** [Features & Limits](../TINH_NANG.md) · [Known Issues](../DA_BIET.md) · [Troubleshooting](../KHAC_PHUC_SU_CO.md)

---

## ⚡ Highlights (`v0.1.4`)

- **Pure Rust Scale & Performance:** **109,540 lines of Rust** across **78 files** (`106,425` LoC in `src/` + `3,115` LoC in `tests/`), cold startup in `< 160ms`, idle Private RAM ~61–80 MB (~156 MB Working Set vs ~1.8–1.9 GB on official Electron client), 1 single native process.
- **Cross-Platform Desktop:** Native builds & automated CI packages for Windows 10/11 (`.exe` setup & portable, Close-to-Tray `#213`), macOS (Universal `.dmg` Apple Silicon `arm64` + Intel `x86_64`), and Linux (`.deb`/`.rpm`/`.AppImage`/`.tar.gz` + legacy `glibc 2.31`, XDG Data Dir, `ksni` System Tray `#166`, Wayland quit-on-close `#207`, single-instance `#182`).
- **Dual Mode UI + Media & Social:** Full-featured `eframe`/`egui` GUI (Dark/Light themes, 4-digit PIN App Lock `Ctrl+L`, 3-tier Storage Manager, Group Polls, Twemoji & Open Sans, `Ctrl+V` clipboard images, document preview for PDF/DOCX/XLSX/PPTX/CSV, quick `❤️` hover button & exact UTF-16 `@mention` `#174`, aspect-preserving media placeholders + `VANISH_TOMBSTONE_JSON` `#192`) plus lightweight `ratatui` TUI with built-in LLM Agent loop.
- **Dual Mobile History Sync & VoIP:** Embeds Wasmtime (`libzproto_wasm`) for iOS Sync V2 with automatic 5.5 s fallback to Android Sync V1 (`cmd 12000`), alongside a 100% Rust VoIP engine (ZRTP, RTP/RTCP, Opus, H.264).
- **Automation & Local Storage:** Local HTTP REST API & MCP JSON-RPC 2.0 server (`--api-port`, `--mcp`), Rhai per-thread scripts, 2-way Telegram ↔ Zalo bridge, account-isolated SQLite WAL (`accounts/<uin>_history.db`), and automated 8-rule invariant gates (`scripts/check_invariants.py`).

---

## 📥 Download & Installation

Prebuilt installers and portable binaries are available on [GitHub Releases](https://github.com/ctdoteam1110/ctdozlite/releases):

| OS | Package | Instructions |
| :--- | :--- | :--- |
| **Windows** | `.exe` Installer / Portable `.zip` | Run installer or extract and run `ctdozlite.exe` |
| **macOS** | Universal `.dmg` (Apple Silicon & Intel) | Open `.dmg`, drag `ctdozlite.app` to `/Applications` |
| **Linux** | `.deb` / `.rpm` / `.AppImage` / `.tar.gz` | Install via package manager or run directly with `chmod +x` |

### CLI Options

| Option | Description | Example |
| :--- | :--- | :--- |
| `--data-dir <PATH>` | Set custom directory for session, database, cache, and logs | `ctdozlite --data-dir ./my-data --gui` |
| `--print-data-dir` | Print resolved data directory and exit | `ctdozlite --print-data-dir` |
| `--gui` | Launch graphical user interface (default) | `ctdozlite --gui` |
| `--tui` | Launch terminal user interface | `ctdozlite --tui` |
| `--qr` | Request login QR code immediately | `ctdozlite --tui --qr` |
| `--logout` | Clear saved session and exit | `ctdozlite --logout` |
| `--version` / `-V` | Print version and exit without initializing storage | `ctdozlite --version` |

---

## 🛠️ Building from Source

### Prerequisites
- **Rust Toolchain:** Stable Rust (1.80+ recommended) via `rustup`.
- **System Dependencies:**
  - **Linux:** `libasound2-dev`, `libfontconfig1-dev`, `libx11-dev`, `libxcursor-dev`, `libxi-dev`, `libgl1-mesa-dev`, `ffmpeg`.
  - **Windows / macOS:** Standard C toolchain and `ffmpeg` (for media/video calls).

```bash
# Clone the repository
git clone https://github.com/ctdoteam1110/ctdozlite.git
cd ctdozlite

# Build optimized release binary
cargo build --release

# Run GUI
./target/release/ctdozlite --gui
```

---

## 🏆 Key Contributors

Core contributors with representative code commits in the repository:

| Contributor | GitHub | Key Role | Code Volume |
| :--- | :--- | :--- | :---: |
| **TIPC1110** | [@TIPC1110](https://github.com/TIPC1110) | Core architecture, VoIP call engine, egui framework, Contacts & Social Feed, App Lock, 3-tier Storage Manager, cross-platform & releases up to v0.1.4 | **> 75,600+ lines** |
| **Đỗ Trung Thành (thanhdo1110)** | [@thanhdo1110](https://github.com/thanhdo1110) | Modular GUI refactoring, Atom One Dark, MCP/REST API, Message Scheduler, GitHub API Auto-Updater, 25+ Issue fixes & co-releases up to v0.1.4 | **> 35,600+ lines** 🏆 |
| **Thuong Tin** | [@thuongtin](https://github.com/thuongtin) | 10 PRs: OS timezone, customizable data-dir, macOS helpers, session atomic, media cache, updater, message state, exact message identity (#61), quote navigation (#60), macOS notification permissions (#59) | **> 11,500+ lines** 🏆 |
| **ngxccc** | [@ngxccc](https://github.com/ngxccc) | 5 PRs: Engineering standards, session safety, domain resolution | **> 3,400+ lines** |
| **GrootTheDeveloper** | [@GrootTheDeveloper](https://github.com/GrootTheDeveloper) | 8 PRs: SQLite startup speedup, resumable history sync (#45), worker poll timer (#46), chat virtualization, CJK font lazy loading | **~950+ lines** |
| **minhngoc2512** | [@minhngoc2512](https://github.com/minhngoc2512) | Linux XDG & System Tray (#166, #168), Downloads save & ImageViewer (#167, #177), Twemoji & Polls (#189, #193), Single-Instance (#182), Media Sizing (#192), Quick Heart & UTF-16 `@mention` (#174) | **Community PRs** 🏆 |
| **Leoaxo098** | [@Leoaxo098](https://github.com/Leoaxo098) | Restricted group composer lock (#97), image scroll jump fix (#212), Windows Close-to-Tray & pre-login settings (#213) | **Community PRs** |
| **ziq3** | [@ziq3](https://github.com/ziq3) | Reaction replay guard (#154), Wayland quit-on-close (#207), Windows-only corner popup (#208), double-click deadlock fix (#209) | **Community PRs** |
| **hoanglong149, gr219, cuongdm75, Fin12n, TraiNguyenVan, ducseul, NamGTitm** | GitHub Community | Wasmtime Sync V2, TUI LLM Agent, Telegram bridge, multi-file & album grid, file cards redesign, Linux IME fix, multi-account switcher & sync diagnostics | **Community PRs** 🏆 |

> *Note: The complete hall of fame honoring all contributors (bug reports, feature requests, pull requests) is published at [Contributors Wall](https://ctdozlite.ctdo.net/contributors.html).*

---


---

## 🙏 Acknowledgements

`ctdozlite` expresses its deepest gratitude to **VNG Corporation** and the **Zalo** development team, who built an excellent messaging platform serving tens of millions of users in Vietnam.

### Purpose of this project

- This project exists for **one purpose only**: learning and systems-engineering research, specifically understanding how a modern messaging application operates at the network-protocol, encryption, and native desktop architecture layers.
- All source code is **written from scratch in Rust**. To understand packet framing, request signing and parameter encoding, the maintainers performed **static analysis** (reading and decompiling) of the **publicly distributed** Zalo installer and its bundled native libraries. Those findings were used **only as reference for an independent reimplementation** — no Zalo machine code, JS/WASM source, `.so`, `.dex` or APK file is copied, embedded or redistributed in this repository. The unpacked analysis directories (`xapk_extracted/`, `test_lab/`) live **outside** the repository and are never committed.
- **Third-party artwork notice:** the bundled sticker set under `src/assets/ami_fat_cat/` is the work of a **third party (the original artist)**. It is not a VNG/Zalo asset, we claim no rights over it, and it is included for UI testing only — it will be removed immediately upon request from the rights holder.
- We **do not own** and **do not claim ownership** of the `Zalo` trademark, logo, or any VNG intellectual property. The name `Zalo` is used descriptively only, to state **which service's protocol this software is compatible with**, so users understand the product's nature.
- The project is **entirely non-commercial**: 100% open source, no fees, no advertising, no data sales, no monetization of any kind.

### Data privacy

- All messages and user data are stored **locally on your machine** (`zalo_history.db`, `session.json`, `threads_cache.json`).
- The application **sends no data** to the maintainer's servers or any third party. The only network connection is to the official Zalo system, identical to a browser opening `chat.zalo.me`.

### Takedown policy

If VNG Corporation or an authorized Zalo representative has any request regarding this project (including but not limited to takedown, renaming, or content adjustment), please contact us via **GitHub Issues** or the maintainer's email. We commit to **responding within 72 hours** and **complying with any legitimate request**.

> **In short:** We are deeply grateful to VNG and the Zalo team for building a great product. This project is only a learning effort by a small group of developers who love Rust and want to understand modern messaging systems more deeply. If anything is incorrect, we are ready to listen and change.

## 📜 License

Distributed under the dual **[MIT](../../LICENSE-MIT) OR [Apache-2.0](../../LICENSE-APACHE)** license. Developed strictly for educational and system-level engineering research purposes.

