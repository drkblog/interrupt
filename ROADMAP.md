# Interrupt Roadmap

This document outlines the proposed feature enhancements for **Interrupt** to improve user experience, security, and utility.

## Feature Index

| Feature ID | Feature Name | State |
|---|---|---|
| FT-01 | Custom Screensaver Parameter Configuration | Proposed |
| FT-02 | Audio Notifications and Relaxing Chimes | Proposed |
| FT-03 | Local Break Analytics and Compliance Logging | Proposed |
| FT-04 | System Tray Integration and Minimize-to-Tray | Proposed |
| FT-05 | Math Lock Screen | Proposed |

---

## Proposed Feature Details

### FT-01: Custom Screensaver Parameter Configuration
* **Description**: Enable users to fine-tune visual properties of the screensavers directly from the Settings screen.
* **Benefits**:
  - Customize the breathing rhythm speeds of the Default aurora.
  - Change colors (e.g., green vs. blue Matrix digital rain, custom monochrome clock fonts).
  - Adjust particle counts or character densities for performance tuning.
* **Implementation Plan**: Extend the `ScreensaverComponent` trait to accept dynamic config values, and expose styling widgets under a new "Screensaver Settings" expander in the UI.

### FT-02: Audio Notifications and Relaxing Chimes
* **Description**: Integrate subtle audio feedback to alert users when a state change is imminent or complete.
* **Benefits**:
  - Plays a gentle warn sound when the screen warning banner appears so users aren't startled.
  - Plays a relaxing chime (like a singing bowl or bell) when the break cycle successfully finishes.
* **Implementation Plan**: Integrate a lightweight audio playback crate (such as `rodio` or `kira`) and bundle a few small, high-quality audio files (.wav or .mp3) within the application binary.

### FT-03: Local Break Analytics and Compliance Logging
* **Description**: Save anonymous historical usage patterns to show break compliance trends.
* **Benefits**:
  - Keep track of total screen hours, number of completed breaks, and times the user had to use the override password.
  - Show a small visual chart or progress indicator in the Play UI to gamify break completion.
* **Implementation Plan**: Append records to a simple local file (`%APPDATA%\interrupt\history.json`) and build a tab/view within the settings or main panel to visualize this data using `egui_plot`.

### FT-04: System Tray Integration and Minimize-to-Tray
* **Description**: Allow the application window to be closed or minimized to the Windows System Tray.
* **Benefits**:
  - Keeps the taskbar clean during active working hours.
  - Provides a quick right-click context menu (e.g., "Open Settings", "Lock Now", "Exit").
* **Implementation Plan**: Use a crate like `tray-icon` or `native-dialog` to register a Windows notification tray icon and handle background window minimization.

### FT-05: Math Lock Screen
* **Description**: Introduce a math problem challenge as a requirement/option to unlock or extend the pause screen, encouraging active cognitive breaks or child supervision. Difficulty level, minimum number of questions and percentage of wait time save by answering questions should be configurable.
* **Difficulty Levels**:
  - **Low**: Single/double-digit addition and subtraction (e.g., `17 + 8` or `34 - 15`) with totals between 0 and 40.
  - **Medium**: Three digit addition and subtraction, multiplication and division tables or two-step operations (e.g., `12 * 6` or `45 + 5 - 12`).
  - **High**: Multi-step operations with order of operations and division, or simple algebra (e.g., `(15 * 4) / 5 + 18` or solving `4x - 7 = 25`).
* **Implementation Plan**: Define a configuration setting for math lock screen difficulty levels, render a math challenge input block in the pause screen overlay, and only allow unblocking once the correct answer is entered (or the master password is typed).
