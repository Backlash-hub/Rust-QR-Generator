use qrcode::QrCode;
use image::Luma;

fn main() {

    let code = QrCode::new(b"01234567").unwrap();

    let image = code.render::<Luma<u8>>().build();

    image.save("qrcode.png").unwrap();
}
