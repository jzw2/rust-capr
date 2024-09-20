use std::sync::Arc;

use rustfst::algorithms::compose::compose;
use rustfst::algorithms::rm_epsilon::*;
use rustfst::fst;
use rustfst::prelude::closure::{closure, ClosureType};
use rustfst::prelude::determinize::determinize;
use rustfst::prelude::union::union;
use rustfst::prelude::{tr_sort, ILabelCompare, OLabelCompare};
use rustfst::{
    algorithms::{concat::concat, project},
    fst_impls::VectorFst,
    fst_traits::{Fst, MutableFst},
    semirings::ProbabilityWeight,
    utils::{acceptor, epsilon_machine, transducer},
    Label, Semiring, SymbolTable, Tr,
};
use serde::{Deserialize, Serialize};

use crate::negate::{negate, negate_with_symbol_table};

pub type SoundFst = VectorFst<ProbabilityWeight>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SoundLaw {
    from: String,
    to: String,
    left_context: String,
    right_context: String,
}

impl SoundLaw {
    pub fn new(from: &str, to: &str, left_context: &str, right_context: &str) -> SoundLaw {
        SoundLaw {
            from: from.to_string(), to: to.to_string(), left_context: left_context.into(),right_context: right_context.into()
        }
    }
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

fn any_star(st: &SymbolTable) -> SoundFst {
    let mut fst: SoundFst = epsilon_machine().unwrap();
    for label in st.labels() {
        let _ = fst.add_tr(0, Tr::new(label, label, ProbabilityWeight::one(), 0));
    }
    fst
}


//might be unneeded
/* fn subtract(fst1: &SoundFst, fst2: &SoundFst) -> SoundFst {
    // mostly translated from hfst's version
    // in TroplicalWeightTransducer.cc
    let mut new_fst1 = fst1.clone();
    rm_epsilon(&mut new_fst1).unwrap();
    let mut new_fst2 = fst2.clone();
    rm_epsilon(&mut new_fst2).unwrap();

    tr_sort(&mut new_fst1, OLabelCompare {}); // weird design syntax
    tr_sort(&mut new_fst2, ILabelCompare {});

    let fst2_det: SoundFst = determinize(&new_fst2).unwrap();

    todo!()
} */

// given t that actually does the replacement, creates a transuducer that makes sure
// all substrings are repalced
fn replace(
    t: SoundFst,
    optional: bool,
    alphabet: &SymbolTable,
) -> VectorFst<ProbabilityWeight> {
    let mut projection = t.clone();
    project(
        &mut projection,
        rustfst::algorithms::ProjectType::ProjectInput,
    ); // should be output in the lower level projection
    let star = any_star(alphabet);

    let mut tc = star.clone();

    concat(&mut tc, &t).unwrap();
    concat(&mut tc, &star).unwrap();

    let tc_neg = negate(&tc, &alphabet.labels().collect::<Vec<_>>());

    let mut retval = tc_neg.clone();
    concat(&mut retval, &t).unwrap();
    closure(&mut retval, ClosureType::ClosureStar);
    concat(&mut retval, &tc_neg).unwrap();


    if optional {
        union(&mut retval, &star).unwrap();
    }

retval
}

// calls replace, but first ignores brackets and makes sure replacement occures only in brackets
fn replace_transducer(
    t: SoundFst,
    left_marker: &str,
    right_marker: &str,
    alphabet: SymbolTable,
) -> Option<VectorFst<ProbabilityWeight>> {
    todo!()

}

// allows s to be inputted anywhere inside the fst
fn insert_freely(fst: &mut SoundFst, s: &str) {
    todo!();
}

// given t that does replacement, with contexyts
fn replace_context(
    t: &SoundFst,
    left_context: &str,
    right_context: &str,
    alphabet: &SymbolTable,
) -> Option<VectorFst<ProbabilityWeight>> {

    // copied from hfst, ideally I'll refactor it so that it actually makes sense
    let mut t_copy = t.clone();
    

    insert_freely(&mut t_copy, left_context);
    insert_freely(&mut t_copy, right_context);

    let pi_star = any_star(alphabet);
    let mut arg1 = any_star(alphabet);
    concat(&mut arg1, &t_copy).unwrap();

    let mut new_table = alphabet.clone(); // I think I might have to this above? I don't know
    // assuming that it is not in the alphabet
    let m1 = new_table.add_symbol(left_context);
    let m2 = new_table.add_symbol(right_context);
    let m1_tr: SoundFst = fst![m1];
    let mut tmp = pi_star.clone();
    concat(&mut tmp, &m1_tr).unwrap();
    let arg2 = negate_with_symbol_table(&tmp, alphabet);

    let ct: SoundFst = compose(arg1, arg2).unwrap();
    let mut mt: SoundFst = fst![m2];
    closure(&mut mt, ClosureType::ClosureStar);
    concat(&mut mt, &m1_tr).unwrap();
    concat(&mut mt, &pi_star).unwrap();

    // iff statement
    let tmp2 = negate_with_symbol_table(&mt, alphabet);
    let mut ct_neg_mt = ct.clone();
    concat(&mut ct_neg_mt, &tmp2).unwrap();


    let mut neg_ct_mt = negate_with_symbol_table(&ct, alphabet);
    concat(&mut neg_ct_mt, &mt).unwrap();


    let mut disj = neg_ct_mt;
    union(&mut disj, &ct_neg_mt).unwrap();
    let retval = negate_with_symbol_table(&disj, alphabet);
    
    // they optimize it, don't know what the equivalent is
    Some(retval)
}

impl SoundLaw {

    //might be unneeded if I want to refactor it completely with just labels, vs passing the string along always
    fn to_labels(&self, table: Arc<SymbolTable>) -> Option<SoundLawLabels> {
        let left = get_labels_from_str(&self.left_context, Arc::clone(&table))?;
        let right = get_labels_from_str(&self.right_context, Arc::clone(&table))?;
        let from = get_labels_from_str(&self.from, Arc::clone(&table))?;
        let to = get_labels_from_str(&self.to, Arc::clone(&table))?;

        Some(SoundLawLabels {
            from,
            to,
            left_context: left,
            right_context: right,
        })
    }

    // right now it also adds the replace context
    pub fn to_fst(&self, table: Arc<SymbolTable>) -> SoundFst {
        let SoundLawLabels {
            from,
            to,
            left_context,
            right_context,
        } = self.to_labels(Arc::clone(&table)).unwrap();
        let mut left_context_fst: VectorFst<_> = acceptor(&left_context, ProbabilityWeight::one());
        let right_context_fst: VectorFst<_> = acceptor(&right_context, ProbabilityWeight::one());

        let transform: VectorFst<_> = transducer(&from, &to, ProbabilityWeight::one());

        concat(&mut left_context_fst, &transform).expect("concat failed");
        concat(&mut left_context_fst, &right_context_fst).expect("concat failed");

        left_context_fst.set_input_symbols(Arc::clone(&table));
        left_context_fst.set_output_symbols(Arc::clone(&table));

let ret = replace_context(&transform, &self.left_context, &self.right_context, &table);
ret.unwrap()
    }
}


// old method to just transduce without paying attention to context, remove this later
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
    let labels: Vec<_> = chars
        .iter()
        .map(|x| symbol_table.get_label(x).unwrap())
        .collect();

    let acceptor: VectorFst<_> = acceptor(&labels, ProbabilityWeight::one());
    dbg!(&labels);
    dbg!(&chars);
    dbg!(&fst);
    dbg!(&acceptor);

    let mut composed: VectorFst<_> = compose(acceptor, fst).expect("Error in composition");
    composed.set_input_symbols(Arc::clone(&symbol_table));
    composed.set_output_symbols(Arc::clone(&symbol_table));

    dbg!(&composed);
    let paths: Vec<_> = composed.string_paths_iter().unwrap().collect();

    paths[0].ostring().expect("Error getting output string")
}

#[cfg(test)]
mod tests {
    use std::vec;

    use rustfst::{fst, symt};

    use super::*;

    #[test]
    fn test_simple() {
        let law = vec![vec!["h".into(), "q".into()], vec!["i".into(), "i".into()]];
        let transduced = transduce_text(law, String::from("hi"));
        assert_eq!(transduced, "q i");
    }

    #[test]
    #[ignore]
    fn test_no_duplicate() {
        let law = vec![vec!["h".into(), "q".into()]];
        let transduced = transduce_text(law, String::from("hi"));
        assert_eq!(transduced, "q i");
    }

    #[test]
    #[ignore]
    fn test_no_change() {
        let law = vec![vec!["a".into(), "b".into()]];
        let transduced = transduce_text(law, String::from("hi"));
        assert_eq!(transduced, "h i");
    }

    #[test]
    fn test_labels_from_string() {
        let table = symt!["a", "b", "c"];

        let transduced = get_labels_from_str("abba", Arc::new(table));
        assert_eq!(transduced, Some(vec![1, 2, 2, 1]));
    }

    #[test]
    fn test_no_context_soundlaw() {
        let table = symt!["a", "b", "c"];
        let table = Arc::new(table);
        let law = SoundLaw {
            from: "a".to_string(),
            to: "b".to_string(),
            left_context: "".to_string(),
            right_context: "".to_string(),
        };
        let fst = law.to_fst(table);
        dbg!(&fst);
        let paths: Vec<_> = fst.string_paths_iter().unwrap().collect();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].istring().unwrap(), "a");
        assert_eq!(paths[0].ostring().unwrap(), "b");
    }

    #[test]
    fn test_some_context_soundlaw() {
        let table = symt!["a", "b", "c"];
        let table = Arc::new(table);
        let law = SoundLaw {
            from: "a".to_string(),
            to: "b".to_string(),
            left_context: "c".to_string(),
            right_context: "c".to_string(),
        };
        let fst = law.to_fst(table);
        dbg!(&fst);
        let paths: Vec<_> = fst.string_paths_iter().unwrap().collect();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].istring().unwrap(), "c a c");
        assert_eq!(paths[0].ostring().unwrap(), "c b c");
    }

    #[ignore]
    #[test]
    fn right_arrow_test1() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let mapping: SoundFst = fst![3, 2 => 4];

        let input1: SoundFst = fst![3, 1, 3, 1, 3, 1, 3]; // "cacacac"

        let replaced = replace(mapping, false, &symbol_tabl);

        let expected: SoundFst = fst![3, 1, 3, 1, 3, 1 => 4, 4, 4];

        let actual = compose(input1, replaced).unwrap();

        assert_eq!(expected, actual);
    }
}
