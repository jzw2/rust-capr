use rustfst::{
    prelude::{concat::concat, union::union, VectorFst},
    SymbolTable,
};
use wasm_bindgen::prelude::*;

use crate::{
    cross_product::cross_product,
    sound_law::SoundLaw,
    tables::{ipa, xsampa_ascii},
    trans::{SoundFst, SoundWeight},
};

//change this to not a new type that also contains information for how it was created
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

    pub fn to_string(&self) -> String {
        // I don't know why I chose a different name but whatever
        "implement this later".into()
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

impl Regex {
    pub fn to_sound_fst(&self) -> SoundFst {
        todo!()
    }
    pub fn regex_cross_product(a: &Regex, b: &Regex, table: &SymbolTable) -> SoundFst {
        SoundFst(cross_product(&a.0, &b.0, table))
    }
}
