use rustfst::prelude::{concat::concat, union::union, VectorFst};
use wasm_bindgen::prelude::*;

use crate::{
    sound_law::SoundLaw,
    tables::{ipa, xsampa_ascii},
    trans::SoundWeight,
};

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

    pub fn new_xsampa_disjunction(strings: Vec<String>) -> Regex {
        let xsampa = xsampa_ascii();
        let fst = SoundLaw::disjunction_vec_fst(&strings, &xsampa);
        Regex(fst)
    }
    // TODO change to maybe just pass in the table or somehting like that
    pub fn new_ipa_disjunction(strings: Vec<String>) -> Regex {
        let table = ipa();
        let fst = SoundLaw::disjunction_vec_fst(&strings, &table);
        Regex(fst)
    }
}
