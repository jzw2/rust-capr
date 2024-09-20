use crate::trans::SoundFst;
use rustfst::{
    prelude::{
        compose::compose, determinize::determinize, rm_epsilon::rm_epsilon, CoreFst, Fst,
        MutableFst, ProbabilityWeight, SerializableFst, StateIterator,
    },
    utils::acceptor,
    Label, Semiring, SymbolTable,
};

pub fn negate_with_symbol_table(fst: &SoundFst, alphabet: &SymbolTable) -> SoundFst {
    let label_vec: Vec<_> = alphabet.labels().collect();
    negate(fst, &label_vec)
}
pub fn negate(fst: &SoundFst, alphabet: &[Label]) -> SoundFst {
    // assumet that the fst is deterministic, and also acceptor or whatever that is aka is a regex

    // also destroys weights
    let mut ret = fst.clone();
    rm_epsilon(&mut ret).unwrap();
    let mut ret: SoundFst = determinize(&ret).unwrap();

    let accept = ret.add_state();

    let fst = ret.clone();

    for state in ret.states_iter() {
        dbg!(state);
        if fst.is_final(state).unwrap() {
            let _ = ret.set_final(state, ProbabilityWeight::zero());
        } else {
            let _ = ret.set_final(state, ProbabilityWeight::one());
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
    ret.set_final(accept, ProbabilityWeight::one()).unwrap();

    if ret.start().is_none() {
        ret.set_start(accept).unwrap();
    }

    ret
}

fn accepts(fst: &SoundFst, string: &[Label]) -> bool {
    let accept: SoundFst = acceptor(string, ProbabilityWeight::one());
    let composed: SoundFst = compose(accept, fst.clone()).expect("Error in composition");
    composed.draw("accepts.out", &Default::default()).unwrap();
    composed.paths_iter().next().is_some()
}
#[cfg(test)]
mod tests {
    use rustfst::{
        fst,
        prelude::{determinize::determinize, rm_epsilon::rm_epsilon},
        Tr,
    };

    use crate::{
        negate::{accepts, negate},
        trans::SoundFst,
    };

    use super::*;

    #[test]
    fn negate_test1() {
        let fst = fst![1, 2, 3];

        let negate = negate(&fst, &[1, 2, 3]);

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
        rustfst::algorithms::union::union(&mut fst1, &mut fst2).unwrap();

        let mut det_union_fst = determinize(&fst1).unwrap();
        rm_epsilon(&mut det_union_fst).unwrap();

        let negate_fst = negate(&det_union_fst, &alpha);
        //:dbg!(negate_fst.get_trs(8).unwrap().len());
        //negate_fst.draw("image.txt", &DrawingConfig::default());

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

    #[test]
    fn negate_test_simple_string() {
        // FST that accepts [1,2,3]
        let fst: SoundFst = fst![1, 2, 3];
        let alpha = vec![1, 2, 3, 4, 5, 6];

        let mut det_fst = determinize(&fst).unwrap();
        rm_epsilon(&mut det_fst).unwrap();

        let negate_fst = negate(&det_fst, &alpha);

        let input = vec![1, 2, 3];
        assert!(accepts(&det_fst, &input));
        assert!(!accepts(&negate_fst, &input));

        let other_input = vec![4, 5, 6];
        assert!(!accepts(&det_fst, &other_input));
        assert!(accepts(&negate_fst, &other_input));
    }
    #[test]
    fn negate_test_empty_string() {
        // FST that accepts the empty string
        let mut fst: SoundFst = SoundFst::new();
        let s0 = fst.add_state();
        fst.set_start(s0).unwrap();
        fst.set_final(s0, ProbabilityWeight::one()).unwrap();

        let alpha = vec![1];

        let mut det_fst = determinize(&fst).unwrap();
        rm_epsilon(&mut det_fst).unwrap();

        let negate_fst = negate(&det_fst, &alpha);

        let empty_input: Vec<Label> = vec![];
        let non_empty_input = vec![1];

        assert!(accepts(&det_fst, &empty_input));
        assert!(!accepts(&det_fst, &non_empty_input));

        assert!(!accepts(&negate_fst, &empty_input));
        assert!(accepts(&negate_fst, &non_empty_input));
    }
    #[test]
    fn negate_test_all_strings() {
        // FST that accepts all strings over the alphabet
        let mut fst: SoundFst = SoundFst::new();
        let s0 = fst.add_state();
        fst.set_start(s0).unwrap();
        fst.set_final(s0, ProbabilityWeight::one()).unwrap();
        let alpha = vec![1, 2, 3];

        for &label in &alpha {
            fst.add_tr(s0, Tr::new(label, label, ProbabilityWeight::one(), s0))
                .unwrap();
        }

        let mut det_fst = determinize(&fst).unwrap();
        rm_epsilon(&mut det_fst).unwrap();

        let negate_fst = negate(&det_fst, &alpha);

        let any_input = vec![1, 2, 3];
        let empty_input: Vec<Label> = vec![];

        assert!(accepts(&det_fst, &any_input));
        assert!(accepts(&det_fst, &empty_input));

        assert!(!accepts(&negate_fst, &any_input));
        assert!(!accepts(&negate_fst, &empty_input));
    }
    #[test]
    fn negate_test_no_strings() {
        // FST that accepts no strings
        let mut fst: SoundFst = SoundFst::new();
        let s0 = fst.add_state();
        fst.set_start(s0).unwrap();
        // No final states are set
        let alpha = vec![1, 2, 3];

        let mut det_fst = determinize(&fst).unwrap();
        rm_epsilon(&mut det_fst).unwrap();

        let negate_fst = negate(&det_fst, &alpha);
        dbg!(&negate_fst);
        dbg!(&det_fst);
        //negate_fst.draw("negate_test.txt", &DrawingConfig::default());

        let any_input = vec![1, 2, 3];
        let empty_input: Vec<Label> = vec![];

        assert!(!accepts(&det_fst, &any_input));
        assert!(!accepts(&det_fst, &empty_input));

        assert!(accepts(&negate_fst, &any_input));
        assert!(accepts(&negate_fst, &empty_input));
    }
}
