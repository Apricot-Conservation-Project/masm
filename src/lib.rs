#![feature(alloc_error_hook)]
use mindus::{data::DataRead, Map, Renderable, Serializable};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// returns PNG
pub fn render(v: Vec<u8>) -> Vec<u8> {
    macro_rules! unwrap {
        ($e:expr) => {
            match $e {
                Ok(v) => v,
                Err(_) => std::process::abort(),
            }
        };
    }
    let mut r = DataRead::new(&v);
    let m = unwrap!(Map::deserialize(&mut r));
    let i = m.render();
    let mut v = Vec::with_capacity(2usize.pow(27));
    let mut enc = png::Encoder::new(&mut v, i.width(), i.height());
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    enc.set_source_chromaticities(png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    ));
    let mut writer = unwrap!(enc.write_header());
    _ = writer.write_image_data(i.buffer());
    _ = writer.finish();
    v
}
