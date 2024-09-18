use std::sync::Arc;

use rustfst::algorithms::compose::compose;
use rustfst::algorithms::rm_epsilon::*;
use rustfst::fst;
use rustfst::prelude::determinize::determinize;
use rustfst::prelude::{
    tr_sort, CoreFst, ILabelCompare, OLabelCompare, SerializableFst, StateIterator,
};
use rustfst::{
    algorithms::{concat::concat, project},
    fst_impls::VectorFst,
    fst_traits::{Fst, MutableFst},
    semirings::ProbabilityWeight,
    utils::{acceptor, epsilon_machine, transducer},
    Label, Semiring, SymbolTable, Tr,
};
use serde::{Deserialize, Serialize};

type SoundFst = VectorFst<ProbabilityWeight>;

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

fn any_star(st: &SymbolTable) -> SoundFst {
    let mut fst: SoundFst = epsilon_machine().unwrap();
    for label in st.labels() {
        let _ = fst.add_tr(0, Tr::new(label, label, ProbabilityWeight::one(), 0));
    }
    fst
}

fn negate(fst: &SoundFst, alphabet: &[Label]) -> SoundFst {
    // assumet that the fst is deterministic, and also acceptor or whatever that is aka is a regex

    // also destroys weights
    let mut ret = fst.clone();

    let accept = ret.add_state();

    let fst = ret.clone();

    dbg!(accept);

    for state in ret.states_iter() {
        dbg!(state);
        if fst.is_final(state).unwrap() {
            ret.set_final(state, ProbabilityWeight::zero());
        } else {
            ret.set_final(state, ProbabilityWeight::one());
        }
        alphabet
            .iter()
            .filter(|label| {
                fst.get_trs(state)
                    .unwrap()
                    .iter()
                    .all(|tr| tr.ilabel != **label)
            })
            .for_each(|label| {
                dbg!(label);
                ret.emplace_tr(state, *label, *label, ProbabilityWeight::one(), accept)
                    .expect("unable to add label");
                dbg!(ret.get_trs(state).unwrap().len());
            });

        dbg!(state);
    }
    ret.set_final(accept, ProbabilityWeight::one());

    ret
}

fn subtract(fst1: &SoundFst, fst2: &SoundFst) -> SoundFst {
    // mostly translated from hfst's version
    // in TroplicalWeightTransducer.cc
    let mut new_fst1 = fst1.clone();
    rm_epsilon(&mut new_fst1);
    let mut new_fst2 = fst2.clone();
    rm_epsilon(&mut new_fst2);

    tr_sort(&mut new_fst1, OLabelCompare {}); // weird design syntax
    tr_sort(&mut new_fst2, ILabelCompare {});

    let fst2_det: SoundFst = determinize(&new_fst2).unwrap();

    todo!()
}

// given t that actually does the replacement, creates a transuducer that makes sure
// all substrings are repalced
fn replace(
    t: SoundFst,
    optional: bool,
    alphabet: &SymbolTable,
) -> Option<VectorFst<ProbabilityWeight>> {
    let mut projection = t.clone();
    project(
        &mut projection,
        rustfst::algorithms::ProjectType::ProjectInput,
    ); // should be output in the lower level projection
    let star = any_star(alphabet);

    let tc = star.clone();

    todo!()
}

// calls replace, but first ignores brackets and makes sure replacement occures only in brackets
fn replace_transducer(
    t: SoundFst,
    left_marker: String,
    right_marker: &String,
    alphabet: SymbolTable,
) -> Option<VectorFst<ProbabilityWeight>> {
    todo!()
}

// given t that does replacement, m1, m2 are the contexts
fn function_name(
    t: SoundFst,
    left_context: String,
    right_context: &String,
    alphabet: SymbolTable,
) -> Option<VectorFst<ProbabilityWeight>> {
    todo!()
}

impl SoundLaw {
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

    fn to_fst(&self, table: Arc<SymbolTable>) -> SoundFst {
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

        left_context_fst
    }
}

// I forgot what this is for
/// - I don't think this approach is the most general approach
fn sound_laws_to_fst(laws: &[SoundLaw], table: Arc<SymbolTable>) -> SoundFst {
    todo!()
}

fn accepts(fst: &SoundFst, string: &[Label]) -> bool {
    let accept: SoundFst = acceptor(string, ProbabilityWeight::one());
    let composed: SoundFst = compose(accept, fst.clone()).expect("Error in composition");
    composed.draw("accepts.out", &Default::default());
    composed.paths_iter().next().is_some()
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

    use actix_web::cookie::time::{formatting, macros};
    use rustfst::{
        prelude::{FstIterator, SerializableFst},
        symt, DrawingConfig,
    };

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

    #[test]
    fn right_arrow_test1() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let mapping: SoundFst = fst![3, 2 => 4];

        let input1: SoundFst = fst![3, 1, 3, 1, 3, 1, 3]; // "cacacac"

        let replaced = replace(mapping, false, &symbol_tabl).unwrap();

        let expected: SoundFst = fst![3, 1, 3, 1, 3, 1 => 4, 4, 4];

        let actual = compose(input1, replaced).unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn negate_test1() {
        let fst = fst![1, 2, 3];

        let negate = negate(&fst, &vec![1, 2, 3]);

        let str = vec![1, 2, 3];
        assert!(accepts(&fst, &str));
        assert!(!accepts(&negate, &str));
    }
    #[test]
    fn negate_test_multiple_strings() {
        // FST that accepts [1,2,3] and [4,5,6]
        let mut fst1: SoundFst = fst![1, 2, 3];
        let mut fst2: SoundFst = fst!(4, 5, 6);
        let alpha = vec![1, 2, 3, 4, 5, 6];
        let _ = rustfst::algorithms::union::union(&mut fst1, &mut fst2).unwrap();

        let mut det_union_fst = determinize(&fst1).unwrap();
        let _ = rm_epsilon(&mut det_union_fst).unwrap();

        let negate_fst = negate(&det_union_fst, &alpha);
        //:dbg!(negate_fst.get_trs(8).unwrap().len());
        negate_fst.draw("image.txt", &DrawingConfig::default());

        let input1 = vec![1, 2, 3];
        let input2 = vec![4, 5, 6];
        let input3 = vec![3, 2, 1];

        assert!(accepts(&det_union_fst, &input1));
        assert!(accepts(&det_union_fst, &input2));
        assert!(!accepts(&det_union_fst, &input3));

        assert!(!accepts(&negate_fst, &input1));
        assert!(!accepts(&negate_fst, &input2));

        dbg!(&negate_fst);
        assert!(accepts(&negate_fst, &input3));
    }
}
