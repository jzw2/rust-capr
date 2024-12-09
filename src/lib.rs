extern crate console_error_panic_hook;
use tables::lower_case_latin;
use wasm_bindgen::prelude::*;

mod negate;
mod sound_law;
mod tables;
mod trans;

use sound_law::SoundLaw;

#[wasm_bindgen]
pub fn create_law(left: &str, right: &str, from: &str, to: &str) -> SoundLaw {
    let latin = lower_case_latin();

    SoundLaw::new(from, to, left, right, &latin)
}

#[wasm_bindgen]
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

#[wasm_bindgen]
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

#[cfg(test)]
mod tests {

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
}
