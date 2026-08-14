use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, WPARAM, GetLastError, POINT, RECT};
use windows_sys::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    VK_ESCAPE, VK_F4, VK_LWIN, VK_RWIN, VK_TAB, keybd_event, KEYEVENTF_KEYUP,
};
use windows_sys::Win32::System::Diagnostics::Debug::MessageBeep;
use windows_sys::Win32::Graphics::Gdi::{CreateBitmap, DeleteObject, HGDIOBJ};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BringWindowToTop, CallNextHookEx, EnumThreadWindows, GetForegroundWindow, GetSystemMetrics,
    GetWindowTextW, GetWindowThreadProcessId, IsWindow, SetForegroundWindow, SetWindowLongW, SetWindowPos,
    SetWindowsHookExW, ShowWindow, UnhookWindowsHookEx, GWL_STYLE, HWND_TOPMOST,
    KBDLLHOOKSTRUCT, LLKHF_ALTDOWN, MB_ICONINFORMATION, MB_ICONWARNING, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
    SM_YVIRTUALSCREEN, SWP_FRAMECHANGED, SWP_SHOWWINDOW, HWND_NOTOPMOST, SW_RESTORE, SW_SHOW, WH_KEYBOARD_LL,
    WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_VISIBLE,
    CreatePopupMenu, AppendMenuW, TrackPopupMenu, DestroyMenu, GetCursorPos, SetWindowLongPtrW,
    GWLP_WNDPROC, WM_USER, WM_LBUTTONUP, WM_LBUTTONDBLCLK, WM_RBUTTONUP, MF_STRING, MF_SEPARATOR,
    TPM_RETURNCMD, TPM_NONOTIFY, WNDPROC, CreateIconIndirect, ICONINFO, WM_CLOSE, WM_SYSCOMMAND, SC_MINIMIZE, DestroyIcon, IsWindowVisible,
    GetWindowLongW, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW, SWP_NOSIZE, SWP_NOZORDER, GetWindowRect, SM_CXSCREEN, SM_CYSCREEN, SW_HIDE,
};
use windows_sys::Win32::UI::Shell::{
    NOTIFYICONDATAW, Shell_NotifyIconW, NIM_ADD, NIM_DELETE, NIF_ICON, NIF_MESSAGE, NIF_TIP
};

static HOOK_HANDLE: AtomicIsize = AtomicIsize::new(0);
static HOOK_ENABLED: AtomicBool = AtomicBool::new(false);
static SAVED_FOREGROUND_HWND: AtomicIsize = AtomicIsize::new(0);

pub static SAVED_WINDOW_X: AtomicIsize = AtomicIsize::new(-1);
pub static SAVED_WINDOW_Y: AtomicIsize = AtomicIsize::new(-1);
pub static SAVED_WINDOW_WIDTH: AtomicIsize = AtomicIsize::new(640);
pub static SAVED_WINDOW_HEIGHT: AtomicIsize = AtomicIsize::new(560);

pub static WAS_VISIBLE_BEFORE_LOCK: AtomicBool = AtomicBool::new(true);
pub static VISIBILITY_RECORDED: AtomicBool = AtomicBool::new(false);

pub fn record_visibility_before_lock() {
    if VISIBILITY_RECORDED.swap(true, Ordering::SeqCst) {
        return; // Already recorded by tray menu!
    }
    unsafe {
        let hwnd = get_app_window_handle();
        if !hwnd.is_null() {
            let visible = IsWindowVisible(hwnd) != 0;
            WAS_VISIBLE_BEFORE_LOCK.store(visible, Ordering::SeqCst);
            log_to_file(&format!("[DEBUG] record_visibility_before_lock: {}", visible));
        }
    }
}

pub struct VirtualScreenRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub fn get_virtual_screen_rect() -> VirtualScreenRect {
    unsafe {
        let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let y = GetSystemMetrics(SM_YVIRTUALSCREEN);
        let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);

        VirtualScreenRect {
            x,
            y,
            width: if width > 0 { width } else { 1920 },
            height: if height > 0 { height } else { 1080 },
        }
    }
}

pub fn capture_foreground_window() {
    unsafe {
        let hwnd = GetForegroundWindow();
        if !hwnd.is_null() {
            SAVED_FOREGROUND_HWND.store(hwnd as isize, Ordering::SeqCst);
        }
    }
}

pub fn restore_foreground_window() {
    let saved = SAVED_FOREGROUND_HWND.swap(0, Ordering::SeqCst);
    if saved == 0 {
        return;
    }
    let target_hwnd = saved as HWND;
    unsafe {
        if IsWindow(target_hwnd) == 0 {
            return;
        }

        let _current_hwnd = GetForegroundWindow();
        let current_thread = GetCurrentThreadId();
        let target_thread = GetWindowThreadProcessId(target_hwnd, std::ptr::null_mut());

        if current_thread != target_thread && target_thread != 0 {
            AttachThreadInput(current_thread, target_thread, 1);
            ShowWindow(target_hwnd, SW_RESTORE);
            BringWindowToTop(target_hwnd);
            SetForegroundWindow(target_hwnd);
            AttachThreadInput(current_thread, target_thread, 0);
        } else {
            ShowWindow(target_hwnd, SW_SHOW);
            BringWindowToTop(target_hwnd);
            SetForegroundWindow(target_hwnd);
        }
    }
}

unsafe extern "system" fn enum_thread_window_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let title = get_window_title(hwnd);
    log_to_file(&format!("[DEBUG] enum_thread_window_callback: hwnd = {:?}, title = '{}'", hwnd, title));
    if title.contains("Interrupt") {
        let hwnd_ptr = lparam as *mut HWND;
        *hwnd_ptr = hwnd;
        0 // Stop enumeration
    } else {
        1 // Continue enumeration
    }
}

pub fn get_app_window_handle() -> HWND {
    let mut hwnd: HWND = std::ptr::null_mut();
    unsafe {
        EnumThreadWindows(
            GetCurrentThreadId(),
            Some(enum_thread_window_callback),
            &mut hwnd as *mut HWND as LPARAM,
        );
    }
    hwnd
}

unsafe fn get_window_title(hwnd: HWND) -> String {
    let mut buf = [0u16; 256];
    let len = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
    if len > 0 {
        String::from_utf16_lossy(&buf[..len as usize])
    } else {
        "Unknown Title".to_string()
    }
}

pub fn make_app_window_fullscreen_topmost() {
    unsafe {
        let hwnd = get_app_window_handle();
        let title = if !hwnd.is_null() { get_window_title(hwnd) } else { "".to_string() };
        log_to_file(&format!("[DEBUG] make_app_window_fullscreen_topmost: app hwnd = {:?} (title: '{}')", hwnd, title));
        
        if !hwnd.is_null() {
            let rect = get_virtual_screen_rect();
            let style_res = SetWindowLongW(hwnd, GWL_STYLE, (WS_POPUP | WS_VISIBLE) as i32);
            log_to_file(&format!("[DEBUG] make_app_window_fullscreen_topmost: SetWindowLongW old style = {}", style_res));
            let pos_res = SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                SWP_SHOWWINDOW | SWP_FRAMECHANGED,
            );
            log_to_file(&format!("[DEBUG] make_app_window_fullscreen_topmost: SetWindowPos result = {}", pos_res));
        } else {
            log_to_file("[DEBUG] make_app_window_fullscreen_topmost: app hwnd is null!");
        }
    }
}

pub fn restore_app_window_normal() {
    unsafe {
        let hwnd = get_app_window_handle();
        let title = if !hwnd.is_null() { get_window_title(hwnd) } else { "".to_string() };
        log_to_file(&format!("[DEBUG] restore_app_window_normal: app hwnd = {:?} (title: '{}')", hwnd, title));
        
        if !hwnd.is_null() {
            let style_res = SetWindowLongW(hwnd, GWL_STYLE, WS_OVERLAPPEDWINDOW as i32);
            log_to_file(&format!("[DEBUG] restore_app_window_normal: SetWindowLongW old style = {}", style_res));
            
            let was_visible = WAS_VISIBLE_BEFORE_LOCK.load(Ordering::SeqCst);
            log_to_file(&format!("[DEBUG] restore_app_window_normal: was_visible = {}", was_visible));
            
            show_app_window(was_visible);
            
            VISIBILITY_RECORDED.store(false, Ordering::SeqCst);
        } else {
            log_to_file("[DEBUG] restore_app_window_normal: app hwnd is null!");
        }
    }
}

pub fn enable_keyboard_hook() {
    if HOOK_ENABLED.load(Ordering::SeqCst) {
        log_to_file("[HOOK] Hook already enabled");
        return;
    }
    HOOK_ENABLED.store(true, Ordering::SeqCst);

    unsafe {
        let tid = GetCurrentThreadId();
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), std::ptr::null_mut(), 0);
        if !hook.is_null() {
            HOOK_HANDLE.store(hook as isize, Ordering::SeqCst);
            log_to_file(&format!("[HOOK] Hook successfully enabled on thread {}: {:?}", tid, hook));
        } else {
            let err = GetLastError();
            log_to_file(&format!("[HOOK] SetWindowsHookExW failed on thread {} with error code: {}", tid, err));
        }
    }
}

pub fn disable_keyboard_hook() {
    log_to_file("[HOOK] disable_keyboard_hook() called");
    HOOK_ENABLED.store(false, Ordering::SeqCst);
    let hook = HOOK_HANDLE.swap(0, Ordering::SeqCst);
    if hook != 0 {
        unsafe {
            let res = UnhookWindowsHookEx(hook as _);
            log_to_file(&format!("[HOOK] Hook disabled. UnhookWindowsHookEx result = {}", res));
        }
    } else {
        log_to_file("[HOOK] disable_keyboard_hook: Hook handle was 0");
    }
}

unsafe extern "system" fn low_level_keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 && HOOK_ENABLED.load(Ordering::SeqCst) {
        let kbd = *(lparam as *const KBDLLHOOKSTRUCT);
        let vk = kbd.vkCode as u16;
        let is_key_event = wparam == WM_KEYDOWN as usize
            || wparam == WM_SYSKEYDOWN as usize
            || wparam == WM_KEYUP as usize
            || wparam == WM_SYSKEYUP as usize;
        let is_alt_down = (kbd.flags & LLKHF_ALTDOWN) != 0;

        // Block Alt+Tab, Alt+Esc, Ctrl+Esc, Win key, Alt+F4
        let block = match vk {
            VK_TAB => is_alt_down,
            VK_ESCAPE => is_alt_down || (kbd.flags & 0x01 != 0), // Ctrl+Esc or Alt+Esc
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

pub static LOGGING_ENABLED: AtomicBool = AtomicBool::new(false);

pub fn init_logging(enabled: bool) {
    LOGGING_ENABLED.store(enabled, Ordering::SeqCst);
    if enabled {
        if let Some(mut path) = dirs::config_dir() {
            path.push("interrupt");
            let _ = std::fs::create_dir_all(&path);
            path.push("debug.log");
            let _ = std::fs::write(&path, ""); // Overwrite/clear
            log_to_file("[LOG] Logging initialized and cleared");
        }
    }
}

pub fn log_to_file(msg: &str) {
    if !LOGGING_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    if let Some(mut path) = dirs::config_dir() {
        path.push("interrupt");
        path.push("debug.log");
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            use std::io::Write;
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            let _ = writeln!(file, "[{}] {}", time, msg);
        }
    }
}

pub fn play_sound_warning() {
    unsafe {
        MessageBeep(MB_ICONWARNING);
    }
}

pub fn play_sound_info() {
    unsafe {
        MessageBeep(MB_ICONINFORMATION);
    }
}

pub fn show_app_window(visible: bool) {
    unsafe {
        let hwnd = get_app_window_handle();
        if !hwnd.is_null() {
            log_to_file(&format!("[DEBUG] show_app_window: visible = {}", visible));
            if visible {
                let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
                style &= !WS_EX_TOOLWINDOW;
                style |= WS_EX_APPWINDOW;
                SetWindowLongW(hwnd, GWL_EXSTYLE, style as i32);

                let mut rect: RECT = std::mem::zeroed();
                GetWindowRect(hwnd, &mut rect);

                let is_offscreen = rect.left <= -10000 || rect.top <= -10000;
                let (x, y, w, h) = if is_offscreen {
                    let saved_x = SAVED_WINDOW_X.load(Ordering::SeqCst);
                    let saved_y = SAVED_WINDOW_Y.load(Ordering::SeqCst);
                    let saved_w = SAVED_WINDOW_WIDTH.load(Ordering::SeqCst) as i32;
                    let saved_h = SAVED_WINDOW_HEIGHT.load(Ordering::SeqCst) as i32;

                    let width = if saved_w > 0 { saved_w } else { 640 };
                    let height = if saved_h > 0 { saved_h } else { 560 };

                    if saved_x > -10000 && saved_y > -10000 {
                        (saved_x as i32, saved_y as i32, width, height)
                    } else {
                        let screen_w = GetSystemMetrics(SM_CXSCREEN);
                        let screen_h = GetSystemMetrics(SM_CYSCREEN);
                        let cx = if screen_w > 0 { (screen_w - width) / 2 } else { 100 };
                        let cy = if screen_h > 0 { (screen_h - height) / 2 } else { 100 };
                        (cx, cy, width, height)
                    }
                } else {
                    let width = rect.right - rect.left;
                    let height = rect.bottom - rect.top;
                    (rect.left, rect.top, if width > 0 { width } else { 640 }, if height > 0 { height } else { 560 })
                };

                log_to_file(&format!("[DEBUG] show_app_window: restoring window to position ({}, {}), size {}x{}", x, y, w, h));

                SetWindowPos(
                    hwnd,
                    HWND_NOTOPMOST,
                    x,
                    y,
                    w,
                    h,
                    SWP_SHOWWINDOW | SWP_FRAMECHANGED,
                );
                ShowWindow(hwnd, SW_RESTORE);
                ShowWindow(hwnd, SW_SHOW);
                BringWindowToTop(hwnd);
                SetForegroundWindow(hwnd);
            } else {
                let mut rect: RECT = std::mem::zeroed();
                GetWindowRect(hwnd, &mut rect);
                if rect.left > -10000 && rect.top > -10000 {
                    SAVED_WINDOW_X.store(rect.left as isize, Ordering::SeqCst);
                    SAVED_WINDOW_Y.store(rect.top as isize, Ordering::SeqCst);
                    let w = rect.right - rect.left;
                    let h = rect.bottom - rect.top;
                    if w > 0 { SAVED_WINDOW_WIDTH.store(w as isize, Ordering::SeqCst); }
                    if h > 0 { SAVED_WINDOW_HEIGHT.store(h as isize, Ordering::SeqCst); }
                    log_to_file(&format!("[DEBUG] show_app_window: saved position ({}, {}), size {}x{}", rect.left, rect.top, w, h));
                }

                let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
                style &= !WS_EX_APPWINDOW;
                style |= WS_EX_TOOLWINDOW;
                SetWindowLongW(hwnd, GWL_EXSTYLE, style as i32);

                SetWindowPos(
                    hwnd,
                    std::ptr::null_mut(),
                    -32000,
                    -32000,
                    0,
                    0,
                    SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
                );
                ShowWindow(hwnd, SW_HIDE);
            }
        }
    }
}

fn encode_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub static ORIGINAL_WNDPROC: AtomicIsize = AtomicIsize::new(0);
pub static PENDING_TRAY_COMMAND: AtomicIsize = AtomicIsize::new(0);
pub static SHOULD_EXIT: AtomicBool = AtomicBool::new(false);
static TRAY_HICON: AtomicIsize = AtomicIsize::new(0);

pub fn poll_pending_tray_command() -> isize {
    PENDING_TRAY_COMMAND.swap(0, Ordering::SeqCst)
}

pub unsafe extern "system" fn app_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let original = ORIGINAL_WNDPROC.load(Ordering::SeqCst);
    if msg == WM_USER + 1 {
        let event = lparam as u32;
        log_to_file(&format!("[DEBUG] app_wnd_proc: received WM_USER+1 message, event = {}", event));
        if event == WM_LBUTTONUP || event == WM_LBUTTONDBLCLK {
            log_to_file("[DEBUG] app_wnd_proc: Left click or Double click detected -> Showing window");
            show_app_window(true);
            return 0;
        } else if event == WM_RBUTTONUP {
            log_to_file("[DEBUG] app_wnd_proc: Right click detected -> Showing context menu");
            show_tray_context_menu(hwnd);
            return 0;
        }
    }
    
    if msg == WM_CLOSE {
        if SHOULD_EXIT.load(Ordering::SeqCst) {
            log_to_file("[DEBUG] app_wnd_proc: WM_CLOSE allowed because SHOULD_EXIT is true");
        } else {
            log_to_file("[DEBUG] app_wnd_proc: WM_CLOSE intercepted -> Hiding window instead of closing");
            show_app_window(false);
            return 0;
        }
    }
    
    if msg == WM_SYSCOMMAND {
        let cmd = wparam as u32 & 0xFFF0;
        if cmd == SC_MINIMIZE {
            log_to_file("[DEBUG] app_wnd_proc: SC_MINIMIZE intercepted -> Hiding window instead of minimizing to taskbar");
            show_app_window(false);
            return 0;
        }
    }
    
    if msg == 2 {
        log_to_file("[DEBUG] app_wnd_proc: received WM_DESTROY message");
    }
    if msg == 130 {
        log_to_file("[DEBUG] app_wnd_proc: received WM_NCDESTROY message");
    }
    
    if original != 0 {
        let prev_proc: WNDPROC = std::mem::transmute(original);
        if let Some(proc) = prev_proc {
            proc(hwnd, msg, wparam, lparam)
        } else {
            0
        }
    } else {
        0
    }
}

unsafe fn show_tray_context_menu(hwnd: HWND) {
    let hmenu = CreatePopupMenu();
    let open_str = encode_wide("Open Interrupt");
    let lock_str = encode_wide("Lock Screen Now");
    let exit_str = encode_wide("Exit");

    AppendMenuW(hmenu, MF_STRING, 1001, open_str.as_ptr());
    AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());
    AppendMenuW(hmenu, MF_STRING, 1002, lock_str.as_ptr());
    AppendMenuW(hmenu, MF_SEPARATOR, 0, std::ptr::null());
    AppendMenuW(hmenu, MF_STRING, 1003, exit_str.as_ptr());

    let mut pt = POINT { x: 0, y: 0 };
    GetCursorPos(&mut pt);
    SetForegroundWindow(hwnd);
    let cmd = TrackPopupMenu(
        hmenu,
        TPM_RETURNCMD | TPM_NONOTIFY,
        pt.x,
        pt.y,
        0,
        hwnd,
        std::ptr::null(),
    );
    DestroyMenu(hmenu);
    log_to_file(&format!("[DEBUG] show_tray_context_menu: TrackPopupMenu returned cmd = {}", cmd));
 
    if cmd == 1001 {
        log_to_file("[DEBUG] show_tray_context_menu: Open requested -> Showing window");
        show_app_window(true);
    } else if cmd == 1002 {
        log_to_file("[DEBUG] show_tray_context_menu: Lock requested -> Showing window and setting lock command");
        record_visibility_before_lock();
        show_app_window(true);
        PENDING_TRAY_COMMAND.store(cmd as isize, Ordering::SeqCst);
    } else if cmd == 1003 {
        log_to_file("[DEBUG] show_tray_context_menu: Exit requested -> Disabling hook, unregistering tray and exiting process");
        disable_keyboard_hook();
        unregister_tray_icon();
        std::process::exit(0);
    }
}

unsafe fn create_hicon_from_rgba(rgba: &[u8], width: i32, height: i32) -> windows_sys::Win32::UI::WindowsAndMessaging::HICON {
    let mut bgra = vec![0u8; rgba.len()];
    for i in (0..rgba.len()).step_by(4) {
        bgra[i] = rgba[i + 2];     // B
        bgra[i + 1] = rgba[i + 1]; // G
        bgra[i + 2] = rgba[i];     // R
        bgra[i + 3] = rgba[i + 3]; // A
    }

    let hbm_color = CreateBitmap(width, height, 1, 32, bgra.as_ptr() as *const _);
    let mask_bits = vec![0u8; (width * height / 8) as usize];
    let hbm_mask = CreateBitmap(width, height, 1, 1, mask_bits.as_ptr() as *const _);

    let icon_info = ICONINFO {
        fIcon: 1,
        xHotspot: 0,
        yHotspot: 0,
        hbmMask: hbm_mask,
        hbmColor: hbm_color,
    };

    let hicon = CreateIconIndirect(&icon_info);

    DeleteObject(hbm_color as HGDIOBJ);
    DeleteObject(hbm_mask as HGDIOBJ);

    hicon
}

pub fn register_tray_icon() {
    unsafe {
        let hwnd = get_app_window_handle();
        if hwnd.is_null() {
            log_to_file("[ERROR] register_tray_icon: app hwnd is null!");
            return;
        }

        // Subclass window if not already done
        if ORIGINAL_WNDPROC.load(Ordering::SeqCst) == 0 {
            let subclassed = SetWindowLongPtrW(hwnd, GWLP_WNDPROC, app_wnd_proc as *const () as isize);
            ORIGINAL_WNDPROC.store(subclassed, Ordering::SeqCst);
            log_to_file(&format!("[LOG] Window subclassed. Original WndProc: {}", subclassed));
        }

        let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        nid.uCallbackMessage = WM_USER + 1;

        // Generate custom RGBA icon pixels
        let mut icon_rgba = vec![0u8; 16 * 16 * 4];
        for y in 0..16 {
            for x in 0..16 {
                let idx = (y * 16 + x) * 4;
                let dx = x as f32 - 7.5;
                let dy = y as f32 - 7.5;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= 6.0 {
                    icon_rgba[idx] = 99;      // R
                    icon_rgba[idx + 1] = 102;  // G
                    icon_rgba[idx + 2] = 241;  // B
                    icon_rgba[idx + 3] = 255;  // A
                }
                if dist <= 2.0 {
                    icon_rgba[idx] = 244;      // R
                    icon_rgba[idx + 1] = 63;   // G
                    icon_rgba[idx + 2] = 94;   // B
                    icon_rgba[idx + 3] = 255;  // A
                }
            }
        }
        let hicon = create_hicon_from_rgba(&icon_rgba, 16, 16);
        TRAY_HICON.store(hicon as isize, Ordering::SeqCst);
        nid.hIcon = hicon;

        let tip = encode_wide("Interrupt - Screen Break Manager");
        let len = tip.len().min(nid.szTip.len() - 1);
        nid.szTip[..len].copy_from_slice(&tip[..len]);

        Shell_NotifyIconW(NIM_ADD, &nid);
        log_to_file("[LOG] Native tray icon registered with custom color icon");
    }
}

pub fn unregister_tray_icon() {
    unsafe {
        let hwnd = get_app_window_handle();
        if hwnd.is_null() {
            return;
        }
        let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
        nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = 1;
        Shell_NotifyIconW(NIM_DELETE, &nid);
        log_to_file("[LOG] Native tray icon unregistered");

        let hicon = TRAY_HICON.swap(0, Ordering::SeqCst);
        if hicon != 0 {
            DestroyIcon(hicon as windows_sys::Win32::UI::WindowsAndMessaging::HICON);
        }
    }
}
