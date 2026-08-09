# Known Issues - Interrupt

This document tracks active issues for **Interrupt**.

| Issue ID | Description | Status |
|---|---|---|
| **ISSUE-01** | Headless Graphics Context Panic (OpenGL initialization fails in headless environments) | Active (Environment Restriction) |
| **ISSUE-02** | Window Visibility Restoration on Unblocking (Winit asynchronously forces visibility when restoring styles) | Active |
| **ISSUE-03** | Application process doesn't exit even after the window is closed | Active |

---

## Active Issues

### ISSUE-01: Headless Graphics Context Panic
- **Description**: Running the application in headless or non-interactive terminal sessions (such as remote build agents or background CLI processes) causes the application to panic during OpenGL context creation.
- **Error Trace**:
  ```
  thread 'main' panicked at glow_integration.rs:
  called `Result::unwrap()` on an `Err` value: Error { raw_code: Some(50), raw_os_message: Some("The request is not supported. (os error 50)"), kind: Misc }
  ```
- **Workaround/Remedy**: This is an environmental restriction. The application must be launched from an interactive user session with standard desktop shell and GPU capabilities.

### ISSUE-02: Window Visibility Restoration on Unblocking
- **Description**: If the main window is hidden (minimized to tray) before locking, unblocking (entering password) still restores the window to full visibility instead of keeping it hidden in the tray.
- **Notes**: Attempted to solve it by tracking `WAS_VISIBLE_BEFORE_LOCK` and sending `egui::ViewportCommand::Visible(false)` on unblocking, but the window is still visible. Additional diagnostic logging has been added to trace the window state during unblocking.

### ISSUE-03: Application Process Doesn't Exit After Window Close
- **Description**: When closing the application window, the window disappears, but the background process does not terminate cleanly and remains active in memory.
- **Notes**: This could be caused by active low-level keyboard hook threads, background event handlers, or message loop subclassing blocks keeping the application thread active. Diagnostic logging has been added to trace window subclass destruction, keyboard hook cleanup, and loop exit sequences.
