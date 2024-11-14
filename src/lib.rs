extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;

mod negate;
mod trans;

use rustfst::SymbolTable;
use trans::{SoundFst, SoundLaw};

pub fn create_law(left: &str, right: &str, from: &str, to: &str) -> SoundLaw {
    let alphabet: Vec<_> = "abcedfghijklmnopqrstuvwxyz"
        .chars()
        .map(|x| x.to_string())
        .collect();
    // let alphabet: Vec<_> = "abcx".split("").collect();
    let mut table = SymbolTable::new();
    table.add_symbols(alphabet);

    let sound = SoundLaw::new(from, to, left, right, &table);
    sound
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
    let mut fst = law.get_fst().clone();

    fst.invert();
    fst.df("inverted");
    let table = law.get_table();
    fst.transduce_text(&table, input)
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
    let fst = law.get_fst().clone();
    fst.df("non_invert");

    let table = law.get_table();
    fst.transduce_text(&table, input)
}

#[cfg(test)]
mod tests {
    use crate::trans::SoundLaw;
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
