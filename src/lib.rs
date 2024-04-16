use mindus::{data::DataRead, Map, Renderable, Serializable};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// returns PNG
pub fn render(v: &[u8]) -> Vec<u8> {
    let mut r = DataRead::new(&v);
    Map::deserialize(&mut r)
        .map_err(|_| ())
        .and_then(|m| {
            let i = m.render();
            let mut v = Vec::with_capacity(pngenc::size(pngenc::RGB, (i.width(), i.height())));
            pngenc::ode(pngenc::RGB, (i.width(), i.height()), i.bytes(), &mut v)
                .map_err(|_| ())
                .map(|()| v)
        })
        .unwrap_or_else(|_| include_bytes!("../fail.png").to_vec())
}
