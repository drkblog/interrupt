# AGENTS.md - Developer & Agent Guide for Interrupt

`interrupt` is a native Windows desktop application written in Rust designed to enforce screen breaks for children or users needing structured pause times.

## Architecture Overview

The codebase follows a modular structure:

```
src/
├── main.rs       # Application entry point, state machine, eframe/egui UI rendering
├── config.rs     # Configuration model, password hashing (SHA-256), settings persistence
└── win32.rs      # Native Windows integration (Focus capture/restore, low-level keyboard hook)
```

### Core Components

1. **Config & Security (`src/config.rs`)**:
   - `AppSettings` handles `play_time_minutes`, `pause_time_minutes`, and `password_hash`.
   - Master Password: `"mindfulness"` (hardcoded constant for emergency unlock & admin access).
   - Saved at `%APPDATA%\interrupt\settings.json`.

2. **Windows API Integration (`src/win32.rs`)**:
   - **Focus Capture & Restoration**: Uses `GetForegroundWindow()` before locking, and restores focus using `SetForegroundWindow()`, `AttachThreadInput()`, and `BringWindowToTop()` when unblocked.
   - **Low-Level Keyboard Hook (`WH_KEYBOARD_LL`)**: Intercepts task-switching keys (`Alt+Tab`, `Alt+Esc`, `Ctrl+Esc`, `Win key`, `Alt+F4`) during the `Pause` state while allowing standard alphanumeric key input for typing the unlock password.
   - **Multi-Monitor Screen Rect**: Queries `GetSystemMetrics` for `SM_XVIRTUALSCREEN`, `SM_YVIRTUALSCREEN`, `SM_CXVIRTUALSCREEN`, and `SM_CYVIRTUALSCREEN` to span all monitors during pause overlay.

3. **Application State Machine (`src/main.rs`)**:
   - **`AppState::Play`**: Normal computer usage. Tracks elapsed time toward pause duration.
   - **`AppState::Warning`**: Triggered 60 seconds before pause time. Displays a floating red warning banner with live countdown (`00:59`).
   - **`AppState::Pause`**: Fullscreen topmost dark overlay covering all virtual screen space. Enforces keyboard hook, captures input focus, requires password to unlock early or auto-unlocks when pause timer expires.

## Building and Testing

### Prerequisites
- Windows OS (x86_64)
- Rust 1.70+ toolchain (`cargo`)

### Commands
- **Check Compilation**: `cargo check`
- **Run Tests**: `cargo test`
- **Run Locally**: `cargo run`
- **Run with Debug Lock (20s Auto-Unlock)**: `cargo run --features debug`
- **Build Release Binary**: `cargo build --release`
- **Build Release with Debug Lock**: `cargo build --release --features debug`

## Critical Rules for Agents
- **Preserve Keyboard Hook Safety**: Always guarantee `disable_keyboard_hook()` is invoked whenever exiting `Pause` state or unblocking.
- **Preserve Focus Restoration**: Ensure `capture_foreground_window()` is called right before entering `Pause` state.
- **Master Password**: Never remove or change the `MASTER_PASSWORD = "mindfulness"` constant.
