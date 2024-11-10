extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;

mod negate;
mod trans;

use rustfst::SymbolTable;
use trans::SoundLaw;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
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
    let sound = SoundLaw::new(from, to, left, right);
    let alphabet: Vec<_> = "abcedfghijklmnopqrstuvwxyz".split("").collect();
    let mut table = SymbolTable::new();
    table.add_symbols(alphabet);

    let mut fst = sound.to_fst(&table);
    fst.invert();
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
    let sound = SoundLaw::new(from, to, left, right);
    let alphabet: Vec<_> = "abcedfghijklmnopqrstuvwxyz".split("").collect();
    let mut table = SymbolTable::new();
    table.add_symbols(alphabet);

    let fst = sound.to_fst(&table);
    fst.transduce_text(&table, input)
}

#[cfg(test)]
mod tests {
    use crate::trans::SoundLaw;
    use crate::{transduce_context, SymbolTable};
    #[test]
    fn sound_law() {
        let left = "a";
        let right = "c";
        let from = "b";
        let to = "x";
        let input = "abc";

        let output = transduce_context(left, right, from, to, input);

        assert_eq!(&output[0], "a x c");
    }
}
