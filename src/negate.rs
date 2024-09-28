use crate::trans::{FstTraits, SoundFst, SoundWeight};
use rustfst::{
    prelude::{
        determinize::{determinize, determinize_with_config, DeterminizeConfig, DeterminizeType}, rm_epsilon::rm_epsilon, CoreFst, MutableFst, ProbabilityWeight,
        StateIterator,
    }, DrawingConfig, Label, Semiring, SymbolTable
};

pub trait SoundFstNegateTrait: FstTraits + for<'a> StateIterator<'a> {
    fn negate_with_symbol_table(&self, alphabet: &SymbolTable) -> Self {
        self.negate(&alphabet.labels().collect::<Vec<_>>())
    }

    fn negate(&self, alphabet: &[Label]) -> Self {
        // assumet that the fst is deterministic, and also acceptor or whatever that is aka is a regex

        dbg!(&self);
        // also destroys weights
        self.draw("images/image.txt", &DrawingConfig::default()).unwrap();
        println!("draing text");
        let mut ret = self.clone();
        dbg!(&ret);
        rm_epsilon(&mut ret).unwrap();
        println!("removed espslon");
        self.draw("images/image_rm.txt", &DrawingConfig::default()).unwrap();
        let mut ret: Self = determinize_with_config(&ret, DeterminizeConfig { det_type: DeterminizeType::DeterminizeNonFunctional, ..Default::default()} ).unwrap();
        println!("determinized");
        let accept = ret.add_state();

        let fst = ret.clone();
        for state in fst.states_iter() {
            dbg!(state);
            if fst.is_final(state).unwrap() {
                let _ = ret.set_final(state, SoundWeight::zero());
            } else {
                let _ = ret.set_final(state, SoundWeight::one());
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
                    ret.emplace_tr(state, *label, *label, SoundWeight::one(), accept)
                        .expect("unable to add label");
                    dbg!(ret.get_trs(state).unwrap().len());
                });

            dbg!(state);
        }
        ret.set_final(accept, SoundWeight::one()).unwrap();

        if ret.start().is_none() {
            ret.set_start(accept).unwrap();
        }

        ret
    }
}

impl SoundFstNegateTrait for SoundFst {}

#[cfg(test)]
mod tests {
    use rustfst::{
        fst,
        prelude::{
            compose::compose, determinize::determinize, rm_epsilon::rm_epsilon, Fst,
            SerializableFst,
        },
        utils::acceptor,
        Tr,
    };

    use crate::trans::{SoundFst, SoundWeight};

    use super::*;
    fn accepts(fst: &SoundFst, string: &[Label]) -> bool {
        // might be easier to directly check if the path is included within the string
        let accept: SoundFst = acceptor(string, SoundWeight::one());
        let composed: SoundFst = compose(accept, fst.clone()).expect("Error in composition");
        composed.draw("images/accepts.out", &Default::default()).unwrap();
        composed.paths_iter().next().is_some()
    }

    #[test]
    fn negate_test1() {
        let fst: SoundFst = fst![1, 2, 3];

        let negate = fst.negate(&[1, 2, 3]);

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

        let mut det_union_fst: SoundFst = determinize(&fst1).unwrap();
        rm_epsilon(&mut det_union_fst).unwrap();

        let negate_fst = SoundFst::negate(&det_union_fst, &alpha);
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

        let mut det_fst: SoundFst = determinize(&fst).unwrap();
        rm_epsilon(&mut det_fst).unwrap();

        let negate_fst = SoundFst::negate(&det_fst, &alpha);

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
        fst.set_final(s0, SoundWeight::one()).unwrap();

        let alpha = vec![1];

        let mut det_fst: SoundFst = determinize(&fst).unwrap();
        rm_epsilon(&mut det_fst).unwrap();

        let negate_fst = det_fst.negate(&alpha);

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
        fst.set_final(s0, SoundWeight::one()).unwrap();
        let alpha = vec![1, 2, 3];

        for &label in &alpha {
            fst.add_tr(s0, Tr::new(label, label, SoundWeight::one(), s0))
                .unwrap();
        }

        let mut det_fst: SoundFst = determinize(&fst).unwrap();
        rm_epsilon(&mut det_fst).unwrap();

        let negate_fst = det_fst.negate(&alpha);

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

        let mut det_fst: SoundFst = determinize(&fst).unwrap();
        rm_epsilon(&mut det_fst).unwrap();

        let negate_fst = det_fst.negate(&alpha);
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

    #[test]
    fn star_then_star() {
        // FST that accepts no strings
        let mut fst: SoundFst = fst![1];
        let alpha = vec![1];
        fst.emplace_tr(0, 1, 1, SoundWeight::one(), 0).unwrap();
        fst.emplace_tr(1, 1, 1, SoundWeight::one(), 1).unwrap();


        let negate_fst = fst.negate(&alpha);
        dbg!(&negate_fst);
        negate_fst.draw("images/negate_test.txt", &DrawingConfig::default()).unwrap();

        let just1 = vec![1];
        assert!(accepts(&fst, &just1));
        assert!(!accepts(&negate_fst, &just1));
        let nothing: Vec<u32> = vec![];
        assert!(!accepts(&fst, &nothing));
        assert!(accepts(&negate_fst, &nothing));
    }

    #[test]
    fn star_then_star_2_alphabet() {
        // FST that accepts no strings
        let mut fst: SoundFst = fst![1];
        let alpha = vec![1,2];
        fst.emplace_tr(0, 1, 1, SoundWeight::one(), 0).unwrap();
        fst.emplace_tr(1, 1, 1, SoundWeight::one(), 1).unwrap();


        let negate_fst = fst.negate(&alpha);
        dbg!(&negate_fst);
        negate_fst.draw("images/negate_test.txt", &DrawingConfig::default()).unwrap();

        let just1 = vec![1];
        assert!(accepts(&fst, &just1));
        assert!(!accepts(&negate_fst, &just1));
        let nothing: Vec<u32> = vec![];
        assert!(!accepts(&fst, &nothing));
        assert!(accepts(&negate_fst, &nothing));
        let just2: Vec<u32> = vec![2];
        assert!(!accepts(&fst, &just2));
        assert!(accepts(&negate_fst, &just2));
    }



}
