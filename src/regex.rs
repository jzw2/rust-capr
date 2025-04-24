use rustfst::prelude::{concat::concat, union::union, VectorFst};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::Symbol;

use crate::trans::SoundWeight;

#[wasm_bindgen]
pub struct Regex(VectorFst<SoundWeight>);

#[wasm_bindgen]
impl Regex {
    pub fn concat(&mut self, other: &Regex) {
        let _ = concat(&mut self.0, &other.0);
    }
    pub fn disjoint(&mut self, other: &Regex) {
        // I don't know why I chose a different name but whatever
        let _ = union(&mut self.0, &other.0);
    }
    //implement the rest later
    pub fn kleen(&mut self) {
        unimplemented!();
    }
}
