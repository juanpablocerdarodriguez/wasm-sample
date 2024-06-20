extern crate wasm_bindgen;
extern crate image;
extern crate qrcode;
extern crate serde_json;
extern crate base64;

use wasm_bindgen::prelude::*;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::Luma;
use qrcode::QrCode;
use std::io::Cursor;

// Function to create a QR code from a given text and return it as a base64 encoded image
#[wasm_bindgen]
pub fn create_qr_code(text: &str) -> String {
    let code = QrCode::new(text).unwrap(); // Use the corrected identifier
    let image = code.render::<Luma<u8>>().build();
    
    let mut buffer = Cursor::new(Vec::new());
    image::DynamicImage::ImageLuma8(image).write_to(&mut buffer, image::ImageFormat::Png).unwrap();
    let buffer = buffer.into_inner();
    
    let encoded_image = STANDARD.encode(&buffer);
    encoded_image
}
