use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    VK_ESCAPE, VK_F4, VK_LWIN, VK_RWIN, VK_TAB,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BringWindowToTop, CallNextHookEx, GetForegroundWindow, GetSystemMetrics,
    GetWindowThreadProcessId, IsWindow, SetForegroundWindow, SetWindowLongW, SetWindowPos,
    SetWindowsHookExW, ShowWindow, UnhookWindowsHookEx, GWL_STYLE, HWND_NOTOPMOST, HWND_TOPMOST,
    KBDLLHOOKSTRUCT, LLKHF_ALTDOWN, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
    SM_YVIRTUALSCREEN, SWP_FRAMECHANGED, SWP_SHOWWINDOW, SW_RESTORE, SW_SHOW, WH_KEYBOARD_LL,
    WM_KEYDOWN, WM_SYSKEYDOWN, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_VISIBLE,
};

static HOOK_HANDLE: AtomicIsize = AtomicIsize::new(0);
static HOOK_ENABLED: AtomicBool = AtomicBool::new(false);
static SAVED_FOREGROUND_HWND: AtomicIsize = AtomicIsize::new(0);

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

pub fn make_app_window_fullscreen_topmost() {
    unsafe {
        let hwnd = GetForegroundWindow();
        if !hwnd.is_null() {
            let rect = get_virtual_screen_rect();
            SetWindowLongW(hwnd, GWL_STYLE, (WS_POPUP | WS_VISIBLE) as i32);
            SetWindowPos(
                hwnd,
                HWND_TOPMOST,
                rect.x,
                rect.y,
                rect.width,
                rect.height,
                SWP_SHOWWINDOW | SWP_FRAMECHANGED,
            );
        }
    }
}

pub fn restore_app_window_normal() {
    unsafe {
        let hwnd = GetForegroundWindow();
        if !hwnd.is_null() {
            SetWindowLongW(hwnd, GWL_STYLE, WS_OVERLAPPEDWINDOW as i32);
            SetWindowPos(
                hwnd,
                HWND_NOTOPMOST,
                100,
                100,
                540,
                360,
                SWP_SHOWWINDOW | SWP_FRAMECHANGED,
            );
        }
    }
}

pub fn enable_keyboard_hook() {
    if HOOK_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    HOOK_ENABLED.store(true, Ordering::SeqCst);

    unsafe {
        let hmod = GetModuleHandleW(std::ptr::null());
        let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(low_level_keyboard_proc), hmod, 0);
        if !hook.is_null() {
            HOOK_HANDLE.store(hook as isize, Ordering::SeqCst);
        }
    }
}

pub fn disable_keyboard_hook() {
    HOOK_ENABLED.store(false, Ordering::SeqCst);
    let hook = HOOK_HANDLE.swap(0, Ordering::SeqCst);
    if hook != 0 {
        unsafe {
            UnhookWindowsHookEx(hook as _);
        }
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
        let is_syskey = wparam == WM_SYSKEYDOWN as usize || wparam == WM_KEYDOWN as usize;
        let is_alt_down = (kbd.flags & LLKHF_ALTDOWN) != 0;

        // Block Alt+Tab, Alt+Esc, Ctrl+Esc, Win key, Alt+F4
        let block = match vk {
            VK_TAB => is_alt_down,
            VK_ESCAPE => is_alt_down || (kbd.flags & 0x01 != 0), // Ctrl+Esc or Alt+Esc
            VK_LWIN | VK_RWIN => true,
            VK_F4 => is_alt_down,
            _ => false,
        };

        if block && is_syskey {
            return 1; // Block key
        }
    }

    let hook = HOOK_HANDLE.load(Ordering::SeqCst);
    CallNextHookEx(hook as _, code, wparam, lparam)
}
