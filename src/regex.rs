use rustfst::{
    prelude::{concat::concat, union::union, VectorFst},
    utils::acceptor,
    Semiring, SymbolTable,
};
use wasm_bindgen::prelude::*;

use crate::{
    cross_product::cross_product,
    sound_law::SoundLaw,
    tables::{ipa, xsampa_ascii},
    trans::{SoundFst, SoundVec, SoundWeight},
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum RegexOperator {
    Acceptor(String), // use this as the basic operator vs basing it on just a single character
    Star(Box<RegexOperator>),
    Concat(Box<RegexOperator>, Box<RegexOperator>),
    Union(Box<RegexOperator>, Box<RegexOperator>), // empty may be uneeded?
}

impl RegexOperator {
    fn to_string(&self) -> String {
        match self {
            RegexOperator::Acceptor(x) => x.to_string(),
            RegexOperator::Star(regex_operator) => format!("({})*", regex_operator.to_string()),
            RegexOperator::Concat(regex_operator, regex_operator1) => format!(
                "{}{}",
                regex_operator.to_string(),
                regex_operator1.to_string()
            ),
            RegexOperator::Union(regex_operator, regex_operator1) => format!(
                "({} + {})",
                regex_operator.to_string(),
                regex_operator1.to_string()
            ),
        }
    }
}

//change this to not a new type that also contains information for how it was created
#[wasm_bindgen]
#[derive(Debug)]
pub struct RegexFst {
    fst: VectorFst<SoundWeight>,
    operator: RegexOperator,
}
#[wasm_bindgen]
impl RegexFst {
    pub fn concat(&mut self, other: &RegexFst) {
        let _ = concat(&mut self.fst, &other.fst);
    }
    pub fn disjoint(&mut self, other: &RegexFst) {
        // I don't know why I chose a different name but whatever
        let _ = union(&mut self.fst, &other.fst);
    }

    pub fn to_string(&self) -> String {
        // I don't know why I chose a different name but whatever
        "implement this later".into()
    }

    //implement the rest later
    pub fn kleen(&mut self) {
        unimplemented!();
    }

    //quetionable interface, maybe I should wrap the table
    pub fn new_from_ipa(v: String) -> RegexFst {
        let table = ipa();

        // todo implement error handling
        let v: Vec<_> = v
            .chars()
            .map(|c| {
                table
                    .get_label(c.to_string())
                    .expect("Failed to find character in ipa table")
            })
            .collect();

        let acceptor: SoundVec = acceptor(&v, SoundWeight::one());
        RegexFst { fst: acceptor }
    }
}

impl RegexFst {
    pub fn to_sound_fst(&self) -> SoundFst {
        todo!()
    }
    pub fn regex_cross_product(a: &RegexFst, b: &RegexFst, table: &SymbolTable) -> SoundFst {
        SoundFst(cross_product(&a.fst, &b.fst, table))
    }
}
