use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use rustfst::{
    prelude::{
        closure::{closure, ClosureType},
        concat::concat,
        optimize,
        union::union,
        Fst,
    },
    utils::acceptor,
    Label, Semiring, SymbolTable,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    cross_product::cross_product,
    tables::ipa,
    trans::{SoundFst, SoundVec, SoundWeight},
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
enum RegexOperator {
    Acceptor(String), // use this as the basic operator vs basing it on just a single character
    SingleLabel(Label),
    Star(Box<RegexOperator>),
    Concat(Box<RegexOperator>, Box<RegexOperator>),
    Union(Box<RegexOperator>, Box<RegexOperator>), // empty may be uneeded?
}

impl Display for RegexOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = match self {
            RegexOperator::Acceptor(x) => x.to_string(),
            RegexOperator::Star(regex_operator) => format!("({})*", regex_operator),
            RegexOperator::Concat(regex_operator, regex_operator1) => {
                format!("{}{}", regex_operator, regex_operator1)
            }
            // todo fix this so that it doesn't make redundant parenthesis
            RegexOperator::Union(regex_operator, regex_operator1) => {
                format!("({} + {})", regex_operator, regex_operator1)
            }
            RegexOperator::SingleLabel(s) => s.to_string(),
        };
        write!(f, "{}", s)
    }
}

//change this to not a new type that also contains information for how it was created
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexFst {
    fst: SoundFst,
    operator: RegexOperator,
}
#[wasm_bindgen]
impl RegexFst {
    // not sure if including json in here is the best in terms of design
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Unwrap json string failed")
    }

    pub fn from_json(s: &str) -> Self {
        // just asume an ipa for now
        let mut ret: Self = serde_json::from_str(s).expect("Unwrap json string failed");
        let table = Arc::new(ipa());
        ret.fst.0.set_input_symbols(Arc::clone(&table));
        ret.fst.0.set_output_symbols(Arc::clone(&table));
        ret
    }

    // because wasmbindgen is stupid
    pub fn dup(&self) -> Self {
        self.clone()
    }

    pub fn concat(&mut self, other: &RegexFst) {
        let _ = concat(&mut self.fst.0, &other.fst.0);
        self.operator = RegexOperator::Concat(
            Box::new(self.operator.clone()),
            Box::new(other.operator.clone()),
        )
    }
    pub fn disjoint(&mut self, other: &RegexFst) {
        // I don't know why I chose a different name but whatever
        let _ = union(&mut self.fst.0, &other.fst.0);
        self.operator = RegexOperator::Union(
            Box::new(self.operator.clone()),
            Box::new(other.operator.clone()),
        )
    }

    //implement the rest later
    pub fn kleen(&mut self) {
        closure(&mut self.fst.0, ClosureType::ClosureStar);
        self.operator = RegexOperator::Star(Box::new(self.operator.clone()))
    }

    // might be redudant to use both strings and labels
    pub fn new_from_label(label: Label) -> Self {
        let acceptor: SoundVec = acceptor(&[label], SoundWeight::one());
        RegexFst {
            fst: SoundFst(acceptor),
            operator: RegexOperator::SingleLabel(label),
        }
    }

    //quetionable interface, maybe I should wrap the table
    pub fn new_from_ipa(s: String) -> RegexFst {
        let table = ipa();

        // todo implement error handling
        let v: Vec<_> = s
            .chars()
            .map(|c| {
                table
                    .get_label(c.to_string())
                    .unwrap_or_else(|| panic!("Failed to find character {} in ipa table", &c))
            })
            .collect();

        let acceptor: SoundVec = acceptor(&v, SoundWeight::one());
        RegexFst {
            fst: SoundFst(acceptor),
            operator: RegexOperator::Acceptor(s),
        }
    }
    pub fn is_empty(&self) -> bool {
        if let RegexOperator::Acceptor(s) = self.operator.clone() {
            return s.is_empty();
        }
        false
    }

    pub fn string_form(&self) -> String {
        self.to_string()
    }
}

impl Display for RegexFst {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // I don't know why I chose a different name but whatever
        write!(f, "{}", self.operator)
    }
}

impl RegexFst {
    pub fn to_sound_fst(&self) -> SoundFst {
        let mut inner_fst = self.fst.clone();
        optimize(&mut inner_fst.0).unwrap(); // don't want a repeat of last time
        inner_fst
    }
    pub fn regex_cross_product(a: &RegexFst, b: &RegexFst, table: &SymbolTable) -> SoundFst {
        SoundFst(cross_product(&a.fst.0, &b.fst.0, table))
    }
}
