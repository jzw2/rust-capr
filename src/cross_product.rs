use core::panic;

use rustfst::{
    prelude::{invert, optimize, MutableFst},
    Label, Semiring, SymbolTable, EPS_LABEL,
};

use crate::trans::{SoundVec, SoundWeight};

fn any_to_single_label(table: &SymbolTable, single_label: Label) -> SoundVec {
    let mut fst = SoundVec::new();
    let state = fst.add_state();
    fst.set_final(state, SoundWeight::one());

    for (label, _) in table.iter() {
        fst.emplace_tr(state, label, single_label, SoundWeight::one(), state);
    }

    fst
}

fn cross_product(a: &SoundVec, b: &SoundVec, table: &SymbolTable) -> SoundVec {
    // check to make sure it doens't do something bad

    if table.contains_symbol("MARK") {
        panic!("Why did you add MARK as your symbol")
    }
    let mut table_with_mark = table.clone();
    let mark_label = table_with_mark.add_symbol("MARK");

    let mut any_to_mark = any_to_single_label(table, mark_label);
    let mut any_to_epsilon = any_to_single_label(table, EPS_LABEL);
    let mut mark_to_any = any_to_mark.clone();
    invert(&mut mark_to_any);

    let mut epsilon_to_mark = any_to_epsilon.clone();
    invert(&mut epsilon_to_mark);

    optimize(&mut any_to_mark);
    optimize(&mut any_to_epsilon);
    optimize(&mut mark_to_any);
    optimize(&mut epsilon_to_mark);

    todo!()
}

// add tests or something
