use std::sync::Arc;

use rustfst::algorithms::compose::compose;
use rustfst::algorithms::determinize::determinize;
use rustfst::algorithms::{reverse, tr_sort, ProjectType};
use rustfst::fst_traits::StateIterator;
use rustfst::prelude::closure::{closure, ClosureType};
use rustfst::prelude::encode::{decode, encode};
use rustfst::prelude::rm_epsilon::rm_epsilon;
use rustfst::prelude::union::union;
use rustfst::prelude::{invert, minimize, OLabelCompare, SerializableFst, TropicalWeight};
use rustfst::{
    algorithms::{concat::concat, project},
    fst_impls::VectorFst,
    fst_traits::{Fst, MutableFst},
    utils::{acceptor, epsilon_machine},
    Label, Semiring, SymbolTable, Tr,
};
use rustfst::{fst, DrawingConfig};

#[derive(Clone, Debug, PartialEq)]
pub struct SoundFst(pub SoundVec);

#[cfg(test)]
const DEBUG: bool = true;

#[cfg(not(test))]
const DEBUG: bool = false;

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

    pub fn invert(&mut self) {
        invert(&mut self.0);
    }

    pub fn input_project(&mut self) {
        project(&mut self.0, ProjectType::ProjectInput)
    }

    pub fn output_project(&mut self) {
        project(&mut self.0, ProjectType::ProjectOutput)
    }
    fn any_star(st: &SymbolTable) -> Self {
        let mut fst: SoundVec = epsilon_machine().unwrap();
        for label in st.labels() {
            if label != 0 {
                let _ = fst.add_tr(0, Tr::new(label, label, SoundWeight::one(), 0));
            }
        }

        let arc: Arc<_> = st.clone().into();
        fst.set_input_symbols(Arc::clone(&arc));
        fst.set_output_symbols(Arc::clone(&arc));

        fst.into()
    }

    pub fn optimize(&mut self) {
        rm_epsilon(&mut self.0).unwrap();
        let table = encode(
            &mut self.0,
            rustfst::prelude::encode::EncodeType::EncodeLabels,
        )
        .unwrap();
        self.0 = determinize(&self.0).unwrap();
        minimize(&mut self.0).unwrap();
        decode(&mut self.0, table).unwrap();
    }

    pub fn compose(&mut self, other: &SoundFst) {
        tr_sort(&mut self.0, OLabelCompare {});
        let composed: SoundVec = compose(self.0.clone(), other.0.clone()).unwrap();

        self.0 = composed;
        // self.optimize();
    }

    fn concatenate(&mut self, other: &SoundFst) {
        concat(&mut self.0, &other.0).unwrap()
    }

    pub fn union(&mut self, other: &SoundFst) {
        union(&mut self.0, &other.0).unwrap()
    }

    pub fn determinize(&mut self) {
        rm_epsilon(&mut self.0).unwrap();
        self.0 = determinize(&self.0).unwrap();
    }

    pub fn reverse(&mut self) {
        self.0 = reverse(&self.0).unwrap();
    }
    pub fn df(&self, s: &str) {
        if DEBUG {
            self.0
                .draw(format!("images/{}.dot", s), &DrawingConfig::default())
                .unwrap()
        }
    }
    pub fn d(&self, line: u32) {
        if DEBUG {
            self.0
                .draw(format!("images/{}.dot", line), &DrawingConfig::default())
                .unwrap()
        }
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

        SoundFst::from(tc).negate(&alphabet.labels().collect::<Vec<_>>())
    }

    fn replace(&self, optional: bool, alphabet: &SymbolTable) -> Self {
        let tc_neg: Self = self.no_upper(alphabet);
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

    fn end_in_string(
        &self,
        left_context: Label,
        right_context: Label,
        alphabet: &SymbolTable,
    ) -> Self {
        // any string that ends in the current transducer, ignoring brackts except for a final <
        let mut transducer = self.clone();

        transducer.insert_freely(left_context);
        transducer.insert_freely(right_context);
        println!("{}", line!());

        let pi_star = Self::any_star(alphabet);
        let mut pi_star_free_mark = Self::any_star(alphabet);
        pi_star_free_mark.concatenate(&transducer);

        println!("{}", line!());
        let left_transducer: SoundFst = Self::from_single_label(left_context);
        let mut pi_star_copy = pi_star.clone();
        pi_star_copy.concatenate(&left_transducer);
        let pi_star_neg = pi_star_copy.negate_with_symbol_table(alphabet);

        println!("{}", line!());
        // let composed_transducer: SoundVec = compose(pi_star_free_mark, pi_star_neg).unwrap();
        pi_star_free_mark.compose(&pi_star_neg);

        let mut ret = pi_star_free_mark;
        ret.optimize();
        rm_epsilon(&mut ret.0).unwrap();
        ret.determinize();
        ret.optimize();
        ret
    }

    fn begin_bracket(left_context: Label, right_context: Label, alphabet: &SymbolTable) -> Self {
        // let left_transducer: SoundVec = fst![left_context];
        let left_transducer = Self::from_single_label(left_context);
        let pi_star = Self::any_star(alphabet);
        // let mut full_trans: SoundVec = fst![right_context];
        let mut full_trans = Self::from_single_label(right_context);
        closure(&mut full_trans.0, ClosureType::ClosureStar);
        // concat(&mut full_trans, &left_transducer).unwrap();
        full_trans.concatenate(&left_transducer);
        // concat(&mut full_trans, &pi_star.0).unwrap();
        full_trans.concatenate(&pi_star);
        full_trans
    }

    fn if_end_then_start(start: &SoundFst, end: &SoundFst, alphabet: &SymbolTable) -> SoundFst {
        let mut start = start.clone();
        let negated = end.negate_with_symbol_table(alphabet);
        start.concatenate(&negated);
        start
    }

    fn if_start_then_end(start: &SoundFst, end: &SoundFst, alphabet: &SymbolTable) -> SoundFst {
        let mut negated = start.negate_with_symbol_table(alphabet);
        negated.concatenate(end);
        negated
    }
    // take all contexts and replace it with a left marker
    fn replace_context(
        &self,
        left_context_marker: Label,
        right_context_marker: Label,
        alphabet: &SymbolTable,
    ) -> Self {
        println!("starting replace context");
        let end_in_transducer =
            self.end_in_string(left_context_marker, right_context_marker, alphabet);
        let start_bracket =
            Self::begin_bracket(left_context_marker, right_context_marker, alphabet);

        println!("{}", line!());
        // iff statement
        let start_then_end = Self::if_start_then_end(&end_in_transducer, &start_bracket, alphabet);

        let end_ten_start = Self::if_end_then_start(&end_in_transducer, &start_bracket, alphabet);
        //end_ten_start.optimize();

        let mut disjunction = start_then_end;
        disjunction.union(&end_ten_start);

        println!("disjunction {}", line!());
        disjunction.optimize();
        let mut ret = disjunction.negate_with_symbol_table(alphabet);

        ret.optimize();
        println!("Finished replace context");
        ret
    }

    fn insert_boundry_markers(alphabet: &SymbolTable, left: Label, right: Label) -> SoundFst {
        let mut pi = Self::any_star(alphabet);
        pi.0.emplace_tr(0, 0, left, SoundWeight::one(), 0).unwrap();
        pi.0.emplace_tr(0, 0, right, SoundWeight::one(), 0).unwrap();
        pi
    }
    fn remove_boundry_markers(alphabet: &SymbolTable, left: Label, right: Label) -> SoundFst {
        let mut pi = Self::any_star(alphabet);
        pi.0.emplace_tr(0, left, 0, SoundWeight::one(), 0).unwrap();
        pi.0.emplace_tr(0, right, 0, SoundWeight::one(), 0).unwrap();
        pi
    }

    fn constrain_boundry_markers(alphabet: &SymbolTable, left: Label, right: Label) -> SoundFst {
        let left_to_left = Self::from_single_label(left);

        let right_to_right = Self::from_single_label(right);
        let mut star = Self::any_star(alphabet);
        star.concatenate(&left_to_left);
        star.concatenate(&right_to_right);
        star.concatenate(&Self::any_star(alphabet));

        // star.optimize();
        star.negate_with_symbol_table(alphabet)
    }

    pub fn replace_in_context(
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

        println!("inserting boundry markers");
        let ibt: SoundFst = Self::insert_boundry_markers(alphabet, left_marker, right_marker);
        println!("removing boundry markers");
        let rbt: SoundFst = Self::remove_boundry_markers(alphabet, left_marker, right_marker); // remove boundry markers

        println!("constriaingin boundry markers");
        let cbt = Self::constrain_boundry_markers(&alphabet_with_marker, left_marker, right_marker);
        cbt.df("cbt");

        println!("left context");
        let mut lct =
            left_context.replace_context(left_marker, right_marker, &alphabet_with_marker);
        lct.optimize();
        lct.df("left_context");

        println!("right context");
        let mut right_rev: SoundFst = right_context;
        right_rev.reverse();
        right_rev.optimize();

        let mut rct = right_rev.replace_context(right_marker, left_marker, &alphabet_with_marker);
        rct.reverse();
        rct.optimize();
        rct.df("right_context");

        println!("create replace tranducer");
        let mut rt = self.replace_transducer(left_marker, right_marker, &alphabet_with_marker);
        rt.optimize();

        let mut result: SoundFst = ibt.clone();
        println!("composing cbt");
        result.compose(&cbt);
        result.df("compose_cbt");
        println!("composing rct");
        result.compose(&rct);
        result.df("compose_rct");
        println!("composing lct");
        result.compose(&lct);
        println!("composing lt");
        result.df("compose_lct");
        result.compose(&rt);
        result.df("compose_rt");
        println!("composing rbt");
        result.compose(&rbt);
        println!("done");
        result.df("compose_rbt");

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
        let mut transducer = self.clone();
        transducer.optimize();
        transducer.insert_freely(right_marker);
        transducer.insert_freely(left_marker);

        let mut marker_transducer: SoundFst = SoundFst::from_single_label(left_marker);
        let right_fst: Self = SoundFst::from_single_label(right_marker);
        marker_transducer.concatenate(&transducer);
        marker_transducer.concatenate(&right_fst);
        marker_transducer.optimize();

        let mut ret = marker_transducer.replace(false, alphabet);
        ret.optimize();
        ret
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

// given t that actually does the replacement, creates a transuducer that makes sure
// all substrings are repalced

// calls replace, but first ignores brackets and makes sure replacement occures only in brackets

#[cfg(test)]
mod tests {
    use std::vec;

    use rustfst::{fst, prelude::rm_epsilon::rm_epsilon, symt, DrawingConfig, utils::transducer};
    use sound_law::SoundLaw;

    use crate::sound_law;

    use super::*;

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
        rm_epsilon(&mut actual.0).unwrap();
        actual.optimize();
        let p: Vec<_> = actual.0.paths_iter().collect();
        assert_eq!(p[0].ilabels.as_slice(), &[1, 1, 1, 2, 3, 1, 2]);
        assert_eq!(p[0].olabels.as_slice(), &[1, 1, 3, 4, 3, 3, 4]);

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

        //assert_eq!(expected, actual.0);
    }

    #[test]
    fn begin_bracket_test() {
        let input: SoundVec = fst![4];
        let symbol_tabl = symt!["a", "c", "d", "<", ">"];
        let fst = SoundFst::begin_bracket(4, 5, &symbol_tabl);

        let output: SoundVec = compose(input, fst.0).unwrap();
        assert!(output.paths_iter().count() == 1);
    }
    #[test]
    fn end_in_string() {
        let left: SoundVec = fst![2, 1];
        let left: SoundFst = left.into();
        let input: SoundVec = fst![2, 2, 2, 1];
        let symbol_tabl = symt!["a", "c", "d", "<", ">"];
        let fst = left.end_in_string(4, 5, &symbol_tabl);

        let output: SoundVec = compose(input, fst.0).unwrap();
        assert!(output.paths_iter().count() == 1);
    }

    #[test]
    fn replace_left_test() {
        let left: SoundVec = fst![2, 1];
        let left: SoundFst = left.into();
        let symbol_tabl = symt!["a", "c", "d", "<", ">"];
        left.replace_context(4, 5, &symbol_tabl);
    }

    #[test]
    fn replace_transducer_test() {
        let symbol_tabl = symt!["a", "c", "d", "<", ">"];
        let mapping: SoundVec = fst![3, 2 => 4];
        let mapping: SoundFst = mapping.into();

        //let input1: SoundVec = fst![3, 1, 3, 1, 3, 1, 3]; // "cacacac"
        mapping.replace_transducer(4, 5, &symbol_tabl);
    }

    #[test]
    fn right_arrow_test1() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let mapping: SoundVec = fst![3, 1 => 4];

        let left: SoundVec = fst![3, 1];
        let right: SoundVec = fst![3];

        let input1: SoundVec = fst![3, 1, 3, 1, 3, 1, 3]; // "cacacac"

        let mut replaced =
            SoundFst(mapping).replace_in_context(left.into(), right.into(), false, &symbol_tabl);
        replaced.optimize();

        let mut actual = SoundFst(input1);
        actual.compose(&replaced);
        project(&mut actual.0, ProjectType::ProjectOutput);
        let paths: Vec<_> = actual.0.paths_iter().collect();
        let a = paths[0].olabels.as_slice();

        assert_eq!(a, &[3, 1, 4, 4, 3]);
    }

    #[test]
    fn right_arrow_test_with_transduce_text() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let law = SoundLaw::new("ca", "d", "ca", "c", &symbol_tabl);

        let transduced = law.transduce_text("cacacac");

        assert_eq!(transduced[0], "c a d d c");
    }

    #[test]
    fn symbol_compose_test() {
        let st = symt!["a", "b", "c"];
        let st2 = symt!["x", "y", "z"];
        let st3 = symt!["1", "2", "3"];
        let arc = Arc::new(st);
        let mut x = SoundFst::from_single_label(1);
        x.0.set_input_symbols(arc.clone());
        x.0.set_output_symbols(arc.clone());

        let mut x_clone = x.clone();
        x_clone.0.set_input_symbols(st3.clone().into());
        x_clone.0.set_output_symbols(st3.clone().into());

        let star = SoundFst::any_star(&st2);
        x.compose(&star);
        let vec: Vec<_> = x.0.string_paths_iter().unwrap().collect();
        assert_eq!(vec[0].ostring().unwrap(), "x");
        assert_eq!(vec[0].istring().unwrap(), "a");

        x.union(&x_clone);

        let vec: Vec<_> = x.0.string_paths_iter().unwrap().collect();
        assert_eq!(vec[0].ostring().unwrap(), "x");
        assert_eq!(vec[0].istring().unwrap(), "a");
        assert_eq!(vec[1].ostring().unwrap(), "x");
        assert_eq!(vec[1].istring().unwrap(), "a");
    }
}
