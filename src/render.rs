use windows::{
    core::{w, HSTRING},
    Foundation::Numerics::Matrix3x2,
    Win32::{
        Foundation::{GENERIC_READ, HWND, RECT},
        Graphics::{
            Direct2D::{
                Common::{
                    D2D1_COLOR_F, D2D1_FIGURE_BEGIN_FILLED, D2D1_FIGURE_END_CLOSED, D2D_POINT_2F,
                    D2D_RECT_F, D2D_SIZE_F, D2D_SIZE_U,
                },
                D2D1CreateFactory, ID2D1Bitmap, ID2D1BitmapRenderTarget, ID2D1Brush, ID2D1Factory,
                ID2D1GeometrySink, ID2D1HwndRenderTarget, ID2D1PathGeometry, ID2D1RenderTarget,
                ID2D1SolidColorBrush, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
                D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE, D2D1_DRAW_TEXT_OPTIONS_NONE,
                D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_HWND_RENDER_TARGET_PROPERTIES,
                D2D1_PRESENT_OPTIONS_NONE, D2D1_RENDER_TARGET_PROPERTIES,
            },
            DirectWrite::{
                DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
                DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT_REGULAR,
                DWRITE_MEASURING_MODE_NATURAL, DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
                DWRITE_TEXT_ALIGNMENT_LEADING,
            },
            Imaging::{
                CLSID_WICImagingFactory, GUID_WICPixelFormat32bppPBGRA, IWICImagingFactory,
                WICBitmapDitherTypeNone, WICBitmapPaletteTypeMedianCut,
                WICDecodeMetadataCacheOnLoad,
            },
        },
        System::Com::{CoCreateInstance, CoInitialize, CLSCTX_INPROC_SERVER},
        UI::WindowsAndMessaging::GetClientRect,
    },
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Internal(windows::core::Error),
    #[error("Cannot load a bitmap from file ({file_path}): {source}")]
    BitmapLoad {
        file_path: String,
        #[source]
        source: windows::core::Error,
    },
    #[error("Cannot create a font named \"{face_name}\"")]
    FontCreation {
        face_name: String,
        #[source]
        source: windows::core::Error,
    },
}

pub struct Context {
    d2d_factory: ID2D1Factory,
    primary_render_target: Option<RenderTarget>,

    dwrite_factory: IDWriteFactory,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        let d2d_factory = unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None) }
            .map_err(Error::Internal)?;

        let dwrite_factory =
            unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED) }.map_err(Error::Internal)?;

        Ok(Context {
            d2d_factory,
            primary_render_target: None,

            dwrite_factory,
        })
    }

    pub fn get_primary_render_target(&mut self, hwnd: HWND) -> Result<&RenderTarget, Error> {
        if let Some(ref t) = self.primary_render_target {
            return Ok(t);
        }

        let mut rect = RECT::default();
        _ = unsafe { GetClientRect(hwnd, &mut rect) };

        let size = D2D_SIZE_U {
            width: (rect.right - rect.left) as u32,
            height: (rect.bottom - rect.top) as u32,
        };

        let hwnd_render_target = unsafe {
            self.d2d_factory.CreateHwndRenderTarget(
                &D2D1_RENDER_TARGET_PROPERTIES::default(),
                &D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    hwnd,
                    pixelSize: size,
                    presentOptions: D2D1_PRESENT_OPTIONS_NONE,
                },
            )
        }
        .map_err(Error::Internal)?;

        let render_target = RenderTarget::WindowRenderTarget(hwnd_render_target);

        self.primary_render_target = Some(render_target);

        Ok(self.primary_render_target.as_ref().unwrap())
    }

    pub fn reset_primary_render_target(&mut self) {
        self.primary_render_target = None;
    }

    pub fn create_geometry<F: FnOnce(&GeometryBuilder)>(
        &self,
        creator: F,
    ) -> Result<Geometry, Error> {
        let geometry = unsafe { self.d2d_factory.CreatePathGeometry() }.map_err(Error::Internal)?;

        let sink = unsafe { geometry.Open() }.map_err(Error::Internal)?;
        creator(&GeometryBuilder(&sink));
        unsafe { sink.Close() }.map_err(Error::Internal)?;

        Ok(Geometry(geometry))
    }

    pub fn create_font(&self, face_name: &str, size: i32) -> Result<Font, Error> {
        let text_format = unsafe {
            self.dwrite_factory
                .CreateTextFormat(
                    &HSTRING::from(face_name),
                    None,
                    DWRITE_FONT_WEIGHT_REGULAR,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    size as f32,
                    w!("ja-jp"),
                )
                .and_then(|tf| {
                    tf.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING)?;
                    tf.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_NEAR)?;
                    Ok(tf)
                })
        }
        .map_err(|e| Error::FontCreation {
            face_name: face_name.to_owned(),
            source: e,
        })?;

        Ok(Font(text_format))
    }
}

pub enum RenderTarget {
    WindowRenderTarget(ID2D1HwndRenderTarget),
    BitmapRenderTarget(ID2D1BitmapRenderTarget),
}

impl RenderTarget {
    fn get_common(&self) -> &ID2D1RenderTarget {
        match self {
            Self::WindowRenderTarget(rt) => rt.into(),
            Self::BitmapRenderTarget(rt) => rt.into(),
        }
    }

    pub fn begin(&self) {
        unsafe { self.get_common().BeginDraw() };
    }

    pub fn end(&self) -> bool {
        unsafe { self.get_common().EndDraw(None, None) }.is_ok()
    }

    pub fn create_new_render_target(&self, width: u32, height: u32) -> Result<RenderTarget, Error> {
        let size = D2D_SIZE_F {
            width: width as f32,
            height: height as f32,
        };
        let new_rt = unsafe {
            self.get_common().CreateCompatibleRenderTarget(
                Some(&size),
                None,
                None,
                D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE,
            )
        }
        .map_err(Error::Internal)?;

        Ok(Self::BitmapRenderTarget(new_rt))
    }

    pub fn clear(&self, color: D2D1_COLOR_F) {
        unsafe {
            self.get_common().SetTransform(&Matrix3x2::identity());
            self.get_common().Clear(Some(&color));
        }
    }

    pub fn get_size(&self) -> (u32, u32) {
        let size = unsafe { self.get_common().GetSize() };
        (size.width as u32, size.height as u32)
    }

    pub fn get_bitmap(&self) -> Result<Bitmap, Error> {
        match self {
            RenderTarget::BitmapRenderTarget(rt) => {
                let bitmap = unsafe { rt.GetBitmap() }.map_err(Error::Internal)?;
                Ok(Bitmap(bitmap))
            }
            _ => panic!("Cannot get bitmap from HWND render target"),
        }
    }

    pub fn copy_from(&self, dest_rect: &D2D_RECT_F, src: &Bitmap, src_x: i32, src_y: i32) {
        unsafe {
            self.get_common().DrawBitmap(
                &src.0,
                Some(dest_rect),
                1.0,
                D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
                Some(&rect_wh(
                    src_x,
                    src_y,
                    (dest_rect.right - dest_rect.left) as i32,
                    (dest_rect.bottom - dest_rect.top) as i32,
                )),
            )
        }
    }

    pub fn create_solid_brush(&self, color: D2D1_COLOR_F) -> Result<Brush, Error> {
        let brush = unsafe { self.get_common().CreateSolidColorBrush(&color, None) }
            .map_err(Error::Internal)?;
        Ok(Brush::SolidColor(brush))
    }

    pub fn fill_rect(&self, rect: &D2D_RECT_F, brush: &Brush) {
        unsafe { self.get_common().FillRectangle(rect, brush.get_common()) };
    }

    pub fn draw_rect(&self, rect: &D2D_RECT_F, brush: &Brush) {
        unsafe {
            self.get_common()
                .DrawRectangle(rect, brush.get_common(), 1.0, None)
        };
    }

    pub fn draw_polygon(
        &self,
        geometry: &Geometry,
        x: i32,
        y: i32,
        pen: &Brush,
        brush: &Brush,
        angle: f32,
    ) {
        let rt = self.get_common();
        unsafe {
            let bounds = geometry.0.GetBounds(None).unwrap();
            rt.SetTransform(
                &(Matrix3x2::rotation(
                    angle,
                    (bounds.right - bounds.left) / 2.0,
                    (bounds.bottom - bounds.top) / 2.0,
                ) * Matrix3x2::translation(x as f32, y as f32)),
            );
            rt.DrawGeometry(&geometry.0, pen.get_common(), 1.0, None);
            rt.FillGeometry(&geometry.0, brush.get_common(), None);
            rt.SetTransform(&Matrix3x2::identity());
        }
    }

    pub fn draw_text(&self, text: &str, x: i32, y: i32, font: &Font, brush: &Brush) {
        unsafe {
            self.get_common().DrawText(
                HSTRING::from(text).as_wide(),
                &font.0,
                &rect(x, y, 10000, 10000),
                brush.get_common(),
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            )
        }
    }
}

pub struct Bitmap(ID2D1Bitmap);

pub struct ImageLoader {
    wic_factory: IWICImagingFactory,
}

impl ImageLoader {
    pub fn new() -> Result<Self, Error> {
        let result = unsafe { CoInitialize(None) };
        if result.is_err() {
            return Err(Error::Internal(windows::core::Error::from_hresult(result)));
        }
        let wic_factory: IWICImagingFactory =
            unsafe { CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER) }
                .map_err(Error::Internal)?;

        Ok(ImageLoader { wic_factory })
    }

    pub fn load_bitmap(&self, file_path: &str, rt: &RenderTarget) -> Result<Bitmap, Error> {
        let convert_error = |e: windows::core::Error| Error::BitmapLoad {
            file_path: file_path.to_owned(),
            source: e,
        };

        let decoder = unsafe {
            self.wic_factory.CreateDecoderFromFilename(
                &HSTRING::from(file_path),
                None,
                GENERIC_READ,
                WICDecodeMetadataCacheOnLoad,
            )
        }
        .map_err(convert_error)?;

        let frame = unsafe { decoder.GetFrame(0) }.map_err(convert_error)?;

        let format_converter =
            unsafe { self.wic_factory.CreateFormatConverter() }.map_err(convert_error)?;

        unsafe {
            format_converter.Initialize(
                &frame,
                &GUID_WICPixelFormat32bppPBGRA,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeMedianCut,
            )
        }
        .map_err(convert_error)?;

        let bitmap = unsafe {
            rt.get_common()
                .CreateBitmapFromWicBitmap(&format_converter, None)
        }
        .map_err(convert_error)?;

        Ok(Bitmap(bitmap))
    }
}

pub enum Brush {
    SolidColor(ID2D1SolidColorBrush),
}

impl Brush {
    fn get_common(&self) -> &ID2D1Brush {
        match self {
            Self::SolidColor(brush) => brush.into(),
        }
    }
}

pub struct Geometry(ID2D1PathGeometry);

pub struct GeometryBuilder<'a>(&'a ID2D1GeometrySink);

impl<'a> GeometryBuilder<'a> {
    pub fn begin_figure(&self, point: &D2D_POINT_2F) {
        unsafe { self.0.BeginFigure(*point, D2D1_FIGURE_BEGIN_FILLED) };
    }

    pub fn end_figure(&self) {
        unsafe { self.0.EndFigure(D2D1_FIGURE_END_CLOSED) };
    }

    pub fn add_line(&self, point: &D2D_POINT_2F) {
        unsafe { self.0.AddLine(*point) };
    }
}

pub struct Font(IDWriteTextFormat);

pub fn point(x: i32, y: i32) -> D2D_POINT_2F {
    D2D_POINT_2F {
        x: x as f32,
        y: y as f32,
    }
}

pub fn rect(left: i32, top: i32, right: i32, bottom: i32) -> D2D_RECT_F {
    D2D_RECT_F {
        left: left as f32,
        top: top as f32,
        right: right as f32,
        bottom: bottom as f32,
    }
}

pub fn rect_wh(left: i32, top: i32, width: i32, height: i32) -> D2D_RECT_F {
    D2D_RECT_F {
        left: left as f32,
        top: top as f32,
        right: (left + width) as f32,
        bottom: (top + height) as f32,
    }
}

pub fn color_rgb(r: u8, g: u8, b: u8) -> D2D1_COLOR_F {
    D2D1_COLOR_F {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}
