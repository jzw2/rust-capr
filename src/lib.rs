extern crate console_error_panic_hook;
use std::{borrow::Borrow, future::Future};

use tables::xsampa_ascii;
use trans::SoundFst;
use wasm_bindgen::prelude::*;

mod negate;
mod sound_law;
mod tables;
mod trans;

use sound_law::SoundLaw;
use trans::SoundWeight;
use rustfst::prelude::VectorFst;

#[wasm_bindgen]
pub fn create_law(left: &str, right: &str, from: &str, to: &str) -> SoundLaw {
    // let latin = lower_case_latin();
    let xsampa = xsampa_ascii();

    // SoundLaw::new(from, to, left, right, &latin)
    SoundLaw::new(from, to, left, right, &xsampa)
}

#[wasm_bindgen]
pub struct Disjunction(VectorFst<SoundWeight>);

#[wasm_bindgen]
impl Disjunction {
    pub fn new(strings: Vec<String>) -> Disjunction {
        let xsampa = xsampa_ascii();
        let fst = SoundLaw::disjunction_vec_fst(&strings, &xsampa);
        Disjunction(fst)
    }
}
#[wasm_bindgen]
pub fn create_with_disjunctions(
    left: Disjunction,
    right: Disjunction,
    from: &str,
    to: &str,
) -> SoundLaw {
    // let latin = lower_case_latin();
    let xsampa = xsampa_ascii();

    // SoundLaw::new(from, to, left, right, &latin)
    SoundLaw::new_with_vec_context(from, to, left.0, right.0, &xsampa)
}

#[wasm_bindgen]
pub async fn create_law_async(left: &str, right: &str, from: &str, to: &str) -> SoundLaw {
    let xsampa = xsampa_ascii();

    // SoundLaw::new(from, to, left, right, &latin)
    return SoundLaw::new(from, to, left, right, &xsampa);
}

#[cfg(test)]
mod tests {
    pub fn transduce_context_invert(
        left: &str,
        right: &str,
        from: &str,
        to: &str,
        input: &str,
    ) -> Vec<String> {
        console_error_panic_hook::set_once();
        let law = create_law(left, right, from, to);

        law.transduce_text_backwards(input)
    }

    pub fn transduce_context(
        left: &str,
        right: &str,
        from: &str,
        to: &str,
        input: &str,
    ) -> Vec<String> {
        console_error_panic_hook::set_once();
        let law = create_law(left, right, from, to);
        law.transduce_text(input)
    }

    use crate::*;
    #[test]
    fn sound_law_invert() {
        let left = "x";
        let right = "x";
        let from = "xx";
        let to = "x";
        let input = "xxx";

        let output = transduce_context_invert(left, right, from, to, input);

        assert_eq!(&output[0], "x x x");
        assert_eq!(&output[1], "x x x x");
    }
    #[test]
    fn sound_law_non_invert() {
        let left = "a";
        let right = "c";
        let from = "b";
        let to = "x";
        let input = "abc";

        let output = transduce_context(left, right, from, to, input);
        dbg!(&output);

        assert_eq!(&output[0], "a x c");
    }

    #[test]
    fn symbol_boundry() {
        let left = "a";
        let right = "c";
        let from = "b";
        let to = "x";
        let input = "#abc#";

        let output = transduce_context(left, right, from, to, input);
        assert_eq!(&output[0], "# a x c #");
    }
}
