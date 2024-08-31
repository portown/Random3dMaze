#![windows_subsystem = "windows"]

use std::ptr;
use windows::{
    core::{w, Result, HSTRING, PCWSTR},
    Win32::{
        Foundation::{ERROR_ALREADY_EXISTS, FALSE, HANDLE, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WIN32_ERROR, WPARAM},
        Graphics::Gdi::{BeginPaint, EndPaint, GetStockObject, TextOutW, UpdateWindow, HBRUSH, PAINTSTRUCT, WHITE_BRUSH},
        System::{LibraryLoader::GetModuleHandleW, Threading::CreateMutexW},
        UI::WindowsAndMessaging::{
            AdjustWindowRectEx, CreateWindowExW, DefWindowProcW, DispatchMessageW,
            GetClientRect, GetMessageW, LoadImageW, PostQuitMessage, RegisterClassExW,
            ShowWindow, TranslateMessage, CW_USEDEFAULT, HCURSOR, HICON, HMENU, IDC_ARROW,
            IMAGE_CURSOR, IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED, MSG, SW_SHOW, WINDOW_EX_STYLE,
            WM_DESTROY, WM_PAINT, WNDCLASSEXW, WNDCLASS_STYLES, WS_CAPTION, WS_MINIMIZEBOX, WS_OVERLAPPED, WS_SYSMENU
        },
    },
};

fn to_cursor(handle: HANDLE) -> HCURSOR {
    HCURSOR(handle.0)
}

fn to_icon(handle: HANDLE) -> HICON {
    HICON(handle.0)
}

fn main() {
    let result = run();

    if let Err(error) = result {
        error.code().unwrap()
    }
}

fn run() -> Result<()> {
    let class_name = w!("jp.portown.maze3d");
    _ = unsafe { CreateMutexW(None, FALSE, class_name) }?;
    // When ERROR_ALREADY_EXISTS occurred, CreateMutexW doesn't return Err, do Ok
    if WIN32_ERROR::from_error(&windows::core::Error::from_win32()) == Some(ERROR_ALREADY_EXISTS) {
        return Ok(())
    }

    let instance_handle = HINSTANCE::from(unsafe { GetModuleHandleW(PCWSTR(ptr::null())) }?);

    let wc = unsafe {
        WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: WNDCLASS_STYLES(0),
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance_handle,
            hIcon: to_icon(LoadImageW(
                instance_handle,
                w!("IDI_MAIN"),
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTSIZE | LR_SHARED,
            )?),
            hCursor: to_cursor(LoadImageW(
                HINSTANCE::default(),
                IDC_ARROW,
                IMAGE_CURSOR,
                0,
                0,
                LR_DEFAULTSIZE | LR_SHARED,
            )?),
            hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0),
            lpszMenuName: PCWSTR(ptr::null()),
            lpszClassName: class_name,
            hIconSm: to_icon(LoadImageW(
                instance_handle,
                w!("IDI_MAIN"),
                IMAGE_ICON,
                0,
                0,
                LR_DEFAULTSIZE | LR_SHARED,
            )?),
        }
    };
    assert_ne!(unsafe { RegisterClassExW(&wc) }, 0);

    let window_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION | WS_MINIMIZEBOX;
    let window_ex_style = WINDOW_EX_STYLE::default();

    let mut window_rect = RECT {
        left: 0,
        top: 0,
        right: 48 * 3 + 256 * 2,
        bottom: 48 * 2 + 256,
    };

    unsafe {
        AdjustWindowRectEx(&mut window_rect, window_style, FALSE, window_ex_style)?;
    }

    let hwnd = unsafe {
        CreateWindowExW(
            window_ex_style,
            w!("jp.portown.maze3d"),
            w!("3d Maze"),
            window_style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window_rect.right - window_rect.left,
            window_rect.bottom - window_rect.top,
            HWND::default(),
            HMENU::default(),
            instance_handle,
            None,
        )?
    };

    unsafe {
        _ = ShowWindow(hwnd, SW_SHOW);
        _ = UpdateWindow(hwnd);
    }

    let mut msg = MSG::default();
    loop {
        let ret = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        if ret == false || ret.0 == -1 {
            break;
        }
        unsafe {
            _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => unsafe {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            let mut rc = RECT::default();
            _ = GetClientRect(hwnd, &mut rc);
            let s = format!("Client rect: {}x{}", rc.right - rc.left, rc.bottom - rc.top);
            let hs = HSTRING::from(s);
            _ = TextOutW(hdc, 0, 0, &hs.as_wide());
            _ = EndPaint(hwnd, &mut ps);
        },
        WM_DESTROY => unsafe { PostQuitMessage(0) },
        _ => return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
    LRESULT(0)
}
