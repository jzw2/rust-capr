extern crate console_error_panic_hook;

use ipa_translate::xsampa_to_ipa;
use regex::RegexFst;
use tables::{ipa, xsampa_ascii};
use wasm_bindgen::prelude::*;

mod cross_product;
mod negate;
pub mod regex;
pub mod sound_law;
pub mod tables;
mod trans;

use rustfst::prelude::VectorFst;
use sound_law::SoundLaw;
use trans::SoundWeight;

#[wasm_bindgen]
pub fn create_law(left: &str, right: &str, from: &str, to: &str) -> SoundLaw {
    // let latin = lower_case_latin();
    let xsampa = xsampa_ascii();

    // SoundLaw::new(from, to, left, right, &latin)
    SoundLaw::new(from, to, left, right, &xsampa)
}

#[wasm_bindgen]
pub fn create_with_arbitrary_regex_ipa(
    left: &RegexFst,
    right: &RegexFst,
    from: &RegexFst,
    to: &RegexFst,
) -> SoundLaw {
    SoundLaw::create_with_arbitrary_regex(left, right, from, to, &ipa())
}

#[wasm_bindgen]
pub fn create_law_ipa(left: &str, right: &str, from: &str, to: &str) -> SoundLaw {
    let table = ipa();

    SoundLaw::new(
        &xsampa_to_ipa(from),
        &xsampa_to_ipa(to),
        &xsampa_to_ipa(left),
        &xsampa_to_ipa(right),
        &table,
    )
}

#[wasm_bindgen]
pub struct Disjunction(VectorFst<SoundWeight>);
//eventually get rid of this for the regex

#[wasm_bindgen]
impl Disjunction {
    pub fn new(strings: Vec<String>) -> Disjunction {
        let xsampa = xsampa_ascii();
        let fst = SoundLaw::disjunction_vec_fst(&strings, &xsampa);
        Disjunction(fst)
    }
    // TODO change to maybe just pass in the table or somehting like that
    pub fn new_ipa(strings: Vec<String>) -> Disjunction {
        let table = ipa();
        let fst = SoundLaw::disjunction_vec_fst(&strings, &table);
        Disjunction(fst)
    }
}

#[wasm_bindgen]
pub fn create_with_disjunctions_ipa(
    left: Disjunction,
    right: Disjunction,
    from: &str,
    to: &str,
) -> SoundLaw {
    // let latin = lower_case_latin();
    let table = ipa();

    // SoundLaw::new(from, to, left, right, &latin)
    // assumes left and right were created using ipa table
    SoundLaw::new_with_vec_context(
        &xsampa_to_ipa(from),
        &xsampa_to_ipa(to),
        left.0,
        right.0,
        &table,
    )
}

//old functoin
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
    SoundLaw::new(from, to, left, right, &xsampa)
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
