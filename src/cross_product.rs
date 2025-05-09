use core::panic;

use rustfst::{
    prelude::{
        closure::closure, compose::compose, concat::concat, invert, optimize, tr_sort,
        ILabelCompare, MutableFst, OLabelCompare, SerializableFst,
    },
    DrawingConfig, Label, Semiring, SymbolTable, EPS_LABEL,
};

use crate::{
    regex::RegexFst,
    sound_law::SoundLaw,
    trans::{SoundFst, SoundVec, SoundWeight},
};

fn any_to_single_label(table: &SymbolTable, single_label: Label) -> SoundVec {
    //table should not have the mark in it
    let mut fst = SoundVec::new();
    let state = fst.add_state();
    fst.set_final(state, SoundWeight::one()).unwrap();
    fst.set_start(state).unwrap();

    for (label, _) in table.iter() {
        if label != EPS_LABEL {
            // this is actually uneeded, beccause it gets plugged into the kleene star anyway
            fst.emplace_tr(state, label, single_label, SoundWeight::one(), state)
                .unwrap();
        }
    }
    fst.draw(
        format!("images/{}.dot", "single_label"),
        &DrawingConfig::default(),
    )
    .unwrap();
    println!("{:?}", fst);

    fst
}
fn loop_machine(input: Label, output: Label) -> SoundVec {
    // really dumb of me when I could have just used epsilon machine
    let mut fst = SoundVec::new();
    let state = fst.add_state();
    fst.set_final(state, SoundWeight::one()).unwrap();
    fst.set_start(state).unwrap();

    fst.emplace_tr(state, input, output, SoundWeight::one(), state)
        .unwrap();
    fst
}

//basically stolen from HfstTransducer
pub fn cross_product(a: &SoundVec, b: &SoundVec, table: &SymbolTable) -> SoundVec {
    let mut a = a.clone();
    let mut b = b.clone();
    tr_sort(&mut a, ILabelCompare {});
    tr_sort(&mut b, ILabelCompare {});
    // check to make sure it doens't do something bad

    //should probably extract that to a variable
    if table.contains_symbol("MARK") {
        panic!("Why did you add MARK as your symbol")
        //maybe include handling so that it doens't immdeidately crash
    }
    let mut table_with_mark = table.clone();
    let mark_label = table_with_mark.add_symbol("MARK");

    //requires expansion becuase I don't have "unknown"
    let mut any_to_mark: SoundVec = any_to_single_label(table, mark_label);
    tr_sort(&mut any_to_mark, ILabelCompare {});

    //loop machine is actually useless, becuae I put it in the closue anyway
    let mut epsilon_to_mark: SoundVec = loop_machine(EPS_LABEL, mark_label);
    tr_sort(&mut epsilon_to_mark, ILabelCompare {});

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
    any_to_mark
        .draw(format!("images/{}.dot", "atm"), &DrawingConfig::default())
        .unwrap();
    epsilon_to_mark
        .draw(format!("images/{}.dot", "etm"), &DrawingConfig::default())
        .unwrap();
    mark_to_any
        .draw(format!("images/{}.dot", "mta"), &DrawingConfig::default())
        .unwrap();
    mark_to_epsilon
        .draw(format!("images/{}.dot", "mte"), &DrawingConfig::default())
        .unwrap();

    tr_sort(&mut any_to_mark, ILabelCompare {});
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

    tr_sort(&mut left, ILabelCompare {});
    tr_sort(&mut left, OLabelCompare {});

    let mut ret: SoundVec = compose(left, right).unwrap();
    optimize(&mut ret).unwrap();
    ret.draw(format!("images/{}.dot", "ret"), &DrawingConfig::default())
        .unwrap();
    ret
}

// add tests or something
#[cfg(test)]
mod tests {

    use rustfst::{
        fst,
        prelude::{project, union::union, Fst, ProjectType},
        utils::acceptor,
    };

    use super::*;

    #[test]
    fn test_cross_product_basic() {
        // Create a simple symbol table.
        let mut table = SymbolTable::new();
        table.add_symbol("a"); // 1
        table.add_symbol("b"); // 2
        table.add_symbol("c"); // 3

        // Create two simple SoundVecs using union.
        let a: SoundVec = fst![1]; // s0 --a--> s1 (acceptor)
        let b: SoundVec = fst![2]; // s0 --b--> s1 (acceptor)

        // Call the cross_product function.
        let result = cross_product(&a, &b, &table);

        // Create an input FST for testing, e.g., "a"
        let input_fst: SoundVec = acceptor(&[1], SoundWeight::one());

        // Compose and project.
        let mut composed_fst: SoundVec = compose(input_fst, result).unwrap();
        project(&mut composed_fst, ProjectType::ProjectOutput);

        let output_strings: Vec<_> = composed_fst.paths_iter().collect();
        let expected_output_labels = vec![[2]];

        assert_eq!(
            output_strings
                .iter()
                .map(|p| p.olabels.as_slice().to_vec())
                .collect::<Vec<_>>(),
            expected_output_labels
        );
    }

    #[test]
    fn test_cross_product_nontrivial() {
        // Create a symbol table.
        let mut table = SymbolTable::new();
        table.add_symbol("a"); // 1
        table.add_symbol("b"); // 2
        table.add_symbol("c"); // 3
        table.add_symbol("d"); // 4
        table.add_symbol("e"); // 5
        table.add_symbol("f"); // 6
        table.add_symbol("g"); // 7

        // Define two more complex FSTs, a and b, using union.
        // a: s0 --a--> s1, s0 --c--> s2 (acceptor)
        let mut a: SoundVec = fst![1]; // s0 --a--> s1
        let a2: SoundVec = fst![3]; // s0 --c--> s2
        union(&mut a, &a2).unwrap();

        // b: s0 --d--> s1, s0 --f--> s2 (acceptor)
        let mut b: SoundVec = fst![4]; // s0 --d--> s1
        let b2: SoundVec = fst![6]; // s0 --f--> s2
        union(&mut b, &b2).unwrap();

        // Call the cross_product function.
        let result = cross_product(&a, &b, &table);

        // Create an input FST for testing.  For example, the input "ac"
        let input_fst: SoundVec = acceptor(&[1], SoundWeight::one()); // a c

        // Compose the result with the input.
        let mut composed_fst: SoundVec = compose(input_fst, result.clone()).unwrap();

        // Project the output labels.
        project(&mut composed_fst, ProjectType::ProjectOutput);

        // Get the output strings.
        let output_strings: Vec<_> = composed_fst.paths_iter().collect();
        let expected_output_labels = vec![[4], [6]]; // Expected output: "df"  ->  [4,6]

        assert_eq!(
            output_strings
                .iter()
                .map(|p| p.olabels.as_slice().to_vec())
                .collect::<Vec<_>>(),
            expected_output_labels
        );
        let input_fst: SoundVec = acceptor(&[3], SoundWeight::one()); // a c

        // Compose the result with the input.
        let mut composed_fst: SoundVec = compose(input_fst, result).unwrap();

        // Project the output labels.
        project(&mut composed_fst, ProjectType::ProjectOutput);

        // Get the output strings.
        let output_strings: Vec<_> = composed_fst.paths_iter().collect();
        let expected_output_labels = vec![[4], [6]]; // Expected output: "df"  ->  [4,6]

        assert_eq!(
            output_strings
                .iter()
                .map(|p| p.olabels.as_slice().to_vec())
                .collect::<Vec<_>>(),
            expected_output_labels
        );
    }

    #[test]
    fn test_any_to_single_label_basic() {
        let mut table = SymbolTable::new();
        table.add_symbol("a"); // 1
        table.add_symbol("b"); // 2
        table.add_symbol("c"); // 3
        let single_label: Label = 5;

        let result = any_to_single_label(&table, single_label);

        // Test by composing with an input string and checking the output.
        let input_fst: SoundVec = acceptor(&[1, 2, 3], SoundWeight::one());
        let mut composed_fst: SoundVec = compose(input_fst, result).unwrap();
        project(&mut composed_fst, ProjectType::ProjectOutput);
        let output_strings: Vec<_> = composed_fst.paths_iter().collect();

        // assert_eq!(paths[0].ilabels.as_slice(), &[1, 2, 3]);
        // assert_eq!(paths[0].olabels.as_slice(), &[4, 5]);
        assert_eq!(output_strings[0].olabels.as_slice(), &[5, 5, 5]);
    }
}
