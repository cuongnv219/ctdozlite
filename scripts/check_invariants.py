#!/usr/bin/env python3
"""Repository invariant checker (Inner Loop enforcement).

Every failure prints the exact rule violated and a concrete FIX ROOT CAUSE hint
so agents and developers repair the underlying invariant instead of patching
symptoms.
"""

from __future__ import annotations

import pathlib
import re
import subprocess
import sys
from dataclasses import dataclass

ROOT = pathlib.Path(__file__).resolve().parent.parent

GUI_FILES = [
    ROOT / "src/gui.rs",
    ROOT / "src/gui/attachments.rs",
    ROOT / "src/gui/messages.rs",
    ROOT / "src/gui/modals.rs",
    ROOT / "src/gui/theme.rs",
    ROOT / "src/gui/worker.rs",
    ROOT / "src/gui/workspace.rs",
]

UI_THREAD_FILES = [
    ROOT / "src/gui.rs",
    ROOT / "src/gui/attachments.rs",
    ROOT / "src/gui/messages.rs",
    ROOT / "src/gui/modals.rs",
    ROOT / "src/gui/theme.rs",
    ROOT / "src/gui/workspace.rs",
]

STRICT_NON_GUI_MODULES = [
    ROOT / "src/account_manager.rs",
    ROOT / "src/api_mcp.rs",
    ROOT / "src/chat.rs",
    ROOT / "src/crypto.rs",
    ROOT / "src/data_dir.rs",
    ROOT / "src/endpoints.rs",
    ROOT / "src/history_sync.rs",
    ROOT / "src/media_resources.rs",
    ROOT / "src/qr.rs",
    ROOT / "src/reactions.rs",
    ROOT / "src/scheduler.rs",
    ROOT / "src/session.rs",
    ROOT / "src/store.rs",
    ROOT / "src/zproto_host.rs",
]

# Legacy ratchet for modules that still hold historical GuiTx coupling.
# Counts may only decrease toward 0; new references fail CI immediately.
LEGACY_GUI_RATCHET: dict[str, int] = {
    "src/call.rs": 4,
    "src/media.rs": 10,
    "src/tui.rs": 2,
}

LITERAL_NUM_RE = re.compile(
    r"^[+-]?(?:\d+(?:_\d+)*(?:\.\d+(?:_\d+)*)?)(?:_?(?:f32|f64|usize|isize|u\d+|i\d+))?$"
)
WORKAROUND_COMMENT_RE = re.compile(
    r"//.*\b(?:HACK|WORKAROUND|TEMP\s*FIX|TEMPORARY\s*FIX|KLUDGE|XXX|FIXME|TODO)\b",
    re.IGNORECASE,
)

# Ratchets: these technical-debt ceilings may only shrink, never grow.
MAX_REGULAR_COMMENTS = 1430
MAX_GUI_MANUAL_TIMERS = 31


@dataclass
class Violation:
    rule: str
    location: str
    snippet: str
    fix_hint: str


def split_prod_lines(path: pathlib.Path) -> list[tuple[int, str]]:
    """Return 1-based production lines before the file's `#[cfg(test)]` block."""
    if not path.exists():
        return []
    lines = path.read_text(encoding="utf-8", errors="replace").splitlines()
    out: list[tuple[int, str]] = []
    for idx, line in enumerate(lines, start=1):
        if line.strip().startswith("#[cfg(test)]"):
            break
        out.append((idx, line))
    return out


def strip_strings_and_comments(line: str) -> str:
    """Mask string literals and line comments so code-pattern scans ignore prose."""
    out: list[str] = []
    i = 0
    n = len(line)
    in_str = False
    while i < n:
        ch = line[i]
        if not in_str and ch == "/" and i + 1 < n and line[i + 1] == "/":
            break
        if ch == '"' and (i == 0 or line[i - 1] != "\\"):
            if in_str:
                in_str = False
                out.append('"')
                i += 1
                continue
            # Ignore raw byte/string prefixes lightly; standard string toggle
            in_str = True
            out.append('"')
            i += 1
            continue
        if not in_str:
            out.append(ch)
        i += 1
    return "".join(out)


def check_r1_no_gui_unwrap() -> list[Violation]:
    violations: list[Violation] = []
    fix = (
        "FIX ROOT CAUSE: Replace `.unwrap()` / `.expect(...)` with `if let`, `match`, "
        "`let ... else`, or `unwrap_or_else` so UI/worker threads cannot panic on "
        "unexpected network payloads, cache misses, or window states."
    )
    for path in GUI_FILES:
        for lineno, raw in split_prod_lines(path):
            code = strip_strings_and_comments(raw)
            if ".unwrap()" in code or ".expect(" in code:
                violations.append(
                    Violation(
                        rule="R1:no-gui-unwrap",
                        location=f"{path.relative_to(ROOT)}:{lineno}",
                        snippet=raw.strip(),
                        fix_hint=fix,
                    )
                )
    return violations


def extract_clamp_calls(text: str) -> list[tuple[int, str, str]]:
    """Find `.clamp(arg1, arg2)` calls and return (line_number, arg1, arg2)."""
    results: list[tuple[int, str, str]] = []
    needle = ".clamp("
    start = 0
    while True:
        idx = text.find(needle, start)
        if idx == -1:
            break
        lineno = text.count("\n", 0, idx) + 1
        pos = idx + len(needle)
        depth = 1
        comma_pos = -1
        end_pos = -1
        while pos < len(text) and depth > 0:
            c = text[pos]
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    end_pos = pos
                    break
            elif c == "," and depth == 1 and comma_pos == -1:
                comma_pos = pos
            pos += 1
        if end_pos != -1 and comma_pos != -1:
            arg1 = " ".join(text[idx + len(needle) : comma_pos].split())
            arg2 = " ".join(text[comma_pos + 1 : end_pos].split())
            results.append((lineno, arg1, arg2))
            start = idx + len(needle)
        else:
            start = idx + len(needle)
    return results


def parse_literal(expr: str) -> float | None:
    m = LITERAL_NUM_RE.match(expr.strip())
    if not m:
        return None
    cleaned = re.sub(r"_?(?:f32|f64|usize|isize|u\d+|i\d+)$", "", expr.strip())
    cleaned = cleaned.replace("_", "")
    try:
        return float(cleaned)
    except ValueError:
        return None


def check_r2_safe_clamp() -> list[Violation]:
    violations: list[Violation] = []
    fix = (
        "FIX ROOT CAUSE: On minimized/narrow windows or malformed media metadata, dynamic "
        "bounds can invert (`max < min`) and panic `f32::clamp`. Guard with "
        "`.clamp(MIN.min(max), max)` or `.clamp(min, max.max(min))`."
    )
    for path in GUI_FILES:
        prod_text = "\n".join(raw for _, raw in split_prod_lines(path))
        for lineno, arg1, arg2 in extract_clamp_calls(prod_text):
            lit1 = parse_literal(arg1)
            lit2 = parse_literal(arg2)
            if lit1 is not None and lit2 is not None:
                if lit1 > lit2:
                    violations.append(
                        Violation(
                            rule="R2:safe-clamp",
                            location=f"{path.relative_to(ROOT)}:{lineno}",
                            snippet=f".clamp({arg1}, {arg2})",
                            fix_hint=fix,
                        )
                    )
                continue
            # Dynamic bound(s): require explicit `.min(` on min or `.max(` on max
            has_guard = (".max(" in arg2) or (".min(" in arg1)
            # Zero lower bound vs non-negative upper expressions is still safer if .max(0.0),
            # flag positive literal min without .min(...) against dynamic max without .max(...)
            if not has_guard and lit1 is not None and lit1 > 0.0 and lit2 is None:
                violations.append(
                    Violation(
                        rule="R2:safe-clamp",
                        location=f"{path.relative_to(ROOT)}:{lineno}",
                        snippet=f".clamp({arg1}, {arg2})",
                        fix_hint=fix,
                    )
                )
    return violations


def check_r3_architecture_boundaries() -> list[Violation]:
    violations: list[Violation] = []
    fix = (
        "FIX ROOT CAUSE: Domain/protocol/storage modules under `src/` must not import "
        "`crate::gui`. Emit domain events, return `Result`, or accept a trait/sender "
        "injected by `src/gui/worker.rs` instead."
    )
    for path in STRICT_NON_GUI_MODULES:
        for lineno, raw in split_prod_lines(path):
            code = strip_strings_and_comments(raw)
            if "crate::gui" in code:
                violations.append(
                    Violation(
                        rule="R3:arch-boundary",
                        location=f"{path.relative_to(ROOT)}:{lineno}",
                        snippet=raw.strip(),
                        fix_hint=fix,
                    )
                )
    for rel, max_allowed in LEGACY_GUI_RATCHET.items():
        path = ROOT / rel
        hits: list[tuple[int, str]] = []
        for lineno, raw in split_prod_lines(path):
            if "crate::gui" in strip_strings_and_comments(raw):
                hits.append((lineno, raw.strip()))
        if len(hits) > max_allowed:
            lineno, snippet = hits[-1]
            violations.append(
                Violation(
                    rule="R3:arch-boundary-ratchet",
                    location=f"{rel}:{lineno} (found {len(hits)} > limit {max_allowed})",
                    snippet=snippet,
                    fix_hint=fix,
                )
            )
    return violations


def check_r4_fragile_patterns_and_workaround_comments() -> list[Violation]:
    violations: list[Violation] = []
    sleep_fix = (
        "FIX ROOT CAUSE: Never call `thread::sleep` on the immediate-mode egui UI thread; "
        "it freezes IME input and scrolling. Move blocking waits into `src/gui/worker.rs` "
        "or use non-blocking frame state transitions."
    )
    for path in UI_THREAD_FILES:
        for lineno, raw in split_prod_lines(path):
            code = strip_strings_and_comments(raw)
            if "thread::sleep(" in code:
                violations.append(
                    Violation(
                        rule="R4:no-ui-thread-sleep",
                        location=f"{path.relative_to(ROOT)}:{lineno}",
                        snippet=raw.strip(),
                        fix_hint=sleep_fix,
                    )
                )

    comment_fix = (
        "FIX ROOT CAUSE: Remove HACK/WORKAROUND/TEMP FIX/FIXME justification comments by "
        "fixing the underlying state or API invariant and locking it with a unit/property test."
    )
    for path in sorted((ROOT / "src").rglob("*.rs")):
        for lineno, raw in split_prod_lines(path):
            if WORKAROUND_COMMENT_RE.search(raw):
                violations.append(
                    Violation(
                        rule="R4:no-workaround-comments",
                        location=f"{path.relative_to(ROOT)}:{lineno}",
                        snippet=raw.strip(),
                        fix_hint=comment_fix,
                    )
                )
    return violations


def check_r5_data_hygiene() -> list[Violation]:
    violations: list[Violation] = []
    tracked_fix = (
        "FIX ROOT CAUSE: Never commit runtime account databases, session tokens, or logs. "
        "Run `git rm --cached <file>` and ensure `.gitignore` covers `session.json`, "
        "`accounts/`, `*.db`, and `crash.log`."
    )
    try:
        res = subprocess.run(
            ["git", "ls-files"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=False,
        )
        if res.returncode == 0:
            forbidden_re = re.compile(
                r"(^|/)session\.json$|^accounts/|\.db$|\.db-(?:wal|shm)$|(^|/)crash\.log$"
            )
            for tracked in res.stdout.splitlines():
                if forbidden_re.search(tracked.strip()):
                    violations.append(
                        Violation(
                            rule="R5:no-tracked-secrets",
                            location=tracked.strip(),
                            snippet=tracked.strip(),
                            fix_hint=tracked_fix,
                        )
                    )
    except OSError:
        pass

    allowed_session_writers = {"src/session.rs", "src/account_manager.rs"}
    write_fix = (
        "FIX ROOT CAUSE: Route all `session.json` writes through `Session::save` / "
        "`account_manager` so multi-account isolation and atomic writes are never bypassed."
    )
    for path in sorted((ROOT / "src").rglob("*.rs")):
        rel = path.relative_to(ROOT).as_posix()
        if rel in allowed_session_writers:
            continue
        for lineno, raw in split_prod_lines(path):
            if "fs::write" in raw and "session.json" in raw:
                violations.append(
                    Violation(
                        rule="R5:no-direct-session-write",
                        location=f"{rel}:{lineno}",
                        snippet=raw.strip(),
                        fix_hint=write_fix,
                    )
                )
    return violations


def check_r6_comment_budget() -> list[Violation]:
    violations: list[Violation] = []
    regular_count = 0
    for path in sorted((ROOT / "src").rglob("*.rs")):
        for _, raw in split_prod_lines(path):
            s = raw.strip()
            if s.startswith("//") and not (s.startswith("///") or s.startswith("//!")):
                regular_count += 1
    if regular_count > MAX_REGULAR_COMMENTS:
        violations.append(
            Violation(
                rule="R6:comment-budget-ratchet",
                location=f"src/**/*.rs (count={regular_count} > limit={MAX_REGULAR_COMMENTS})",
                snippet=f"{regular_count} regular // comments in production code",
                fix_hint=(
                    f"FIX ROOT CAUSE: Agents must not add `//` comments to justify workarounds or "
                    f"restate obvious code. Express intent with clear types/helpers/tests and keep "
                    f"regular `//` comments <= {MAX_REGULAR_COMMENTS}."
                ),
            )
        )
    return violations


def check_r7_no_new_manual_timers() -> list[Violation]:
    violations: list[Violation] = []
    timer_count = 0
    last_hit = ""
    for path in GUI_FILES:
        for lineno, raw in split_prod_lines(path):
            code = strip_strings_and_comments(raw)
            if "Instant::now()" in code:
                timer_count += 1
                last_hit = f"{path.relative_to(ROOT)}:{lineno}"
    if timer_count > MAX_GUI_MANUAL_TIMERS:
        violations.append(
            Violation(
                rule="R7:no-new-manual-timers",
                location=f"{last_hit} (count={timer_count} > limit={MAX_GUI_MANUAL_TIMERS})",
                snippet=f"{timer_count} Instant::now() calls in src/gui/",
                fix_hint=(
                    f"FIX ROOT CAUSE: Do not add ad-hoc `Instant::now()` manual timers in GUI code; "
                    f"they introduce subtle polling/debounce bugs. Model transitions explicitly or drive "
                    f"timed events from `src/gui/worker.rs` (limit <= {MAX_GUI_MANUAL_TIMERS})."
                ),
            )
        )
    return violations


def check_r8_no_windows_powershell_gui_notify() -> list[Violation]:
    violations: list[Violation] = []
    fix = (
        "FIX ROOT CAUSE: On Windows, chat/reaction/background alerts must use the custom "
        "ctdozlite corner popup (`IncomingPopupState`) and `crate::media::play_notification_chime()`, "
        "never PowerShell `os_notify`. Gate any `os_notify::notify*`/`play_notification_sound` in "
        "GUI files behind `#[cfg(not(target_os = \"windows\"))]`."
    )
    notify_re = re.compile(r"os_notify::(?:notify|notify_chat_batch|play_notification_sound)\s*\(")
    for path in GUI_FILES:
        lines = split_prod_lines(path)
        for idx, (lineno, raw) in enumerate(lines):
            code = strip_strings_and_comments(raw)
            if not notify_re.search(code):
                continue
            window_prev = "\n".join(prev_raw for _, prev_raw in lines[max(0, idx - 6) : idx])
            if 'not(target_os = "windows")' not in window_prev:
                violations.append(
                    Violation(
                        rule="R8:no-windows-powershell-notify",
                        location=f"{path.relative_to(ROOT)}:{lineno}",
                        snippet=raw.strip(),
                        fix_hint=fix,
                    )
                )
    return violations


def main() -> int:
    checks = [
        ("R1 Panic-free GUI (.unwrap/.expect)", check_r1_no_gui_unwrap),
        ("R2 Safe dynamic .clamp(min, max)", check_r2_safe_clamp),
        ("R3 Layer boundaries (non-GUI -> GUI)", check_r3_architecture_boundaries),
        (
            "R4 No UI sleep & no HACK/WORKAROUND comments",
            check_r4_fragile_patterns_and_workaround_comments,
        ),
        ("R5 Session/data & git tracking hygiene", check_r5_data_hygiene),
        (f"R6 Regular comment ratchet (<= {MAX_REGULAR_COMMENTS})", check_r6_comment_budget),
        (f"R7 Manual GUI timer ratchet (<= {MAX_GUI_MANUAL_TIMERS})", check_r7_no_new_manual_timers),
        ("R8 Custom popup on Windows (no PowerShell notify)", check_r8_no_windows_powershell_gui_notify),
    ]

    all_violations: list[Violation] = []
    for label, fn in checks:
        found = fn()
        status = "PASS" if not found else f"FAIL ({len(found)})"
        print(f"[{status}] {label}")
        all_violations.extend(found)

    if not all_violations:
        print("\nAll repository invariants satisfied (0 violations).")
        return 0

    print(f"\nFound {len(all_violations)} invariant violation(s):\n")
    for v in all_violations:
        print(f"- [{v.rule}] {v.location}")
        print(f"    code: {v.snippet}")
        print(f"    {v.fix_hint}\n")
    return 1


if __name__ == "__main__":
    sys.exit(main())
