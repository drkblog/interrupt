# Changelog

All notable changes to the **Interrupt** project will be documented in this file.

The format is based on [Keep a Changelog](https.keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-08-02

### 🎉 Initial Release

#### Core Break Management
- **Automated Time Cycles**: Cycles between **Play Time** (unlocked computer access) and **Pause Time** (enforced fullscreen break overlay).
- **Pre-Lock Warning Banner**: Configurable floating warning banner (`warning_time_seconds`, default 30s) displaying a red countdown alert before locking.
- **Live UI Countdown**: Prominent live countdown timer (`MM:SS`) displayed in the main play mode interface.
- **Timer Suspension during Settings**: Pauses countdown timer accumulation while the settings window is active to ensure configuring options doesn't reduce play/break time.
- **Manual Actions**: Includes password-protected "🔄 Reset Timer" and "🔒 Lock Now" buttons in the main user interface.

#### Modular Screensavers
- **Component Architecture**: Built around a clean `ScreensaverComponent` trait separating visual rendering from input focus and unblock mechanics.
- **Default (Ambient Aurora)**: 60 FPS breathing aurora with floating ambient particles and guided relaxation breathing indicator (`3s Inhale` -> `2s Hold` -> `6s Exhale`).
- **Minimalist (Monochrome Dark)**: Clean monochrome dark theme with quiet typography and clock.
- **Matrix (Digital Green Rain)**: 60 FPS animated digital rain with glowing ASCII characters, digits, tech symbols, trailing streams, and a pulsing matrix console card.

#### Security & Enforcement
- **Strict Keyboard Interception**: Low-level Win32 keyboard hook (`WH_KEYBOARD_LL`) blocking task-switching shortcuts (`Alt+Tab`, `Win key`, `Alt+Esc`, `Ctrl+Esc`, `Alt+F4`) during break time.
- **Multi-Monitor Screen Lock**: Spans all connected monitors via `SM_XVIRTUALSCREEN` metrics with borderless topmost OS window (`HWND_TOPMOST`, `WS_POPUP`).
- **Focus Restoration**: Captures `GetForegroundWindow()` prior to lock and restores exact focus on unblock (`SetForegroundWindow`, `AttachThreadInput`, `BringWindowToTop`).
- **Unblock Panel Behavior**:
  - Hidden by default on lock without stealing keyboard focus.
  - 3-second initial grace period ignoring mouse jitter on lock transition.
  - Reveals and focuses password input immediately upon user interaction (mouse move/click, keypress).
  - Automatically hides after 20 seconds of inactivity.
- **Password Protection**:
  - User password protected settings and early unblock (hashed using SHA-256).
  - Emergency Master Password support (`mindfulness`).
  - Auto-locks settings window upon closing.
- **Persistence**: Settings saved automatically to `%APPDATA%\interrupt\settings.json`.
