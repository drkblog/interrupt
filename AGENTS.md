# AGENTS.md - Developer & Agent Guide for Interrupt

`interrupt` is a native Windows desktop application written in Rust designed to enforce screen breaks for children or users needing structured pause times.

## Architecture Overview

The codebase follows a modular structure:

```
src/
├── main.rs         # Application entry point, state machine, eframe/egui UI rendering
├── config.rs       # Configuration model, password hashing (SHA-256), settings persistence
├── screensaver.rs  # Modular ScreensaverComponent trait and screensaver implementations
└── win32.rs        # Native Windows integration (Focus capture/restore, low-level keyboard hook)
```

### Core Components

1. **Config & Security (`src/config.rs`)**:
   - `AppSettings` handles `play_time_minutes`, `pause_time_minutes`, `warning_time_seconds`, `screensaver_style`, and `password_hash`.
   - Master Password: `"mindfulness"` (hardcoded constant for emergency unlock & admin access).
   - Saved at `%APPDATA%\interrupt\settings.json`.

2. **Modular Screensavers (`src/screensaver.rs`)**:
   - Implements `ScreensaverComponent` trait for visual screensaver rendering (`Default`, `Minimalist`, `Matrix`).
   - Outer shell in `render_pause_screen` handles input focus, password verification, and OS window level, making it trivial to add new screensavers while reusing all unblock mechanics.

3. **Windows API Integration (`src/win32.rs`)**:
   - **Focus Capture & Restoration**: Uses `GetForegroundWindow()` before locking, and restores focus using `SetForegroundWindow()`, `AttachThreadInput()`, and `BringWindowToTop()` when unblocked.
   - **Low-Level Keyboard Hook (`WH_KEYBOARD_LL`)**: Intercepts task-switching keys (`Alt+Tab`, `Alt+Esc`, `Ctrl+Esc`, `Win key`, `Alt+F4`) during the `Pause` state while allowing standard alphanumeric key input for typing the unlock password.
   - **Multi-Monitor Screen Rect**: Queries `GetSystemMetrics` for `SM_XVIRTUALSCREEN`, `SM_YVIRTUALSCREEN`, `SM_CXVIRTUALSCREEN`, and `SM_CYVIRTUALSCREEN` to span all monitors during pause overlay.

4. **Application State Machine (`src/main.rs`)**:
   - **`AppState::Play`**: Normal computer usage. Displays prominent live countdown timer (`MM:SS`), "🔒 Lock Now", and password-protected "🔄 Reset Timer" buttons.
   - **`AppState::Warning`**: Triggered `warning_time_seconds` (default 30s) before pause time. Displays floating red warning banner with live countdown.
   - **`AppState::Pause`**: Fullscreen topmost dark overlay covering all virtual screen space. Enforces keyboard hook, captures input focus, requires password to unlock early or auto-unlocks when pause timer expires.
   - **Timer Suspension**: While the settings window is open (`show_settings = true`), elapsed timer accumulation is suspended so editing settings does not count against play/break duration.

## Building and Testing

### Prerequisites
- Windows OS (x86_64)
- Rust 1.70+ toolchain (`cargo`)

### Commands
- **Check Compilation**: `cargo check`
- **Run Tests**: `cargo test`
- **Run Locally**: `cargo run`
- **Build Release Binary**: `cargo build --release`

## Critical Rules for Agents
- **Preserve Keyboard Hook Safety**: Always guarantee `disable_keyboard_hook()` is invoked whenever exiting `Pause` state or unblocking.
- **Preserve Focus Restoration**: Ensure `capture_foreground_window()` is called right before entering `Pause` state.
- **Master Password**: Never remove or change the `MASTER_PASSWORD = "mindfulness"` constant.
