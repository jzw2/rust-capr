use std::sync::Arc;

use crate::regex::RegexFst;
use crate::tables::ipa;
use crate::trans::SoundFst;
use crate::trans::SoundVec;
use crate::trans::SoundWeight;
use ipa_translate::xsampa_to_ipa;
use rustfst::prelude::union::union;
use rustfst::prelude::Fst;
use rustfst::prelude::MutableFst;
use rustfst::prelude::VectorFst;
use rustfst::utils::acceptor;
use rustfst::utils::transducer;
use rustfst::Label;
use rustfst::Semiring;
use rustfst::SymbolTable;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SoundLaw {
    // need to rewrite so taht it has not such a flat architucture
    // and allows more complex such as full regex expression
    from: String,
    to: String,
    left_context: String,
    right_context: String,
    fst: SoundFst,

    #[serde(skip_serializing, skip_deserializing)]
    table: SymbolTable,
}

const LIMIT: usize = 20;

// this is essnetially useless, and I should delte it soon
#[derive(Debug)]
struct SoundLawLabels {
    //
    from: Vec<Label>,
    to: Vec<Label>,
    left_context: Vec<Label>,
    right_context: Vec<Label>,
}

pub fn get_labels_from_str(s: &str, table: &SymbolTable) -> Option<Vec<Label>> {
    s.chars().map(|x| table.get_label(x.to_string())).collect()
}

/// example we want x -> y / a _ b, ie x turns to y when it is in front of a and before b
/// aka axb -> ayb
/// a = b = x, in string xxxx,
/// `from` is x
/// `to` is y
/// `left_context` is a
/// `right_context` is b

impl SoundLaw {
    // should be removed and the regex method replaced
    pub fn disjunction_vec_fst<T: AsRef<str>>(
        strings: &[T],
        table: &SymbolTable,
    ) -> VectorFst<SoundWeight> {
        let labeled_strings = strings
            .iter()
            .map(|s| get_labels_from_str(s.as_ref(), table).unwrap());
        let mut new_fst: VectorFst<SoundWeight> = VectorFst::new();

        for labeled_string in labeled_strings {
            let acceptor: VectorFst<_> = acceptor(&labeled_string, SoundWeight::one());
            union(&mut new_fst, &acceptor).expect("union failed");
        }
        new_fst
    }

    // should be removed and the regex method replaced
    pub fn new_with_vec_context(
        from: &str,
        to: &str,
        left_context: VectorFst<SoundWeight>,
        right_context: VectorFst<SoundWeight>,
        table: &SymbolTable,
    ) -> SoundLaw {
        let labels = [from, to].map(|s| get_labels_from_str(s, table).unwrap());

        // the left and right contexts fst
        let left_context_fst: VectorFst<_> = left_context;
        let right_context_fst: VectorFst<_> = right_context;

        // the actual input to output
        let transform: VectorFst<_> = transducer(&labels[0], &labels[1], SoundWeight::one());
        let transform = SoundFst(transform);

        let replace_fst = transform.replace_in_context(
            left_context_fst.into(),
            right_context_fst.into(),
            false,
            table,
        );
        SoundLaw {
            from: from.to_string(),
            to: to.to_string(),
            left_context: "disjunction".into(),
            right_context: "disjunction".into(),
            fst: replace_fst,
            table: table.clone(),
        }
    }

    pub fn create_with_arbitrary_regex(
        left: &RegexFst,
        right: &RegexFst,
        from: &RegexFst,
        to: &RegexFst,
        table: &SymbolTable,
    ) -> SoundLaw {
        // let latin = lower_case_latin();

        let contains_epsilon = from.is_empty();

        // let transform: SoundFst = if contains_epsilon {
        //     RegexFst::regex_cross_product(to, from, table)
        // } else {
        //     RegexFst::regex_cross_product(from, to, table)
        // };
        let transform: SoundFst = RegexFst::regex_cross_product(from, to, table);
        transform.df("cross_product");
        let mut replace_fst =
            transform.replace_in_context(left.to_sound_fst(), right.to_sound_fst(), false, table);
        // if contains_epsilon {
        //     replace_fst.invert();
        // }

        replace_fst.df("all_regex");
        SoundLaw {
            from: from.to_string(),
            to: to.to_string(),
            left_context: left.to_string(),
            right_context: right.to_string(),
            fst: replace_fst,
            table: table.clone(),
        }
    }

    // this should not be the new, one isntea,d it should
    // be changed to be basedd on the regex creation
    pub fn new(
        from: &str,
        to: &str,
        left_context: &str,
        right_context: &str,
        table: &SymbolTable,
    ) -> SoundLaw {
        let labels =
            [left_context, right_context, from, to].map(|s| get_labels_from_str(s, table).unwrap());
        // dbg!(&labels);

        // the left and right contexts fst
        let left_context_fst: VectorFst<_> = acceptor(&labels[0], SoundWeight::one());
        let right_context_fst: VectorFst<_> = acceptor(&labels[1], SoundWeight::one());

        // the actual input to output
        let transform: VectorFst<_> = transducer(&labels[2], &labels[3], SoundWeight::one());
        let transform = SoundFst(transform);

        let replace_fst = transform.replace_in_context(
            left_context_fst.into(),
            right_context_fst.into(),
            false,
            table,
        );
        SoundLaw {
            from: from.to_string(),
            to: to.to_string(),
            left_context: left_context.into(),
            right_context: right_context.into(),
            fst: replace_fst,
            table: table.clone(),
        }
    }

    pub fn get_fst(&self) -> &SoundFst {
        &self.fst
    }
    pub fn get_table(&self) -> &SymbolTable {
        &self.table
    }
}

impl SoundLaw {
    // given t that does replacement, with contexyts
    //might be unneeded if I want to refactor it completely with just labels, vs passing the string along always
    fn to_labels(&self, table: &SymbolTable) -> Option<SoundLawLabels> {
        let left = get_labels_from_str(&self.left_context, table)?;
        let right = get_labels_from_str(&self.right_context, table)?;
        let from = get_labels_from_str(&self.from, table)?;
        let to = get_labels_from_str(&self.to, table)?;

        Some(SoundLawLabels {
            from,
            to,
            left_context: left,
            right_context: right,
        })
    }

    // right now it also adds the replace context
    pub fn to_fst(&self, alphabet: &SymbolTable) -> SoundFst {
        let SoundLawLabels {
            from,
            to,
            left_context,
            right_context,
        } = self.to_labels(alphabet).unwrap();
        let left_context_fst: VectorFst<_> = acceptor(&left_context, SoundWeight::one());
        let right_context_fst: VectorFst<_> = acceptor(&right_context, SoundWeight::one());

        let transform: VectorFst<_> = transducer(&from, &to, SoundWeight::one());
        let transform = SoundFst(transform);

        transform.replace_in_context(
            left_context_fst.into(),
            right_context_fst.into(),
            false,
            alphabet,
        )
    }
}

#[wasm_bindgen]
impl SoundLaw {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Unwrap json string failed")
    }

    pub fn from_json(s: &str) -> Self {
        // just asume an ipa for now
        let mut ret: Self = serde_json::from_str(s).expect("Unwrap json string failed");
        let table = Arc::new(ipa());
        ret.table = ipa(); // kind of wasteful, should change this to not recreate the table very time
        ret.fst.0.set_input_symbols(Arc::clone(&table));
        ret.fst.0.set_output_symbols(Arc::clone(&table));
        ret
    }
    pub fn get_from(&self) -> String {
        self.from.to_string()
    }
    pub fn get_to(&self) -> String {
        self.to.to_string()
    }
    pub fn get_left_context(&self) -> String {
        self.left_context.to_string()
    }
    pub fn get_right_context(&self) -> String {
        self.right_context.to_string()
    }

    pub fn transduce_text(&self, text: &str) -> Vec<String> {
        transduce_text_with_symbol_table(&self.fst, &self.table, text)
    }

    pub fn transduce_text_backwards(&self, text: &str) -> Vec<String> {
        let mut invert = self.clone();
        invert.fst.invert();
        invert.transduce_text(text)
    }
}

fn transduce_text_with_symbol_table(
    fst: &SoundFst,
    table: &SymbolTable,
    text: &str,
) -> Vec<String> {
    let labels: Vec<_> = text
        .chars()
        //.inspect(|c| println!("{}", c))
        .map(|c| {
            table.get_label(c.to_string()).expect(&format!(
                "Character {} was not found in symbol table",
                c.to_string()
            ))
        })
        .collect();
    transduce_from_labels(fst, table, &labels)
}

fn transduce_from_labels(fst: &SoundFst, table: &SymbolTable, labels: &[Label]) -> Vec<String> {
    let t = fst;
    let text_fst: VectorFst<_> = acceptor(&labels, SoundWeight::one());
    let mut text_fst: SoundFst = text_fst.into();

    let table = Arc::new(table.clone());
    text_fst.compose(t);
    text_fst.output_project();
    text_fst.0.set_output_symbols(Arc::clone(&table));
    text_fst.0.set_input_symbols(Arc::clone(&table));

    // let acceptor: VectorFst<_> = acceptor(&labels, SoundWeight::one());
    let output: Vec<String> = text_fst
        .0
        .string_paths_iter()
        // .inspect(|x| println!("{:?}", x))
        .unwrap()
        .map(|path| path.ostring().unwrap())
        .take(LIMIT)
        .collect();

    output
}

// todo: make a thing for the symbol table to check
//
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct SoundLawComposition {
    laws: Vec<SoundLaw>,
    final_fst: SoundFst,
    backwards_fst: SoundFst,
}

impl Default for SoundLawComposition {
    fn default() -> Self {
        Self::new()
    }
}

// todo fix memory so I stop cloning
#[wasm_bindgen]
impl SoundLawComposition {
    pub fn clear(&mut self) {
        self.laws = vec![];
    }
    pub fn append(&mut self, other: SoundLawComposition) {
        for law in other.laws {
            self.add_law(&law);
        }
    }
    pub fn new() -> SoundLawComposition {
        console_error_panic_hook::set_once();
        let fst: SoundVec = SoundVec::new();
        SoundLawComposition {
            laws: vec![],
            final_fst: SoundFst(fst),
            backwards_fst: SoundFst(SoundVec::new()),
        }
    }
    pub fn recompute_fsts(&mut self) -> bool {
        if self.laws.is_empty() {
            return false;
        }

        let mut total_fst = self.laws.first().unwrap().fst.clone();

        for new_fst in self.laws.iter().skip(1) {
            total_fst.compose(&new_fst.fst);
        }
        self.final_fst = total_fst;
        self.final_fst.optimize();
        self.backwards_fst = self.final_fst.clone();
        self.backwards_fst.invert();

        true
    }

    pub fn insert(&mut self, index: usize, law: &SoundLaw) {
        self.laws.insert(index, law.clone());
        self.recompute_fsts();
    }

    pub fn rm_law(&mut self, index: usize) -> SoundLaw {
        let rm = self.laws.remove(index);
        self.recompute_fsts();
        rm
    }
    pub fn add_law(&mut self, law: &SoundLaw) {
        self.laws.push(law.clone());
        if self.laws.len() == 1 {
            self.final_fst = law.fst.clone();
            self.backwards_fst = law.fst.clone();
            self.backwards_fst.invert();
        } else {
            self.final_fst.compose(law.get_fst());
            self.final_fst.optimize();
            self.backwards_fst = self.final_fst.clone();
            self.backwards_fst.invert();
        }
        let arc = Arc::new(law.get_table().clone());
        // TODO: do something smarter than this
        self.final_fst.0.set_input_symbols(arc.clone());
        self.final_fst.0.set_output_symbols(arc.clone());
    }

    pub fn transduce_text(&self, text: &str) -> Vec<String> {
        match self.laws.first() {
            None => vec![],
            Some(_) => {
                let table = self.final_fst.0.input_symbols().unwrap();
                transduce_text_with_symbol_table(&self.final_fst, table, text)
            }
        }
    }
    pub fn transduce_text_invert(&self, text: &str) -> Vec<String> {
        match self.laws.first() {
            None => vec![],
            Some(_) => {
                let table = self.final_fst.0.input_symbols().unwrap();
                transduce_text_with_symbol_table(&self.backwards_fst, table, text)
            }
        }
    }
}

#[wasm_bindgen]
pub fn soundlaw_xsampa_to_ipa(s: &str) -> String {
    xsampa_to_ipa(s)
}

#[cfg(test)]
mod tests {
    use std::{env::consts::OS, fmt::Debug};

    use rustfst::symt;

    use crate::tables::{ipa, xsampa_ascii};

    use super::*;

    #[test]
    fn test_labels_from_string() {
        let table = symt!["a", "b", "c"];

        let transduced = get_labels_from_str("abba", &table);
        assert_eq!(transduced, Some(vec![1, 2, 2, 1]));
    }

    #[test]
    fn right_arrow_test_with_soundlaw() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let law = SoundLaw::new("ca", "d", "ca", "c", &symbol_tabl);

        // let input1: SoundVec = fst![3, 1, 3, 1, 3, 1, 3]; // "cacacac"

        let transduced = law.transduce_text("cacacac");

        assert_eq!(transduced[0], "c a d d c");
    }

    #[test]
    fn compose_test() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let law1 = SoundLaw::new("a", "b", "", "", &symbol_tabl);
        let law2 = SoundLaw::new("b", "c", "", "", &symbol_tabl);

        let mut compose = SoundLawComposition::new();
        compose.add_law(&law1);
        compose.add_law(&law2);

        let transduced = compose.transduce_text("a");
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0], "c");

        let transduced = compose.transduce_text("b");
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0], "c");

        let transduced = compose.transduce_text_invert("c");
        assert_eq!(transduced.len(), 3);
        assert!(transduced.contains(&"c".to_owned()));
        assert!(transduced.contains(&"b".to_owned()));
        assert!(transduced.contains(&"a".to_owned()));
    }
    #[test]
    fn compose_test_rm() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let law1 = SoundLaw::new("a", "b", "", "", &symbol_tabl);
        let law2 = SoundLaw::new("b", "c", "", "", &symbol_tabl);

        let mut compose = SoundLawComposition::new();
        compose.add_law(&law1);
        compose.add_law(&law2);
        compose.rm_law(0);

        let transduced = compose.transduce_text("a");
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0], "a");

        let transduced = compose.transduce_text("b");
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0], "c");
    }
    #[test]
    fn compose_test_insert() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let law1 = SoundLaw::new("a", "b", "", "", &symbol_tabl);
        let law2 = SoundLaw::new("c", "d", "", "", &symbol_tabl);
        let law3 = SoundLaw::new("b", "c", "", "", &symbol_tabl);

        let mut compose = SoundLawComposition::new();
        compose.add_law(&law1);
        compose.add_law(&law2);

        compose.insert(1, &law3);

        let transduced = compose.transduce_text("a");
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0], "d");

        let transduced = compose.transduce_text("b");
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0], "d");

        let transduced = compose.transduce_text_invert("d");
        assert_eq!(transduced.len(), 4);
        //assert_eq!(transduced[0], "c");
    }
    #[test]
    fn compose_test_overflow() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let law1 = SoundLaw::new("a", "b", "", "", &symbol_tabl);
        let law2 = SoundLaw::new("c", "d", "", "", &symbol_tabl);
        let law3 = SoundLaw::new("b", "c", "", "", &symbol_tabl);

        let mut compose = SoundLawComposition::new();
        compose.add_law(&law1);
        compose.add_law(&law2);
        compose.insert(1, &law3);

        compose.backwards_fst.optimize();
        compose.backwards_fst.df("overflow");
        let transduced = compose.transduce_text_invert("dddddd");
        assert_eq!(transduced.len(), LIMIT);
    }

    #[test]
    fn small_cons_disjunction_small_symbol_table() {
        //let symbol_tabl = xsampa_ascii();
        let symbol_tabl = symt!["p", "t", "k", "b", "d", "g", "h", "_", "2", "e", "r", "a"];
        //let strings = vec!["p", "t", "k", "b", "d", "g", "b_h", "d_h", "g_h"];
        let strings = vec!["p", "t", "k", "b", "d", "g"];

        let fst = SoundLaw::disjunction_vec_fst(&strings, &symbol_tabl);

        let law =
            SoundLaw::new_with_vec_context("h_2", "a", fst.clone(), fst.clone(), &symbol_tabl);
        assert_eq!(law.transduce_text("ph_2ter")[0], "p a t e r");
    }
    #[test]
    fn small_cons_disjunction_xsampa() {
        let symbol_tabl = xsampa_ascii();
        //let strings = vec!["p", "t", "k", "b", "d", "g", "b_h", "d_h", "g_h"];
        let strings = vec!["p", "t", "k", "b", "d", "g"];

        let fst = SoundLaw::disjunction_vec_fst(&strings, &symbol_tabl);

        let law =
            SoundLaw::new_with_vec_context("h_2", "a", fst.clone(), fst.clone(), &symbol_tabl);
        assert_eq!(law.transduce_text("ph_2ter")[0], "p a t e r");
    }
    #[test]
    fn med_cons_disjunction_xsampa() {
        let symbol_tabl = xsampa_ascii();
        let strings = vec!["p", "t", "k", "b", "d", "g", "q", "r", "s"];

        let fst = SoundLaw::disjunction_vec_fst(&strings, &symbol_tabl);

        let law =
            SoundLaw::new_with_vec_context("h_2", "a", fst.clone(), fst.clone(), &symbol_tabl);
        assert_eq!(law.transduce_text("ph_2ter")[0], "p a t e r");
    }

    #[test]
    fn arbitrary_regex_test_simple1() {
        let table = ipa();
        let a = RegexFst::new_from_ipa("a".into());
        a.to_sound_fst().df("regex");

        let b = RegexFst::new_from_ipa("b".into());
        let x = RegexFst::new_from_ipa("x".into());
        let y = RegexFst::new_from_ipa("y".into());
        let law = SoundLaw::create_with_arbitrary_regex(&a, &b, &x, &y, &table);
        let transduced = law.transduce_text("axb");
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0], "a y b");
    }
    #[test]
    fn simple_character_to_epsilon_test() {
        let table = ipa();
        let left = RegexFst::new_from_ipa("a".into());
        let right = RegexFst::new_from_ipa("b".into());
        let from = RegexFst::new_from_ipa("x".into());
        let to = RegexFst::new_from_ipa("".into());

        let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &table);
        let transduced = law.transduce_text(&xsampa_to_ipa("axb"));
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0].replace(" ", ""), xsampa_to_ipa("ab"));
    }

    #[test]
    fn simple_epsilon_to_character_test() {
        let table = ipa();
        let left = RegexFst::new_from_ipa("a".into());
        let right = RegexFst::new_from_ipa("b".into());
        let from = RegexFst::new_from_ipa("".into());
        let to = RegexFst::new_from_ipa("x".into());

        let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &table);
        let transduced = law.transduce_text(&xsampa_to_ipa("ab"));
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0].replace(" ", ""), xsampa_to_ipa("abx"));
    }
    #[test]
    fn serialize_sound_law() {
        let symbol_tabl = symt!["a", "b", "c", "d"];
        let law1 = SoundLaw::new("ca", "d", "ca", "c", &symbol_tabl);

        let json = serde_json::to_string(&law1).expect("serialize should work");

        let mut back_to_law: SoundLaw = serde_json::from_str(&json).expect("deserialized crash");
        back_to_law.table = symbol_tabl;

        // let input1: SoundVec = fst![3, 1, 3, 1, 3, 1, 3]; // "cacacac"

        let transduced = back_to_law.transduce_text("cacacac");

        assert_eq!(transduced[0], "c a d d c");
    }
}
