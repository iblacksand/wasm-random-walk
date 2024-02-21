mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-random-walk!");
}

#[wasm_bindgen]
pub fn get_neighborhood(network: JsValue, genes: JsValue, n: JsValue) -> JsValue {
    let network: Vec<String> = serde_wasm_bindgen::from_value(network).unwrap();
    let genes: Vec<String> = serde_wasm_bindgen::from_value(genes).unwrap();
    let n: usize = serde_wasm_bindgen::from_value(n).unwrap();
    let neighborhood = utils::get_all_neighborhood(network, genes, n);
    serde_wasm_bindgen::to_value(&neighborhood).unwrap()
}
