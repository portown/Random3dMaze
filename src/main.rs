#![windows_subsystem = "windows"]

use render::begin_paint;
use std::{ffi::c_void, ptr};
use windows::{
    core::{w, HSTRING, PCWSTR},
    Win32::{
        Foundation::{
            ERROR_ALREADY_EXISTS, FALSE, HANDLE, HINSTANCE, HWND, LPARAM, LRESULT, RECT,
            WIN32_ERROR, WPARAM,
        },
        Graphics::Gdi::{GetStockObject, UpdateWindow, HBRUSH, WHITE_BRUSH},
        System::{LibraryLoader::GetModuleHandleW, Threading::CreateMutexW},
        UI::WindowsAndMessaging::{
            AdjustWindowRectEx, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
            GetMessageW, GetWindowLongPtrW, LoadImageW, MessageBoxW, PostQuitMessage,
            RegisterClassExW, SetWindowLongPtrW, ShowWindow, TranslateMessage, CREATESTRUCTW,
            CW_USEDEFAULT, GWLP_USERDATA, HCURSOR, HICON, HMENU, IDC_ARROW, IMAGE_CURSOR,
            IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED, MB_OK, MSG, SW_SHOW, WINDOW_EX_STYLE, WM_CREATE,
            WM_DESTROY, WM_PAINT, WNDCLASSEXW, WNDCLASS_STYLES, WS_CAPTION, WS_MINIMIZEBOX,
            WS_OVERLAPPED, WS_SYSMENU,
        },
    },
};
mod game;
mod map;
mod player;
mod render;

#[derive(thiserror::Error, Debug)]
enum ApplicationError {
    #[error(transparent)]
    WinError(#[from] windows::core::Error),
    #[error(transparent)]
    GameError(#[from] game::Error),
}

fn to_cursor(handle: HANDLE) -> HCURSOR {
    HCURSOR(handle.0)
}

fn to_icon(handle: HANDLE) -> HICON {
    HICON(handle.0)
}

struct WindowData {
    game: game::Game,
    error: Option<ApplicationError>,
}

impl WindowData {
    fn proc(
        &mut self,
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Result<LRESULT, ApplicationError> {
        match msg {
            WM_PAINT => {
                let surface = begin_paint(hwnd);
                self.game.draw(&surface)?;
            }
            WM_DESTROY => unsafe { PostQuitMessage(0) },
            _ => return Ok(unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }),
        }
        Ok(LRESULT(0))
    }

    fn raise_if_error(self) -> Result<(), ApplicationError> {
        if let Some(e) = self.error {
            return Err(e);
        }
        Ok(())
    }
}

fn main() {
    let result = run();

    if let Err(error) = result {
        let message = HSTRING::from(error.to_string());
        unsafe { MessageBoxW(HWND::default(), &message, w!("Error"), MB_OK) };
    }
}

fn run() -> Result<(), ApplicationError> {
    let class_name = w!("jp.portown.maze3d");
    _ = unsafe { CreateMutexW(None, FALSE, class_name) }?;
    // When ERROR_ALREADY_EXISTS occurred, CreateMutexW doesn't return Err, do Ok
    if WIN32_ERROR::from_error(&windows::core::Error::from_win32()) == Some(ERROR_ALREADY_EXISTS) {
        return Ok(());
    }

    let instance_handle = HINSTANCE::from(unsafe { GetModuleHandleW(PCWSTR(ptr::null())) }?);

    let wc = unsafe {
        WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: WNDCLASS_STYLES(0),
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: size_of::<*mut game::Game>() as i32,
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

    let mut window_data = WindowData {
        game: game::Game::new()?,
        error: None,
    };

    let hwnd = unsafe {
        CreateWindowExW(
            window_ex_style,
            class_name,
            w!("3d Maze"),
            window_style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window_rect.right - window_rect.left,
            window_rect.bottom - window_rect.top,
            HWND::default(),
            HMENU::default(),
            instance_handle,
            Some(std::ptr::from_mut(&mut window_data) as *mut c_void),
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

    window_data.raise_if_error()?;

    Ok(())
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if msg == WM_CREATE {
        unsafe {
            let cs = (lparam.0 as *const CREATESTRUCTW).as_ref().unwrap();
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, cs.lpCreateParams as isize);
        }
        return LRESULT(0);
    }

    let data = unsafe {
        let p = GetWindowLongPtrW(hwnd, GWLP_USERDATA);
        if p == 0 {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
        (p as *mut WindowData).as_mut().unwrap()
    };

    match data.proc(hwnd, msg, wparam, lparam) {
        Ok(r) => r,
        Err(e) => {
            data.error = Some(e);
            _ = unsafe { DestroyWindow(hwnd) };
            LRESULT(0)
        }
    }
}
