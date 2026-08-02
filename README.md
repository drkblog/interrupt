# Interrupt 🌿

**Interrupt** is a lightweight, effective Windows desktop application written in Rust designed to help children and adults manage screen time with enforced, healthy breaks.

It periodically cycles between **Play Time** (unlocked computer access) and **Pause Time** (locked screensaver break mode).

---

## ✨ Features

- **Automated Time Cycles**: Set custom **Play Time** (e.g. 30 minutes) and **Pause Time** (e.g. 5 minutes).
- **1-Minute Warning Banner**: Displays a floating red top banner 60 seconds before locking with a live countdown so the user can finish what they are doing.
- **Ambient Dark Screensaver**: Fullscreen dark overlay spanning across all connected monitors when pause time begins.
- **Strict Keyboard Enforcement**: Low-level Windows keyboard hook (`WH_KEYBOARD_LL`) blocks task switching (`Alt+Tab`, `Win key`, `Alt+Esc`, `Ctrl+Esc`, `Alt+F4`) during break time.
- **Focus Restoration**: Automatically restores focus to the exact application and window that was being used before the screen locked.
- **Password Protection**: Prevents unblocking or changing settings without the user password or master password.
- **Master Password**: Includes a master password (`mindfulness`) for emergency access or administration.
- **Settings Persistence**: Configurable settings saved in `%APPDATA%\interrupt\settings.json`.

---

## 🛠️ Usage & Configuration

### Default Settings
- **Play Time**: 30 minutes
- **Pause Time**: 5 minutes
- **Default Password**: `1234`
- **Master Password**: `mindfulness`

### Unblocking the Screen
When the pause screen is active, the computer will remain locked until the pause timer runs out or the correct password (user password or master password `mindfulness`) is entered into the unblock field.

### Changing Settings
Click the **⚙️ Settings** button in the main window or settings menu, enter the password to authenticate, and adjust:
- Play duration (in minutes)
- Pause duration (in minutes)
- Custom user password

---

## 🚀 Building from Source

### Prerequisites
- Windows OS (Windows 10/11)
- Rust 1.70 or newer (`cargo`)

```bash
# Clone repository
git clone https://github.com/drkblog/interrupt.git
cd interrupt

# Run standard application
cargo run

# Run with debug mode (forces 20-second auto-unlock pause screen)
cargo run --features debug

# Build release executable
cargo build --release

# Build release executable with debug mode enabled
cargo build --release --features debug
```

The compiled executable will be available at `target/release/interrupt.exe`.

---

## 📖 Technical & Agent Documentation

Refer to [AGENTS.md](file:///c:/Users/Leandro/repos/interrupt/AGENTS.md) for technical architecture details, Win32 API hooks, state machine implementation, and developer guidelines.

---

## 📄 License

MIT License. See `LICENSE` file for details.