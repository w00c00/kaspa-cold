use eframe::egui;

pub fn app_icon() -> egui::IconData {
    let decoder = png::Decoder::new(std::io::Cursor::new(include_bytes!(
        "../assets/icon-256.png"
    )));
    let mut reader = decoder
        .read_info()
        .expect("Embedded app icon must be valid PNG");
    let mut pixels = vec![0; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut pixels)
        .expect("Embedded app icon must decode");
    assert_eq!(info.color_type, png::ColorType::Rgba);
    pixels.truncate(info.buffer_size());
    egui::IconData {
        rgba: pixels,
        width: info.width,
        height: info.height,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn icon_is_valid_rgba() {
        let icon = super::app_icon();
        assert_eq!((icon.width, icon.height), (256, 256));
        assert_eq!(icon.rgba.len(), 256 * 256 * 4);
    }
}
