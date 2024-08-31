use std::ffi::c_void;

use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{COLORREF, HWND, POINT, RECT},
        Graphics::Gdi::{
            BeginPaint, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreateFontW, CreatePen, CreateSolidBrush, DeleteDC, DeleteObject, EndPaint, GetObjectW, GetStockObject, Polygon, Rectangle, SelectObject, SetTextColor, TextOutW, BITMAP, CLIP_DEFAULT_PRECIS, DEFAULT_CHARSET, DEFAULT_PITCH, DEFAULT_QUALITY, FF_DONTCARE, FW_NORMAL, HBITMAP, HBRUSH, HDC, HFONT, HPEN, NULL_BRUSH, NULL_PEN, OUT_DEFAULT_PRECIS, PAINTSTRUCT, PS_SOLID, SRCCOPY
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
    #[error("Cannot create a font named \"{face_name}\"")]
    FontCreationError {
        face_name: String
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

pub struct Font {
    handle: HFONT,
}

impl Drop for Font {
    fn drop(&mut self) {
        _ = unsafe { DeleteObject(self.handle) };
    }
}

pub fn create_font(face_name: &str, size: i32) -> Result<Font, Error> {
    let handle = unsafe {
        CreateFontW(
            size,
            0,
            0,
            0,
            FW_NORMAL.0 as i32,
            0,
            0,
            0,
            DEFAULT_CHARSET.0 as u32,
            OUT_DEFAULT_PRECIS.0 as u32,
            CLIP_DEFAULT_PRECIS.0 as u32,
            DEFAULT_QUALITY.0 as u32,
            (DEFAULT_PITCH.0 | FF_DONTCARE.0) as u32,
            &HSTRING::from(face_name),
        )
    };
    if handle.is_invalid() {
        Err(Error::FontCreationError { face_name: face_name.to_owned() })
    } else {
        Ok(Font { handle })
    }
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
            _ = Rectangle(hdc, rect.left, rect.top, rect.right + 1, rect.bottom + 1);
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

    fn draw_polygon(&self, points: &Vec<POINT>, pen: &Pen, brush: &Brush) {
        unsafe {
            let old_pen = SelectObject(self.get_hdc(), pen.handle);
            let old_brush = SelectObject(self.get_hdc(), brush.handle);
            _ = Polygon(self.get_hdc(), points);
            SelectObject(self.get_hdc(), old_brush);
            SelectObject(self.get_hdc(), old_pen);
        }
    }

    fn draw_text(&self, text: &str, x: i32, y: i32, font: &Font, color: COLORREF) {
        unsafe {
            let old_font = SelectObject(self.get_hdc(), font.handle);
            let old_color = SetTextColor(self.get_hdc(), color);
            _ = TextOutW(self.get_hdc(), x, y, HSTRING::from(text).as_wide());
            SetTextColor(self.get_hdc(), old_color);
            SelectObject(self.get_hdc(), old_font);
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

pub fn point(x: i32, y: i32) -> POINT {
    POINT { x, y }
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
