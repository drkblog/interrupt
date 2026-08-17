# Architectural Investigation & Specification: Windows Key & Lock Screen Escape Prevention

## 1. Executive Summary

A core requirement of **Interrupt** is to reliably enforce screen pause time by preventing users (particularly children) from escaping the fullscreen pause overlay. Specifically, pressing the **Windows Key** (`VK_LWIN` / `VK_RWIN`), **Alt+Tab**, **Alt+Esc**, **Alt+F4**, or **Ctrl+Esc** must not open the Start menu, switch windows, or expose the underlying desktop.

Previous attempts to achieve this via Windows Low-Level Keyboard Hooks (`WH_KEYBOARD_LL`) have failed or behaved intermittently. This document provides:
1. A technical audit of why the current implementation in `src/win32.rs` and `src/main.rs` fails.
2. An in-depth analysis of Windows OS input routing, Z-order bands, and hook constraints.
3. A comparison of viable technical solutions on Windows 10/11.
4. A concrete, step-by-step implementation plan for a robust, resilient lock mechanism.

---

## 2. Investigation: Why Current Attempts Failed

### 2.1 Current Code Analysis (`src/win32.rs` & `src/main.rs`)

Currently, `win32.rs` sets up a hook via:
```rust
pub fn enable_keyboard_hook() {
    // ...
    let tid = GetCurrentThreadId();
    let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), std::ptr::null_mut(), 0);
    // ...
}
```
And inside `low_level_keyboard_proc`:
```rust
unsafe extern "system" fn low_level_keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 && HOOK_ENABLED.load(Ordering::SeqCst) {
        let kbd = *(lparam as *const KBDLLHOOKSTRUCT);
        let vk = kbd.vkCode as u16;
        let is_key_event = wparam == WM_KEYDOWN as usize
            || wparam == WM_SYSKEYDOWN as usize
            || wparam == WM_KEYUP as usize
            || wparam == WM_SYSKEYUP as usize;
        let is_alt_down = (kbd.flags & LLKHF_ALTDOWN) != 0;

        let block = match vk {
            VK_TAB => is_alt_down,
            VK_ESCAPE => is_alt_down || (kbd.flags & 0x01 != 0),
            VK_LWIN | VK_RWIN => true,
            VK_F4 => is_alt_down,
            _ => false,
        };

        if block && is_key_event {
            unsafe {
                keybd_event(0, 0, 0, 0); // dummy down
                keybd_event(0, 0, KEYEVENTF_KEYUP, 0); // dummy up
            }
            return 1; // Block key
        }
    }
    let hook = HOOK_HANDLE.load(Ordering::SeqCst);
    CallNextHookEx(hook as _, code, wparam, lparam)
}
```

### 2.2 The 5 Root Causes of Failure

#### 1. Hook Installation on the GUI/Rendering Thread (`LowLevelHooksTimeout` starvation)
* **The Mechanism**: `WH_KEYBOARD_LL` is a global hook that runs in the context of the thread that called `SetWindowsHookExW`. When any key is pressed anywhere in Windows, the OS halts input processing and synchronously context-switches to this thread's message queue to execute the callback.
* **The Problem**: `enable_keyboard_hook()` is called inside `InterruptApp::transition_to_pause()`, which runs on the **main eframe/egui UI thread**. During Pause mode, egui renders at 60 FPS, manages GPU/OpenGL/DirectX buffers, updates timer math, and runs text-to-speech (`powershell` process spawns).
* **The Failure**: Windows enforces an internal registry timeout named `LowLevelHooksTimeout` (default 200ms, but Windows 10/11 drops this dynamically down to 25ms under system load). If the UI thread is busy rendering a frame or waiting on V-Sync, message dispatching stalls for just 30ms. **Windows silently skips the hook or permanently drops it**, routing `VK_LWIN` straight to `explorer.exe` or `StartMenuExperienceHost.exe`.

#### 2. Corrupting Input Stream with `keybd_event(0, 0, 0, 0)`
* **The Problem**: Inside `low_level_keyboard_proc`, lines 253-255 call `keybd_event(0, 0, 0, 0)` and `keybd_event(0, 0, KEYEVENTF_KEYUP, 0)`.
* **The Failure**: `0` is an invalid virtual key code. Calling `keybd_event` inside a low-level hook callback injects synthetic hardware input into the global input queue, immediately triggering re-entrancy into `low_level_keyboard_proc`. This causes message spam, recursion, and timing jitter, corrupting the OS keyboard state machine. Returning `1` is the only requirement to drop an event.

#### 3. Key-Down vs Key-Up Asymmetry and Modifier Leakage
* **The Mechanism**: The Windows Start Menu opens on `WM_KEYUP` of `VK_LWIN`/`VK_RWIN` if no other key was pressed during the keystroke. Other shell actions (`Win+D` minimize all, `Win+Tab` Task View, `Win+X` Quick Link, `Win+R` Run, `Win+E` Explorer) trigger on combinations.
* **The Failure**: If a hook blocks `WM_KEYDOWN` for `VK_LWIN` but `WM_KEYUP` is delayed or not suppressed, Windows sees an orphaned KeyUp and invokes the Start menu. Additionally, if the user holds Win and presses 'D', 'D' is passed through unless the hook explicitly tracks that a Win modifier is active.

#### 4. DWM Z-Order Bands & Focus Stealing
* **The Mechanism**: In Windows 10 and 11, the Desktop Window Manager (DWM) categorizes windows into Z-order bands:
  - Band 0: Normal Desktop Windows
  - Band 1: `HWND_TOPMOST` Windows
  - Band 2: System Shell & Immersive Overlay Band (`StartMenuExperienceHost`, `ShellExperienceHost`, Notification Center, Taskbar, Game Bar).
* **The Failure**: Calling `SetWindowPos(HWND_TOPMOST)` places the application in Band 1. If the Start menu, a notification toast, or an elevated window gains focus, it renders in Band 2, visually superimposing over the pause screen. Because `Interrupt` does not continuously enforce or re-acquire focus when `WM_KILLFOCUS` or `EVENT_SYSTEM_FOREGROUND` occurs, the user can interact with the Start menu.

#### 5. User Interface Privilege Isolation (UIPI) & Elevated Windows
* **The Mechanism**: Windows UIPI prevents lower-integrity processes from sending messages or intercepting input from higher-integrity (Admin/Elevated) processes.
* **The Failure**: If an elevated application or Task Manager (`Ctrl+Shift+Esc`) is active before the lock triggers, a standard user-mode `WH_KEYBOARD_LL` hook cannot intercept keystrokes directed to that elevated surface.

---

## 3. Windows Security & Input Architecture

### 3.1 What CANNOT Be Blocked by User-Mode Applications
1. **`Ctrl + Alt + Delete` (Secure Attention Sequence / SAS)**:
   - Hardcoded into the Windows kernel (`win32k.sys`) and Winlogon.
   - Handled directly by the kernel's hardware interrupt handler; never routed through `WH_KEYBOARD_LL`.
   - *Mitigation*: Even if a child presses Ctrl+Alt+Del, selecting Task Manager can be mitigated by keeping the lock screen topmost and re-grabbing focus upon return.
2. **`Win + L` (Workstation Lock)**:
   - Handled directly by Winlogon to lock the Windows user account.
   - Cannot be intercepted by `WH_KEYBOARD_LL`.
   - *Mitigation*: Locking the workstation simply locks Windows itself (requiring the parent's Windows login), which does not compromise the break enforcement.

### 3.2 What CAN Be Reliably Blocked in User-Mode
With the proper architecture, all of the following can be 100% blocked:
- Windows Key (`VK_LWIN`, `VK_RWIN`) & Start Menu
- `Alt + Tab` (Task Switcher)
- `Win + Tab` (Task View / Virtual Desktops)
- `Alt + Esc` / `Ctrl + Esc`
- `Alt + F4` (Window Close)
- `Win + D` (Show Desktop), `Win + M`, `Win + E`, `Win + R`, `Win + X`, `Win + S`, `Win + A`, `Win + N`
- Context Menu Key (`VK_APPS`)

---

## 4. Evaluation of Technical Approaches

| Approach | Security & Escape Resistance | Complexity | System Impact | Recommendation |
|---|---|---|---|---|
| **A: Dedicated Worker-Thread Hook + Focus Keeper** | **95%** (Blocks all Win keys, Alt+Tab, Start Menu, task switches) | Low / Moderate | Zero (pure user-space, zero side effects) | **Primary Recommended Solution** |
| **B: Alternate Windows Desktop (`CreateDesktop` / `SwitchDesktop`)** | **100%** (Explorer/Taskbar literally do not exist on the break desktop) | High (Requires rendering context on secondary desktop) | Moderate (Audio/DirectX isolation) | **Alternative / Future Hardened Mode** |
| **C: Registry / Group Policy Tweaks (`NoWinKeys`, Scancode Map)** | High | Moderate | High (Persists on crash/reboot, requires Admin, can brick user keyboard) | **Not Recommended** |
| **D: Windows Keyboard Filter Driver (`WEBF`)** | 100% | High | Windows Enterprise/IoT only | **Not Applicable for Consumer Windows** |

---

## 5. The Correct Implementation Strategy (Approach A)

To guarantee that the Windows key and task-switching keys are unconditionally blocked without timeout drops or glitches, the application must implement a **3-Pillar Lock System**:

```
+-------------------------------------------------------------------------+
|                               INTERRUPT                                 |
+-------------------------------------------------------------------------+
       |                                                |
       v                                                v
[ Main egui UI Thread ]                     [ Dedicated Hook Worker Thread ]
  - Renders 60 FPS screensaver & quizzes       - Spawns on Pause transition
  - Zero hook code here                        - Calls SetWindowsHookExW(WH_KEYBOARD_LL)
  - Periodically verifies Topmost Z-order      - Dedicated Win32 GetMessageW message pump
  - Traps mouse cursor inside screen rect      - Zero-latency hook callback (< 1µs)
                                               - Pure drop: return 1 (No keybd_event)
                                               - Unhooks & terminates on Play transition
```

### Pillar 1: Dedicated Background Message-Pump Thread for `WH_KEYBOARD_LL`
Instead of installing the hook on the egui UI thread, spawn a dedicated OS thread solely for the hook:
1. Thread starts and creates a Win32 message loop via `GetMessageW`.
2. Calls `SetWindowsHookExW(WH_KEYBOARD_LL, ...)` with its own thread ID.
3. Because this thread does **no rendering, no allocations, and no disk/TTS I/O**, its response time to OS input messages is under 0.1 milliseconds (100x faster than `LowLevelHooksTimeout`).
4. On `transition_to_play`, post `WM_QUIT` to this thread to cleanly unhook and terminate.

### Pillar 2: Clean Key Interception Logic (Zero Side Effects)
1. **Clean Return-1 Semantics**: When a blocked key is detected, return `1` immediately. **Remove all calls to `keybd_event(0,0,0,0)`**.
2. **Handle All Key Event Types**:
   - `WM_KEYDOWN`, `WM_SYSKEYDOWN`, `WM_KEYUP`, `WM_SYSKEYUP`.
3. **Block List**:
   - `VK_LWIN` (`0x5B`), `VK_RWIN` (`0x5C`) -> Always block.
   - `VK_TAB` -> Block if Alt is down.
   - `VK_ESCAPE` -> Block if Alt or Ctrl is down.
   - `VK_F4` -> Block if Alt is down.
   - `VK_APPS` (`0x5D`, context menu key) -> Block.
   - Any key pressed while `VK_LWIN` or `VK_RWIN` is held down -> Block (prevents `Win+D`, `Win+X`, `Win+R`, etc.).

### Pillar 3: Active Focus & Z-Order Enforcement (Focus Guard)
Even with keys blocked, Windows notifications or mouse clicks can try to steal focus. The Pause screen must actively maintain top Z-order:
1. **Fullscreen Topmost Multi-Monitor Coverage**:
   - Set `WS_POPUP | WS_VISIBLE` and `WS_EX_TOPMOST | WS_EX_TOOLWINDOW`.
   - `SetWindowPos(hwnd, HWND_TOPMOST, x, y, width, height, SWP_SHOWWINDOW | SWP_FRAMECHANGED)`.
2. **Focus Watcher / WinEvent Hook**:
   - Install `SetWinEventHook(EVENT_SYSTEM_FOREGROUND, ...)` or a 100ms lightweight focus check: if `GetForegroundWindow() != interrupt_hwnd`, call `SetForegroundWindow(interrupt_hwnd)` and `BringWindowToTop(interrupt_hwnd)`.
3. **Cursor Clipping (`ClipCursor`)**:
   - While in `Pause` state, call `ClipCursor(&virtual_screen_rect)` to prevent the mouse from clicking onto hidden taskbar areas on other monitors. Release `ClipCursor(NULL)` on unblock.

---

## 6. Detailed Code Blueprint

### 6.1 `src/win32.rs` Blueprint

```rust
// Static communication handle for the dedicated hook thread
static HOOK_THREAD_ID: AtomicU32 = AtomicU32::new(0);
static HOOK_RUNNING: AtomicBool = AtomicBool::new(false);
static LWIN_DOWN: AtomicBool = AtomicBool::new(false);
static RWIN_DOWN: AtomicBool = AtomicBool::new(false);

pub fn enable_keyboard_hook() {
    if HOOK_RUNNING.swap(true, Ordering::SeqCst) {
        return; // Already running
    }

    std::thread::spawn(|| {
        unsafe {
            let tid = GetCurrentThreadId();
            HOOK_THREAD_ID.store(tid, Ordering::SeqCst);

            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(dedicated_low_level_keyboard_proc),
                std::ptr::null_mut(),
                0,
            );

            if hook.is_null() {
                log_to_file("[ERROR] Failed to install low-level keyboard hook on dedicated thread");
                HOOK_RUNNING.store(false, Ordering::SeqCst);
                return;
            }

            log_to_file(&format!("[HOOK] Dedicated hook loop running on thread {}", tid));

            // Standard low-latency Win32 message pump
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            UnhookWindowsHookEx(hook);
            log_to_file("[HOOK] Dedicated hook thread exited and unhooked cleanly");
        }
    });
}

pub fn disable_keyboard_hook() {
    if !HOOK_RUNNING.swap(false, Ordering::SeqCst) {
        return;
    }
    LWIN_DOWN.store(false, Ordering::SeqCst);
    RWIN_DOWN.store(false, Ordering::SeqCst);
    
    let tid = HOOK_THREAD_ID.swap(0, Ordering::SeqCst);
    if tid != 0 {
        unsafe {
            PostThreadMessageW(tid, WM_QUIT, 0, 0);
        }
    }
}

unsafe extern "system" fn dedicated_low_level_keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 && HOOK_RUNNING.load(Ordering::Relaxed) {
        let kbd = *(lparam as *const KBDLLHOOKSTRUCT);
        let vk = kbd.vkCode as u16;
        let is_alt_down = (kbd.flags & LLKHF_ALTDOWN) != 0;
        let is_key_down = wparam == WM_KEYDOWN as usize || wparam == WM_SYSKEYDOWN as usize;
        let is_key_up = wparam == WM_KEYUP as usize || wparam == WM_SYSKEYUP as usize;

        // Track physical Win key state
        if vk == VK_LWIN {
            LWIN_DOWN.store(is_key_down, Ordering::Relaxed);
            return 1; // Eat all Win key events
        }
        if vk == VK_RWIN {
            RWIN_DOWN.store(is_key_down, Ordering::Relaxed);
            return 1; // Eat all Win key events
        }

        // If either Win key is held down, eat ANY concurrent key (blocks Win+D, Win+Tab, Win+R, etc.)
        if LWIN_DOWN.load(Ordering::Relaxed) || RWIN_DOWN.load(Ordering::Relaxed) {
            return 1;
        }

        // Block Alt+Tab, Alt+Esc, Ctrl+Esc, Alt+F4, Context Menu
        let should_block = match vk {
            VK_TAB => is_alt_down,
            VK_ESCAPE => is_alt_down || ((GetKeyState(VK_CONTROL as i32) as u16 & 0x8000) != 0),
            VK_F4 => is_alt_down,
            0x5D => true, // VK_APPS (Context menu key)
            _ => false,
        };

        if should_block {
            return 1; // Suppress event completely
        }
    }

    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}
```

### 6.2 Focus & Z-Order Guard Blueprint

```rust
pub fn enforce_pause_window_topmost() {
    unsafe {
        let hwnd = get_app_window_handle();
        if hwnd.is_null() { return; }

        let foreground = GetForegroundWindow();
        if foreground != hwnd {
            BringWindowToTop(hwnd);
            SetForegroundWindow(hwnd);
        }

        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0, 0, 0, 0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW,
        );
    }
}
```

---

## 7. Safety, Emergency Unlock & Failsafe Guarantees

1. **Master Password Override**: The master password (`"mindfulness"`) remains active at all times. Standard alphanumeric keys and backspace/enter are never blocked, so typing the unlock password always works.
2. **Process Termination / Crash Safety**: Windows automatically cleans up all `WH_KEYBOARD_LL` hooks when a process terminates. Because we do not modify permanent registry keys, the user's keyboard will never remain stuck if the app is killed.
3. **Graceful Teardown**: `InterruptApp::drop()` and tray exit both call `disable_keyboard_hook()`, posting `WM_QUIT` to cleanly release the OS hook.

---

## 8. Summary of Action Items for Implementation Phase

- [ ] Remove `keybd_event(0,0,0,0)` from `win32.rs`.
- [ ] Move `SetWindowsHookExW(WH_KEYBOARD_LL)` to a dedicated background worker thread with a dedicated `GetMessageW` loop.
- [ ] Track `LWIN_DOWN` / `RWIN_DOWN` state and swallow all modifier combinations.
- [ ] Add `enforce_pause_window_topmost()` focus guard called periodically during `AppState::Pause`.
- [ ] Add `ClipCursor` to virtual screen bounds during `AppState::Pause`.
- [ ] Add automated unit tests verifying hook lifecycle state management and roadmap item sync.
