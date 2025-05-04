use core::panic;
use std::f32::EPSILON;

use rustfst::{
    prelude::{
        closure::closure,
        compose::{self, compose},
        concat::concat,
        invert, optimize, MutableFst,
    },
    utils::epsilon_machine,
    Label, Semiring, SymbolTable, EPS_LABEL,
};
use wasm_bindgen::UnwrapThrowExt;

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
fn loop_machine(input: Label, output: Label) -> SoundVec {
    let mut fst = SoundVec::new();
    let state = fst.add_state();
    fst.set_final(state, SoundWeight::one());

    fst.emplace_tr(state, input, output, SoundWeight::one(), state);
    fst
}

//basically stolen from HfstTransducer
fn cross_product(a: &SoundVec, b: &SoundVec, table: &SymbolTable) -> SoundVec {
    // check to make sure it doens't do something bad

    if table.contains_symbol("MARK") {
        panic!("Why did you add MARK as your symbol")
    }
    let mut table_with_mark = table.clone();
    let mark_label = table_with_mark.add_symbol("MARK");

    let mut any_to_mark: SoundVec = any_to_single_label(table, mark_label);

    let mut epsilon_to_mark: SoundVec = loop_machine(EPS_LABEL, mark_label);

    let mut mark_to_any = any_to_mark.clone();
    invert(&mut mark_to_any);
    let mut mark_to_epsilon = epsilon_to_mark.clone();
    invert(&mut mark_to_epsilon);

    closure(
        &mut any_to_mark,
        rustfst::prelude::closure::ClosureType::ClosureStar,
    );
    closure(
        &mut epsilon_to_mark,
        rustfst::prelude::closure::ClosureType::ClosureStar,
    );
    closure(
        &mut mark_to_any,
        rustfst::prelude::closure::ClosureType::ClosureStar,
    );
    closure(
        &mut mark_to_epsilon,
        rustfst::prelude::closure::ClosureType::ClosureStar,
    );

    optimize(&mut any_to_mark).unwrap();
    optimize(&mut epsilon_to_mark).unwrap();
    optimize(&mut mark_to_any).unwrap();
    optimize(&mut mark_to_epsilon).unwrap();

    // called a1 in hfst
    let mut left: SoundVec = compose(a.clone(), any_to_mark).unwrap();
    optimize(&mut left).unwrap();
    concat(&mut left, &epsilon_to_mark).unwrap();
    optimize(&mut left).unwrap();

    // called b1
    let mut right: SoundVec = compose(mark_to_any.clone(), b.clone()).unwrap();
    optimize(&mut right).unwrap();
    concat(&mut right, &mark_to_epsilon).unwrap();
    optimize(&mut right).unwrap();

    let mut ret = compose(left, right).unwrap();
    optimize(&mut ret).unwrap();
    ret
}

// add tests or something
