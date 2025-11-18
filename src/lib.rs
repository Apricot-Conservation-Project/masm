use mindus::{Map, Renderable, Schematic, Serializable, data::DataRead};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn tags_(t: &[u8]) -> Vec<String> {
    let mut t = DataRead::new(&t);
    Schematic::deserialize(&mut t)
        .map_err(|_| ())
        .map(|x| {
            x.tags
                .iter()
                .map(|(a, b)| {
                    let mut buf = String::default();
                    buf.push_str(a);
                    buf.push('⬟');
                    buf.push_str(b);
                    buf
                })
                .collect()
        })
        .unwrap_or_else(|_| loop {})
}

fn render(t: impl Renderable) -> Result<Vec<u8>, ()> {
    let i = t.render();
    let mut v = Vec::with_capacity(pngenc::size(pngenc::RGB, (i.width(), i.height())));
    pngenc::ode(pngenc::RGB, (i.width(), i.height()), i.bytes(), &mut v)
        .map_err(|_| ())
        .map(|()| v)
}

fn deser_and_render<T: Serializable + Renderable>(v: &[u8]) -> Vec<u8> {
    let mut r = DataRead::new(&v);
    T::deserialize(&mut r)
        .map_err(|_| ())
        .and_then(render)
        .unwrap_or_else(|_| include_bytes!("../fail.png").to_vec())
}

#[wasm_bindgen]
/// returns PNG
pub fn render_map_(v: &[u8]) -> Vec<u8> {
    deser_and_render::<Map>(v)
}

#[wasm_bindgen]
pub fn render_schem_(v: &[u8]) -> Vec<u8> {
    deser_and_render::<Schematic>(v)
}
