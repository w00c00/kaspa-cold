use qrcode::{Color, QrCode};

pub struct AddressQr {
    pub size: usize,
    pub pixels: Vec<u8>,
}
pub fn address_qr(address: &str) -> Result<AddressQr, &'static str> {
    if !(address.starts_with("kaspa:") || address.starts_with("kaspatest:")) {
        return Err("Not a public Kaspa address");
    }
    let _: kaspa_addresses::Address = address.try_into().map_err(|_| "Invalid address checksum")?;
    let qr = QrCode::new(address.as_bytes()).map_err(|_| "QR encoding failed")?;
    let scale = 6;
    let border = 4;
    let size = (qr.width() + border * 2) * scale;
    let mut pixels = vec![255; size * size];
    for y in 0..qr.width() {
        for x in 0..qr.width() {
            if qr[(x, y)] == Color::Dark {
                for dy in 0..scale {
                    for dx in 0..scale {
                        pixels[((y + border) * scale + dy) * size + (x + border) * scale + dx] = 0;
                    }
                }
            }
        }
    }
    Ok(AddressQr { size, pixels })
}
pub fn png_bytes(qr: &AddressQr) -> Result<Vec<u8>, &'static str> {
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, qr.size as u32, qr.size as u32);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|_| "PNG encoding failed")?;
        writer
            .write_image_data(&qr.pixels)
            .map_err(|_| "PNG encoding failed")?;
    }
    Ok(bytes)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_secrets() {
        assert!(address_qr("abandon abandon").is_err());
    }
    #[test]
    fn png_has_quiet_zone() {
        let q = address_qr("kaspa:qz7ulu4c25dh7fzec9zjyrmlhnkzrg4wmf89q7gzr3gfrsj3uz6xjellj43pf")
            .unwrap();
        assert!(q.pixels[..q.size * 24].iter().all(|p| *p == 255));
        assert!(png_bytes(&q).unwrap().starts_with(b"\x89PNG\r\n\x1a\n"));
    }
}
