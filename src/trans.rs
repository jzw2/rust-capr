use std::sync::Arc;

use rustfst::algorithms::compose::compose;
use rustfst::algorithms::determinize::determinize;
use rustfst::algorithms::ProjectType;
use rustfst::fst_traits::StateIterator;
use rustfst::prelude::closure::{closure, ClosureType};
use rustfst::prelude::union::union;
use rustfst::prelude::{SerializableFst, TropicalWeight};
use rustfst::{
    algorithms::{concat::concat, project},
    fst_impls::VectorFst,
    fst_traits::{Fst, MutableFst},
    utils::{acceptor, epsilon_machine, transducer},
    Label, Semiring, SymbolTable, Tr,
};
use rustfst::{fst, DrawingConfig};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct SoundFst(pub SoundVec);

pub type SoundVec = VectorFst<SoundWeight>;

impl From<VectorFst<SoundWeight>> for SoundFst {
    fn from(value: VectorFst<SoundWeight>) -> Self {
        SoundFst(value)
    }
}

impl From<SoundFst> for VectorFst<SoundWeight> {
    fn from(value: SoundFst) -> Self {
        value.0
    }
}
pub type SoundWeight = TropicalWeight;

impl SoundFst {
    fn from_single_label(l: Label) -> Self {
        let v: SoundVec = fst![l];
        SoundFst(v)
    }

    pub fn input_project(&mut self) {
        project(&mut self.0, ProjectType::ProjectInput)
    }

    fn any_star(st: &SymbolTable) -> Self {
        let mut fst: SoundVec = epsilon_machine().unwrap();
        for label in st.labels() {
            if label != 0 {
                let _ = fst.add_tr(0, Tr::new(label, label, SoundWeight::one(), 0));
            }
        }
        fst.into()
    }

    pub fn compose(&mut self, other: &SoundFst) {
        let composed: SoundVec = compose(self.0.clone(), other.0.clone()).unwrap();
        self.0 = composed;
    }

    fn concatenate(&mut self, other: &SoundFst) {
        concat(&mut self.0, &other.0).unwrap()
    }

    pub fn union(&mut self, other: &SoundFst) {
        union(&mut self.0, &other.0).unwrap()
    }

    pub fn determinize(&mut self) {
        self.0 = determinize(&self.0).unwrap();
    }
    fn no_upper(&self, alphabet: &SymbolTable) -> Self {
        let mut projection = self.0.clone();
        project(
            &mut projection,
            rustfst::algorithms::ProjectType::ProjectInput,
        ); // should be output in the lower level projection
        let star = Self::any_star(alphabet);

        let mut tc = star.clone().0;

        concat(&mut tc, &projection).unwrap();
        concat(&mut tc, &star.0).unwrap();

        SoundFst::from(tc)
            .negate(&alphabet.labels().collect::<Vec<_>>())
    }

    fn replace(&self, optional: bool, alphabet: &SymbolTable) -> Self {
        let tc_neg: Self = self.no_upper(alphabet);
        tc_neg
            .0
            .draw("images/tc_neg.dot", &DrawingConfig::default())
            .unwrap();
        let star = Self::any_star(alphabet);

        let mut retval: SoundVec = tc_neg.clone().0;
        concat(&mut retval, &self.0).unwrap();
        closure(&mut retval, ClosureType::ClosureStar);
        concat(&mut retval, &tc_neg.0).unwrap();

        if optional {
            union(&mut retval, &star.0).unwrap();
        }

        retval.into()
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
        let mut pi_star_free_mark = Self::any_star(alphabet).0;
        concat(&mut pi_star_free_mark, &transducer.0).unwrap();

        let left_transducer: SoundVec = fst![left_context];
        let mut pi_star_copy = pi_star.clone();
        concat(&mut pi_star_copy.0, &left_transducer).unwrap();
        let pi_star_neg = pi_star_copy.negate_with_symbol_table(alphabet).0;

        let composed_transducer: SoundVec = compose(pi_star_free_mark, pi_star_neg).unwrap();
        let mut full_trans: SoundVec = fst![right_context];
        closure(&mut full_trans, ClosureType::ClosureStar);
        concat(&mut full_trans, &left_transducer).unwrap();
        concat(&mut full_trans, &pi_star.0).unwrap();

        // iff statement
        let neg_full = SoundFst::from(full_trans.clone()).negate_with_symbol_table(alphabet);
        let mut composed_neg_full = composed_transducer.clone();
        concat(&mut composed_neg_full, &neg_full.0).unwrap();

        let mut neg_composed_full =
            SoundFst::from(composed_transducer).negate_with_symbol_table(alphabet);
        concat(&mut neg_composed_full.0, &full_trans).unwrap();

        let mut disjunction = neg_composed_full;
        union(&mut disjunction.0, &composed_neg_full).unwrap();
        disjunction.negate_with_symbol_table(alphabet)

        // they optimize it, don't know what the equivalent is
    }

    fn replace_in_context(
        &self,
        left_context: SoundFst,
        right_context: SoundFst,
        optional: bool,
        alphabet: &SymbolTable,
    ) -> SoundFst {
        let mut t1_proj = left_context.clone();
        t1_proj.input_project();
        let mut t2_proj = right_context.clone();
        t2_proj.input_project();

        // they create some sort of left marker, but I think this is unecessary

        let mut alphabet_with_marker = alphabet.clone();
        let left_marker = alphabet_with_marker.add_symbol("left_marker");
        let right_marker = alphabet_with_marker.add_symbol("right_marker");

        let mut ibt: SoundFst = todo!(); // inserting the boundry markers
        let mut rbt: SoundFst = todo!(); // remove boundry markers

        let pi_star = Self::any_star(&alphabet_with_marker);

        let cbt = todo!(); // constrain boudnry marker

        let lct = left_context.replace_context(left_marker, right_marker, &alphabet_with_marker);

        let right_rev: SoundFst = todo!();

        let rct = right_rev.replace_context(right_marker, right_marker, &alphabet_with_marker);

        let rt = self.replace_transducer(left_marker, right_marker, &alphabet_with_marker);

        let mut result: SoundFst = ibt.clone();
        result.compose(&cbt);
        result.compose(&rct);
        result.compose(&lct);
        result.compose(&rt);
        result.compose(&rbt);

        if optional {
            todo!()
        }

        result
    }

    // add left and right markers and makes sure left/right markers are ignored in oriignal fst
    fn replace_transducer(
        &self,
        left_marker: Label,
        right_marker: Label,
        alphabet: &SymbolTable,
    ) -> Self {
        // ignore the opitmiaze because I don't know what it does

        let mut transducer = self.clone();
        transducer.insert_freely(right_marker);
        transducer.insert_freely(left_marker);

        let mut marker_transducer: SoundFst = SoundFst::from_single_label(left_marker);
        let right_fst: Self = SoundFst::from_single_label(right_marker);
        marker_transducer.concatenate(&transducer);
        marker_transducer.concatenate(&right_fst);

        marker_transducer.replace(false, alphabet)
    }

    // allows s to be inputted anywhere inside the fst
    fn insert_freely(&mut self, s: Label) {
        for state in self.clone().0.states_iter() {
            self.0
                .emplace_tr(state, s, s, SoundWeight::one(), state)
                .unwrap();
        }
    }
}

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
        let mut left_context_fst: VectorFst<_> = acceptor(&left_context, SoundWeight::one());
        let right_context_fst: VectorFst<_> = acceptor(&right_context, SoundWeight::one());

        let transform: VectorFst<_> = transducer(&from, &to, SoundWeight::one());

        concat(&mut left_context_fst, &transform).expect("concat failed");
        concat(&mut left_context_fst, &right_context_fst).expect("concat failed");

        left_context_fst.set_input_symbols(Arc::clone(&alphabet));
        left_context_fst.set_output_symbols(Arc::clone(&alphabet));

        left_context_fst.into()
    }
}

// old method to just transduce without paying attention to context, remove this later
pub fn transduce_text(laws: Vec<Vec<String>>, text: String) -> String {
    let mut fst = VectorFst::<SoundWeight>::new();
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

    let acceptor: VectorFst<_> = acceptor(&labels, SoundWeight::one());
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

    use rustfst::{
        fst, prelude::rm_epsilon::rm_epsilon, symt,
        DrawingConfig,
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
        let paths: Vec<_> = fst.0.string_paths_iter().unwrap().collect();
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
        let paths: Vec<_> = fst.0.string_paths_iter().unwrap().collect();
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].istring().unwrap(), "c a c");
        assert_eq!(paths[0].ostring().unwrap(), "c b c");
    }

    #[test]
    fn simple_replace_multiple() {
        let mapping: SoundVec = fst![1, 2 => 3,4];

        let input1: SoundVec = fst![1, 1, 1, 2, 3, 1, 2];

        let symbol_table = symt![1, 2, 3, 4];

        let replaced = SoundFst(mapping).replace(false, &symbol_table);

        let expected: SoundVec = fst![1,1,1,2,3,1,2 => 1,1,3,4,3,3,4 ];

        // minimize_with_config(&mut expected, MinimizeConfig { allow_nondet: true, ..MinimizeConfig::default()}).unwrap();
        // minimize_with_config(&mut actual, MinimizeConfig { allow_nondet: false, ..MinimizeConfig::default()}).unwrap();
        let mut actual: SoundFst = SoundFst(input1);
        actual.compose(&replaced);
        actual.determinize();
        rm_epsilon(&mut actual.0);
        actual
            .0
            .draw("images/simple_actual_no_rm.dot", &DrawingConfig::default())
            .unwrap();

        expected
            .draw(
                "images/simple_replace_expected.dot",
                &DrawingConfig::default(),
            )
            .unwrap();
        actual
            .0
            .draw(
                "images/simple_actual_expected.dot",
                &DrawingConfig::default(),
            )
            .unwrap();

        assert_eq!(expected, actual.0);
    }

    #[ignore = "Ned to fix it to be replace in context"]
    #[test]
    fn right_arrow_test1() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let mapping: SoundVec = fst![3, 2 => 4];

        let left: SoundVec = fst![3, 1];
        let right: SoundVec = fst![3];

        let input1: SoundVec = fst![3, 1, 3, 1, 3, 1, 3]; // "cacacac"

        let replaced = SoundFst(mapping).replace(false, &symbol_tabl);

        let expected: SoundVec = fst![3, 1, 3, 1, 3, 1 => 4, 4, 4];

        let mut actual = SoundFst(input1);
        actual.compose(&replaced);

        assert_eq!(expected, actual.0);
    }
}
