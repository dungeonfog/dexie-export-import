use js_sys::Promise;
use web_sys::Blob;
use wasm_bindgen::prelude::*;
use dexie::Dexie;

#[wasm_bindgen(module = "dexie-export-import")]
extern "C" {
    #[wasm_bindgen(js_name=importDB)]
    pub fn import_db(blob: &Blob) -> Promise;

    #[wasm_bindgen(js_name=importDB)]
    pub fn import_db_with_options(blob: &Blob, options: JsValue) -> Promise;
    
    #[wasm_bindgen(js_name=exportDB)]
    pub fn export_db(db: &Dexie) -> Promise;
    
    #[wasm_bindgen(js_name=exportDB)]
    pub fn export_db_with_options(db: &Dexie, options: JsValue) -> Promise;

    #[wasm_bindgen(js_name=importInto)]
    pub fn import_into(db: &Dexie, blob: &Blob) -> Promise;

    #[wasm_bindgen(js_name=importInto)]
    pub fn import_into_with_options(db: &Dexie, blob: &Blob, options: JsValue) -> Promise;
}
