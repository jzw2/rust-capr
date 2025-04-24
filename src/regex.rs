use rustfst::prelude::VectorFst;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Symbol;

use crate::trans::SoundWeight;

#[wasm_bindgen]
pub struct Regex(VectorFst<SoundWeight>);

#[wasm_bindgen]
impl Regex {
    pub fn concat() {
        unimplemented!();
    }
    pub fn disjoint() {
        unimplemented!();
    }
    pub fn kleen() {
        unimplemented!();
    }
}
