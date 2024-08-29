use std::ffi::c_void;

use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{COLORREF, HWND, RECT},
        Graphics::Gdi::{
            BeginPaint, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreatePen,
            CreateSolidBrush, DeleteDC, DeleteObject, EndPaint, GetObjectW, GetStockObject,
            Rectangle, SelectObject, BITMAP, HBITMAP, HBRUSH, HDC, HPEN, NULL_BRUSH, NULL_PEN,
            PAINTSTRUCT, PS_SOLID, SRCCOPY,
        },
        System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::{
            GetClientRect, LoadImageW, IMAGE_BITMAP, LR_CREATEDIBSECTION, LR_LOADFROMFILE,
        },
    },
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Cannot load a bitmap from file ({file_path}): {source}")]
    BitmapLoadError {
        file_path: String,
        #[source]
        source: windows::core::Error,
    },
}

pub struct Pen {
    handle: HPEN,
}

impl Drop for Pen {
    fn drop(&mut self) {
        _ = unsafe { DeleteObject(self.handle) };
    }
}

pub fn create_solid_pen(width: i32, color: COLORREF) -> Pen {
    let handle = unsafe { CreatePen(PS_SOLID, width, color) };
    Pen { handle }
}

pub struct Brush {
    handle: HBRUSH,
}

impl Drop for Brush {
    fn drop(&mut self) {
        _ = unsafe { DeleteObject(self.handle) };
    }
}

pub fn create_solid_brush(color: COLORREF) -> Brush {
    let handle = unsafe { CreateSolidBrush(color) };
    Brush { handle }
}

pub trait Surface {
    fn get_hdc(&self) -> HDC;
    fn get_size(&self) -> (u32, u32);

    fn fill_rect(&self, rect: &RECT, brush: &Brush) {
        let hdc = self.get_hdc();
        unsafe {
            let null_pen = GetStockObject(NULL_PEN);
            let old_pen = SelectObject(hdc, null_pen);
            let old_brush = SelectObject(hdc, brush.handle);
            _ = Rectangle(hdc, rect.left, rect.top, rect.right, rect.bottom);
            SelectObject(hdc, old_brush);
            SelectObject(hdc, old_pen);
        }
    }

    fn draw_rect(&self, rect: &RECT, pen: &Pen) {
        let hdc = self.get_hdc();
        unsafe {
            let null_brush = GetStockObject(NULL_BRUSH);
            let old_pen = SelectObject(hdc, pen.handle);
            let old_brush = SelectObject(hdc, null_brush);
            _ = Rectangle(hdc, rect.left, rect.top, rect.right, rect.bottom);
            SelectObject(hdc, old_brush);
            SelectObject(hdc, old_pen);
        }
    }

    fn copy_from(&self, dest_rect: &RECT, src: &impl Surface, src_x: i32, src_y: i32) {
        unsafe {
            _ = BitBlt(
                self.get_hdc(),
                dest_rect.left,
                dest_rect.top,
                dest_rect.right - dest_rect.left,
                dest_rect.bottom - dest_rect.top,
                src.get_hdc(),
                src_x,
                src_y,
                SRCCOPY,
            );
        }
    }
}

pub struct PrimarySurface {
    hdc: HDC,
    hwnd: HWND,
    ps: PAINTSTRUCT,
}

impl PrimarySurface {
    pub fn create_surface(&self, width: u32, height: u32) -> BackSurface {
        unsafe {
            let hdc = CreateCompatibleDC(self.hdc);
            let bitmap = CreateCompatibleBitmap(self.hdc, width as i32, height as i32);
            SelectObject(hdc, bitmap);

            BackSurface { hdc, bitmap }
        }
    }

    pub fn load_bitmap(&self, file_path: &str) -> Result<BackSurface, Error> {
        let bitmap = unsafe {
            let hinst = GetModuleHandleW(None).map_err(|e| Error::BitmapLoadError {
                file_path: file_path.to_owned(),
                source: e,
            })?;
            let handle = LoadImageW(
                hinst,
                &HSTRING::from(file_path),
                IMAGE_BITMAP,
                0,
                0,
                LR_CREATEDIBSECTION | LR_LOADFROMFILE,
            )
            .map_err(|e| Error::BitmapLoadError {
                file_path: file_path.to_owned(),
                source: e,
            })?;
            HBITMAP { 0: handle.0 }
        };

        let new_hdc = unsafe { CreateCompatibleDC(self.hdc) };

        unsafe { SelectObject(new_hdc, bitmap) };

        Ok(BackSurface {
            hdc: new_hdc,
            bitmap,
        })
    }
}

impl Surface for PrimarySurface {
    fn get_hdc(&self) -> HDC {
        self.hdc
    }

    fn get_size(&self) -> (u32, u32) {
        let mut rect = RECT::default();
        _ = unsafe { GetClientRect(self.hwnd, &mut rect) };
        return (
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        );
    }
}

impl Drop for PrimarySurface {
    fn drop(&mut self) {
        _ = unsafe { EndPaint(self.hwnd, &mut self.ps) };
    }
}

pub fn begin_paint(hwnd: HWND) -> PrimarySurface {
    let mut ps = PAINTSTRUCT::default();
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    PrimarySurface { hdc, hwnd, ps }
}

pub struct BackSurface {
    hdc: HDC,
    bitmap: HBITMAP,
}

impl Surface for BackSurface {
    fn get_hdc(&self) -> HDC {
        self.hdc
    }

    fn get_size(&self) -> (u32, u32) {
        let mut info = BITMAP::default();
        unsafe {
            GetObjectW(
                self.bitmap,
                size_of::<BITMAP>() as i32,
                Some(std::ptr::from_mut(&mut info) as *mut c_void),
            );
        };

        (info.bmWidth as u32, info.bmHeight as u32)
    }
}

impl Drop for BackSurface {
    fn drop(&mut self) {
        unsafe {
            _ = DeleteObject(self.bitmap);
            _ = DeleteDC(self.hdc);
        }
    }
}

pub fn rect(left: i32, top: i32, right: i32, bottom: i32) -> RECT {
    RECT {
        left,
        top,
        right,
        bottom,
    }
}

pub fn rect_wh(left: i32, top: i32, width: i32, height: i32) -> RECT {
    RECT {
        left,
        top,
        right: left + width,
        bottom: top + height,
    }
}

pub fn color_rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((0x00u32 << 24) | ((b as u32) << 16) | ((g as u32) << 8) | (r as u32))
}
