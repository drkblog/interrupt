# Interrupt 🌿

**Interrupt** is a lightweight, effective Windows desktop application written in Rust designed to help children and adults manage screen time with enforced, healthy breaks.

It periodically cycles between **Play Time** (unlocked computer access) and **Pause Time** (locked screensaver break mode).

---

## ✨ Features

- **Automated Time Cycles**: Set custom **Play Time** (e.g. 30 minutes) and **Pause Time** (e.g. 5 minutes).
- **Timer Suspension in Settings**: The play/pause timer is automatically suspended whenever the settings window is open.
- **Selectable Screensavers**: Choose between multiple screensaver styles:
  - **Default**: Ambient Aurora dark theme with floating particles and guided breathing indicator (`3s Inhale` -> `2s Hold` -> `6s Exhale`).
  - **Minimalist**: Sleek monochrome dark theme with quiet typography.
  - **Matrix**: Animated green Katakana digital rain theme.
- **Modular Component Architecture**: Extensible `ScreensaverComponent` trait that reuses unblock UI, keyboard hooks, and window focus management across all screensaver styles.
- **Configurable Warning Banner**: Displays a floating red banner before locking with a configurable countdown (default: 30 seconds).
- **Live Countdown Display**: Prominent live timer in the main UI showing exact time left before locking.
- **Lock Now & Reset Timer Buttons**: Manually trigger screen lock anytime, or reset the play timer with password authentication.
- **Strict Keyboard Enforcement**: Low-level Windows keyboard hook (`WH_KEYBOARD_LL`) blocks task switching (`Alt+Tab`, `Win key`, `Alt+Esc`, `Ctrl+Esc`, `Alt+F4`) during break time.
- **Focus Restoration**: Automatically restores focus to the exact application and window that was being used before the screen locked.
- **Password Protection**: Prevents unblocking or changing settings without the user password or master password.
- **Master Password**: Includes a master password (`mindfulness`) for emergency access or administration.
- **Settings Persistence**: Configurable settings saved in `%APPDATA%\interrupt\settings.json`.

---

## 🛠️ Usage & Configuration

### Configurable Settings
- **Play Time (`play_time_minutes`)**: Duration of computer play time in minutes (default: 30 mins).
- **Pause Time (`pause_time_minutes`)**: Duration of screen lock break in minutes (default: 5 mins).
- **Warning Time (`warning_time_seconds`)**: Pre-lock countdown banner duration in seconds (default: 30 secs).
- **Screensaver Style (`screensaver_style`)**: Visual theme selection (`Default`, `Minimalist`, `Matrix`).
- **User Password (`password_hash`)**: SHA-256 hashed password used to unblock screen early or edit settings (default: `1234`).

### Unblocking the Screen
When the pause screen is active, the computer will remain locked until the pause timer runs out or the correct password (user password or master password `mindfulness`) is entered into the unblock field.

---

## 💻 Development & Architecture

### Application Architecture
- **State Machine (`src/main.rs`)**: Controls transition states (`Play` -> `Warning` -> `Pause`).
- **Configuration & Security (`src/config.rs`)**: Hashing with SHA-256 and JSON storage at `%APPDATA%\interrupt\settings.json`.
- **Modular Screensavers (`src/screensaver.rs`)**: Implements `ScreensaverComponent` trait for visual screensavers.
- **Native Windows API (`src/win32.rs`)**: Low-level keyboard hook (`WH_KEYBOARD_LL`), virtual screen metrics, and active foreground window capture/restoration.

### Fixed Timings & System Constants
- **Master Password**: `"mindfulness"` (Emergency unlock and admin access constant).
- **Initial Screen Lock Grace Period**: `3 seconds` (User interaction is ignored during the first 3s of screen lock to prevent mouse jitter from popping up the password box immediately).
- **Unblock Panel Inactivity Auto-Hide**: `20 seconds` (If no mouse/keyboard interaction occurs for 20s while unlocked panel is visible, it hides automatically).
- **Default Breathing Animation Timing**: `11 seconds` total cycle (`3.0s Inhale` -> `2.0s Hold` -> `6.0s Exhale`).
- **Window Level**: Uses `HWND_TOPMOST` and `WS_POPUP` spanning `SM_XVIRTUALSCREEN` / `SM_YVIRTUALSCREEN` bounds during `Pause` state.

---

## 🚀 Building from Source

### Prerequisites
- Windows OS (Windows 10/11)
- Rust 1.70 or newer (`cargo`)

```bash
# Clone repository
git clone https://github.com/drkblog/interrupt.git
cd interrupt

# Run application
cargo run

# Build release executable
cargo build --release

# Create portable distribution ZIP package
.\package.bat
# or: powershell -ExecutionPolicy Bypass -File .\package.ps1
```

The compiled executable will be available at `target/release/interrupt.exe` and the portable ZIP package will be generated at `dist/interrupt-v1.1.0-windows-x64.zip`.

---

## 🛡️ Windows SmartScreen Warning Notice

When downloading and executing `interrupt.exe` on Windows for the first time, Windows Defender SmartScreen may display a warning banner stating *"Windows protected your PC - Microsoft Defender SmartScreen prevented an unrecognized app from starting"*.

### Why this happens
This warning appears for newly built or open-source executables that are not digitally signed with an expensive commercial **EV (Extended Validation) Code Signing Certificate**. It is standard Windows security behavior for unsigned binaries downloaded from GitHub.

### How to run the application
1. On the SmartScreen blue banner, click **"More info"**.
2. Click **"Run anyway"**.
3. *(Alternative)* Right-click `interrupt.exe` -> **Properties** -> Check **Unblock** at the bottom -> Click **Apply / OK**.

### For Developers: Code Signing
If you wish to digitally sign release binaries:
- Obtain a Code Signing Certificate (OV or EV) from a Certificate Authority (DigiCert, Sectigo, or free for open source via [SignPath Foundation](https://about.signpath.io/)).
- Sign the compiled executable using Microsoft `signtool.exe`:
  ```cmd
  signtool sign /fd SHA256 /a /tr http://timestamp.digicert.com /td SHA256 dist\interrupt-v1.1.0-windows-x64\interrupt.exe
  ```

---

## 📖 Technical & Agent Documentation

Refer to [AGENTS.md](file:///c:/Users/Leandro/repos/interrupt/AGENTS.md) for technical architecture details, Win32 API hooks, state machine implementation, and developer guidelines.

---

## 📄 License

MIT License. See `LICENSE` file for details.