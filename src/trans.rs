use std::sync::Arc;

use actix_web::Error;
use rustfst::{algorithms::concat::concat, fst_impls::VectorFst, fst_traits::{Fst, MutableFst}, semirings::ProbabilityWeight, utils::{acceptor, transducer}, Label, Semiring, SymbolTable, Tr};
use rustfst::algorithms::compose::compose;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SoundLaw {
    from: String,
    to: String,
    left_context: String,
    right_context: String,
}

#[derive(Debug)]
struct SoundLawLabels {
    from: Vec<Label>,
    to: Vec<Label>,
    left_context: Vec<Label>,
    right_context: Vec<Label>,
}



fn get_labels_from_str(s: &str, table: Arc<SymbolTable>) -> Option<Vec<Label>> {
    s.chars().map(|x| table.get_label(x.to_string())).collect()
}

// given t that actually does the replacement, creates a transuducer that makes sure
// all substrings are repalced
fn replace(t: VectorFst<ProbabilityWeight>, optional: bool, alphabet: &SymbolTable) -> Option<VectorFst<ProbabilityWeight>> {
    todo!()
}

// calls replace, but first ignores brackets and makes sure replacement occures only in brackets
fn replace_transducer(t: VectorFst<ProbabilityWeight>, left_marker: String, right_marker: &String, alphabet: SymbolTable) -> Option<VectorFst<ProbabilityWeight>> {
    todo!()
}

// given t that does replacement, m1, m2 are the left and right context
fn function_name(t: VectorFst<ProbabilityWeight>, left_marker: String, right_marker: &String, alphabet: SymbolTable) -> Option<VectorFst<ProbabilityWeight>> {
    todo!()
}




impl SoundLaw {
    fn to_labels(&self, table: Arc<SymbolTable>) -> Option<SoundLawLabels> {
        let left = get_labels_from_str(&self.left_context, Arc::clone(&table))?;
        let right = get_labels_from_str(&self.right_context, Arc::clone(&table))?;
        let from = get_labels_from_str(&self.from, Arc::clone(&table))?;
        let to = get_labels_from_str(&self.to, Arc::clone(&table))?;

        Some(SoundLawLabels { from, to, left_context: left, right_context: right })

    }

    fn to_fst(&self, table: Arc<SymbolTable>) -> VectorFst<ProbabilityWeight> {
        let SoundLawLabels { from, to, left_context, right_context } = self.to_labels(Arc::clone(&table)).unwrap();
        let mut left_context_fst: VectorFst<_> = acceptor(&left_context, ProbabilityWeight::one());
        let right_context_fst: VectorFst<_> = acceptor(&right_context, ProbabilityWeight::one());

        let transform: VectorFst<_> = transducer(&from, &to, ProbabilityWeight::one());

        concat(&mut left_context_fst, &transform).expect("concat failed");
        concat(&mut left_context_fst, &right_context_fst).expect("concat failed");

        left_context_fst.set_input_symbols(Arc::clone(&table));
        left_context_fst.set_output_symbols(Arc::clone(&table));

        left_context_fst
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

    #[test]
    fn test_no_context_soundlaw()  {
        let table = symt![ "a", "b", "c"];
        let table = Arc::new(table);
        let law = SoundLaw {
            from: "a".to_string(),
            to: "b".to_string(),
            left_context: "".to_string(),
            right_context: "".to_string()
        };
        let fst = law.to_fst(table);
        dbg!(&fst);
        let paths: Vec<_> = fst.string_paths_iter().unwrap().collect();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].istring().unwrap(), "a");
        assert_eq!(paths[0].ostring().unwrap(), "b");
    }

    #[test]
    fn test_some_context_soundlaw()  {
        let table = symt![ "a", "b", "c"];
        let table = Arc::new(table);
        let law = SoundLaw {
            from: "a".to_string(),
            to: "b".to_string(),
            left_context: "c".to_string(),
            right_context: "c".to_string()
        };
        let fst = law.to_fst(table);
        dbg!(&fst);
        let paths: Vec<_> = fst.string_paths_iter().unwrap().collect();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].istring().unwrap(), "c a c");
        assert_eq!(paths[0].ostring().unwrap(), "c b c");
    }
}
