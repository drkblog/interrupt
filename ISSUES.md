# Known Issues - Interrupt

This document tracks active and resolved issues for **Interrupt**.

| Issue ID | Description | Status |
|---|---|---|
| **ISSUE-01** | Headless Graphics Context Panic (OpenGL initialization fails in headless environments) | Active (Environment Restriction) |
| **ISSUE-02** | Window Visibility Restoration on Unblocking (Winit asynchronously forces visibility when restoring styles) | Resolved |

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

---

## Resolved Issues

### ISSUE-02: Window Visibility Restoration on Unblocking
- **Description**: If the main window is hidden (minimized to tray) before locking, unblocking (entering password) would still restore the window to full visibility instead of keeping it hidden in the tray.
- **Root Cause**: `eframe`/`winit` asynchronously queues viewport commands (like `Fullscreen(false)` and `InnerSize`). These commands run after native subclass window hiding routines, causing winit to force the window visible again at the end of the frame lifecycle.
- **Resolution**: Intercepted the visibility state and explicitly queued `egui::ViewportCommand::Visible(false)` when transitioning to play state, aligning both native Win32 window flags and the winit viewport loop.
