use std::sync::Arc;

use rustfst::algorithms::compose::compose;
use rustfst::algorithms::rm_epsilon::*;
use rustfst::fst;
use rustfst::prelude::closure::{closure, ClosureType};
use rustfst::prelude::determinize::determinize;
use rustfst::prelude::union::union;
use rustfst::prelude::{
    tr_sort, AllocableFst, ExpandedFst, ILabelCompare, OLabelCompare, StateIterator,
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

use crate::negate::SoundFstNegateTrait;

pub type SoundFst = VectorFst<ProbabilityWeight>;

impl FstTraits for SoundFst {}
pub trait FstTraits:
    Clone
    + Fst<ProbabilityWeight>
    + MutableFst<ProbabilityWeight>
    + AllocableFst<ProbabilityWeight>
    + ExpandedFst<ProbabilityWeight>
{
}
pub trait SoundFstTrait: FstTraits + SoundFstNegateTrait {
    fn any_star(st: &SymbolTable) -> Self {
        let mut fst: Self = epsilon_machine().unwrap();
        for label in st.labels() {
            let _ = fst.add_tr(0, Tr::new(label, label, ProbabilityWeight::one(), 0));
        }
        fst
    }

    fn replace(&self, optional: bool, alphabet: &SymbolTable) -> Self {
        let mut projection = self.clone();
        project(
            &mut projection,
            rustfst::algorithms::ProjectType::ProjectInput,
        ); // should be output in the lower level projection
        let star = Self::any_star(alphabet);

        let mut tc = star.clone();

        concat(&mut tc, self).unwrap();
        concat(&mut tc, &star).unwrap();

        let tc_neg: Self = tc.negate(&alphabet.labels().collect::<Vec<_>>());

        let mut retval: Self = tc_neg.clone();
        concat(&mut retval, self).unwrap();
        closure(&mut retval, ClosureType::ClosureStar);
        concat(&mut retval, &tc_neg).unwrap();

        if optional {
            union(&mut retval, &star).unwrap();
        }

        retval
    }

    fn replace_context(
        &self,
        left_context: Label,
        right_context: Label,
        alphabet: &SymbolTable,
    ) -> Self {
        // copied from hfst, ideally I'll refactor it so that it actually makes sense
        let mut transducer = self.clone();

        transducer.insert_freely(left_context);
        transducer.insert_freely(right_context);

        let pi_star = Self::any_star(alphabet);
        let mut pi_star_free_mark = Self::any_star(alphabet);
        concat(&mut pi_star_free_mark, &transducer).unwrap();

        let left_transducer: Self = fst![left_context];
        let mut pi_star_copy = pi_star.clone();
        concat(&mut pi_star_copy, &left_transducer).unwrap();
        let pi_star_neg = pi_star_copy.negate_with_symbol_table(alphabet);

        let composed_transducer: Self = compose(pi_star_free_mark, pi_star_neg).unwrap();
        let mut full_trans: Self = fst![right_context];
        closure(&mut full_trans, ClosureType::ClosureStar);
        concat(&mut full_trans, &left_transducer).unwrap();
        concat(&mut full_trans, &pi_star).unwrap();

        // iff statement
        let neg_full = full_trans.negate_with_symbol_table(alphabet);
        let mut composed_neg_full = composed_transducer.clone();
        concat(&mut composed_neg_full, &neg_full).unwrap();

        let mut neg_composed_full = composed_transducer.negate_with_symbol_table(alphabet);
        concat(&mut neg_composed_full, &full_trans).unwrap();

        let mut disjunction = neg_composed_full;
        union(&mut disjunction, &composed_neg_full).unwrap();
        disjunction.negate_with_symbol_table(alphabet)

        // they optimize it, don't know what the equivalent is
    }

    fn replace_transducer(
        &self,
        left_marker: Label,
        right_marker: Label,
        alphabet: &SymbolTable,
    ) -> Self {
        // ignore the opitmiaze because I don't know what it does

        let transducer = self.clone();
        transducer.insert_freely(right_marker);
        transducer.insert_freely(left_marker);

        let mut marker_transducer: Self = fst![left_marker];
        let right_fst: Self = fst![right_marker];
        concat(&mut marker_transducer, &transducer).unwrap();
        concat(&mut marker_transducer, &right_fst).unwrap();

        marker_transducer.replace(false, alphabet)
    }

    // allows s to be inputted anywhere inside the fst
    fn insert_freely(&self, s: Label) {
        todo!();
    }
}

impl SoundFstTrait for SoundFst {}
/// example we want x -> y / a _ b, ie x turns to y when it is in front of a and before b
/// aka axb -> ayb
/// a = b = x, in string xxxx,
/// `from` is x
/// `to` is y
/// `left_context` is a
/// `right_context` is b
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
            from: from.to_string(),
            to: to.to_string(),
            left_context: left_context.into(),
            right_context: right_context.into(),
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

// calls replace, but first ignores brackets and makes sure replacement occures only in brackets

impl SoundLaw {
    // given t that does replacement, with contexyts
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
    pub fn to_fst(&self, alphabet: Arc<SymbolTable>) -> SoundFst {
        let SoundLawLabels {
            from,
            to,
            left_context,
            right_context,
        } = self.to_labels(Arc::clone(&alphabet)).unwrap();
        let mut left_context_fst: VectorFst<_> = acceptor(&left_context, ProbabilityWeight::one());
        let right_context_fst: VectorFst<_> = acceptor(&right_context, ProbabilityWeight::one());

        let transform: VectorFst<_> = transducer(&from, &to, ProbabilityWeight::one());

        concat(&mut left_context_fst, &transform).expect("concat failed");
        concat(&mut left_context_fst, &right_context_fst).expect("concat failed");

        left_context_fst.set_input_symbols(Arc::clone(&alphabet));
        left_context_fst.set_output_symbols(Arc::clone(&alphabet));

        // transform.replace_context(left_context, right_context, &alphabet);
        todo!()
    }
}

pub fn replaee_in_context(
    contex_left: &SoundFst,
    context_right: &SoundFst,
    t: SoundFst,
    optional: bool,
    alphabet: &SymbolTable,
) {
    // the big function in hfst
    // currently not supporting the replace type paramter
    todo!()
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

        let replaced = mapping.replace(false, &symbol_tabl);

        let expected: SoundFst = fst![3, 1, 3, 1, 3, 1 => 4, 4, 4];

        let actual = compose(input1, replaced).unwrap();

        assert_eq!(expected, actual);
    }
}
