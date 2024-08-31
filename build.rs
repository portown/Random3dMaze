use std::io;

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon_with_id("res/icon.ico", "IDI_MAIN");
        res.compile()?;
    }

    Ok(())
}
