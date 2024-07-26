use std::sync::Arc;

use actix_web::Error;
use rustfst::{fst_impls::VectorFst, fst_traits::{Fst, MutableFst}, semirings::ProbabilityWeight, utils::acceptor, Label, Semiring, SymbolTable, Tr};
use rustfst::algorithms::compose::compose;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SoundLaw {
    from: String,
    to: String,
    left_context: String,
    right_context: String,
}

fn get_labels_from_str(s: &str, table: Arc<SymbolTable>) -> Option<Vec<Label>> {
    s.chars().map(|x| table.get_label(x.to_string())).collect()
}


impl SoundLaw {
    fn to_fst(&self, table: Arc<SymbolTable>) -> VectorFst<ProbabilityWeight> {
        let left_context_fst: VectorFst<_> = acceptor( &get_labels_from_str(&self.left_context, table).unwrap(), ProbabilityWeight::one());




        todo!()
    }

}


fn sound_laws_to_fst(laws: &[SoundLaw], table: Arc<SymbolTable>) ->  VectorFst<ProbabilityWeight> {
    todo!()
}



pub fn transduce_text(laws: Vec<Vec<String>>, text: String) -> String {

    let mut fst = VectorFst::<ProbabilityWeight>::new();
    let mut symbol_table = SymbolTable::new();
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



    paths[0].ostring().expect("Error getting output string")
}


#[cfg(test)]
mod tests {
    use rustfst::symt;

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
    #[ignore]
    fn test_no_duplicate() {
        let law = vec![vec!["h".into(), "q".into()],
        ];
        let transduced = transduce_text(law, String::from("hi"));
        assert_eq!(transduced, "q i");
    }


    #[test]
    #[ignore]
    fn test_no_change() {
        let law = vec![vec!["a".into(), "b".into()],
        ];
        let transduced = transduce_text(law, String::from("hi"));
        assert_eq!(transduced, "h i");
    }

    #[test]
    fn test_labels_from_string() {
        let table = symt![ "a", "b", "c"];

        let transduced = get_labels_from_str("abba", Arc::new(table));
        assert_eq!(transduced, Some(vec![1, 2, 2, 1]));
    }
}
