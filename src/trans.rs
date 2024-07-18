
use std::sync::Arc;

use rustfst::{algorithms::compose, fst_impls::VectorFst, fst_traits::{Fst, MutableFst}, semirings::ProbabilityWeight, utils::acceptor, Semiring, SymbolTable, Tr};
use rustfst::algorithms::compose::compose;
pub fn transduce_text(laws: Vec<Vec<String>>, text: String) -> String {

    let mut fst = VectorFst::<ProbabilityWeight>::new();
    let mut symbol_table = (SymbolTable::new());
    let state = fst.add_state();
    fst.set_start(state).expect("Failed to set start state");
    for mut law in laws {
        let to = law.pop().expect("Not enough argumenets");
        let from = law.pop().expect("Not enouh argments");
        assert!(law.is_empty());
        let to_label = symbol_table.add_symbol(to);
        let from_label = symbol_table.add_symbol(from);


        let _ = fst.add_tr(state, Tr::new(from_label, to_label, 1.0, state));

    }
    let _ = fst.set_final(state, 1.0);

    let chars: Vec<_> = text.chars().map(String::from).collect();

    for c in chars.iter() {
        symbol_table.add_symbol(c);
    }

    let symbol_table = Arc::new(symbol_table);

    fst.set_input_symbols(Arc::clone(&symbol_table));
    fst.set_output_symbols(Arc::clone(&symbol_table));
    let labels: Vec<_> = chars.iter().map(|x| symbol_table.get_label(x).unwrap()).collect();


    let acceptor: VectorFst<_> = acceptor(&labels, ProbabilityWeight::one());
    dbg!(&labels);
    dbg!(&chars);
    dbg!(&fst);
    dbg!(&acceptor);

    let mut composed: VectorFst<_> = compose(acceptor, fst).expect("Error in composition");
    composed.set_input_symbols(Arc::clone(&symbol_table));
    composed.set_output_symbols(Arc::clone(&symbol_table));

    dbg!(&composed);
    let paths : Vec<_> = composed.string_paths_iter().unwrap().collect();



    return paths[0].ostring().expect("Error getting output string");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let law = vec![vec!["h".into(), "q".into()],
                       vec!["i".into(), "i".into()]
        ];
        let transduced = transduce_text(law, String::from("hi"));
        assert_eq!(transduced, "q i");
    }


    #[test]
    fn test_no_duplicate() {
        let law = vec![vec!["h".into(), "q".into()],
        ];
        let transduced = transduce_text(law, String::from("hi"));
        assert_eq!(transduced, "q i");
    }


    #[test]
    fn test_no_change() {
        let law = vec![vec!["a".into(), "b".into()],
        ];
        let transduced = transduce_text(law, String::from("hi"));
        assert_eq!(transduced, "h i");
    }
}
