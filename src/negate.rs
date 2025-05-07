use crate::trans::{SoundFst, SoundWeight};
use rustfst::{
    algorithms::rm_epsilon::rm_epsilon,
    prelude::{
        determinize::determinize,
        encode::{decode, encode},
        minimize, tr_sort, CoreFst, ILabelCompare, MutableFst, OLabelCompare, StateIterator,
    },
    Label, Semiring, SymbolTable,
};

impl SoundFst {
    pub fn negate_with_symbol_table(&self, alphabet: &SymbolTable) -> Self {
        self.negate(&alphabet.labels().collect::<Vec<_>>())
    }

    pub fn negate(&self, alphabet: &[Label]) -> Self {
        self.d(line!());
        let mut ret = self.clone();
        //ret.reverse(); // sfst reverses twice for some reason
        //ret.reverse();
        ret.d(line!());
        println!("{} Removing epsilons", line!());
        rm_epsilon(&mut ret.0).unwrap();

        tr_sort(&mut ret.0, OLabelCompare {}); // I don't think this changes anything, but hfst has this
                                               // hfst also has removing the weights from the vector, but I only have one weight
        ret.d(line!());
        println!("{} Determinizing", line!());
        let encoded = encode(
            &mut ret.0,
            rustfst::prelude::encode::EncodeType::EncodeWeightsAndLabels,
        )
        .unwrap();
        tr_sort(&mut ret.0, ILabelCompare {}); // I don't think this changes anything, but hfst has this
        let mut ret: SoundFst = SoundFst(determinize(&ret.0).unwrap());
        //ret.d(line!());
        println!("{} Minimizing", line!());
        minimize(&mut ret.0).unwrap();
        decode(&mut ret.0, encoded).unwrap();
        ret.d(line!());
        let accept = ret.0.add_state();

        let fst = ret.clone();
        for state in fst.0.states_iter() {
            //dbg!(state);
            if fst.0.is_final(state).unwrap() {
                let _ = ret.0.delete_final_weight(state);
            } else {
                let _ = ret.0.set_final(state, SoundWeight::one());
            }
            alphabet
                .iter()
                .filter(|label| {
                    **label != 0
                        && fst
                            .0
                            .get_trs(state)
                            .unwrap()
                            .iter()
                            .all(|tr| tr.ilabel != **label)
                })
                .for_each(|label| {
                    //dbg!(label);
                    ret.0
                        .emplace_tr(state, *label, *label, SoundWeight::one(), accept)
                        .expect("unable to add label");
                    //dbg!(ret.get_trs(state).unwrap().len());
                });

            //dbg!(state);
        }
        ret.0.set_final(accept, SoundWeight::one()).unwrap();

        if ret.0.start().is_none() {
            ret.0.set_start(accept).unwrap();
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use rustfst::{
        fst,
        fst_impls::VectorFst,
        prelude::{compose::compose, concat::concat, Fst, SerializableFst},
        utils::{acceptor, epsilon_machine},
        Tr,
    };

    use crate::trans::{SoundFst, SoundVec, SoundWeight};

    use super::*;
    fn accepts(fst: &SoundFst, string: &[Label]) -> bool {
        // might be easier to directly check if the path is included within the string
        let accept: VectorFst<SoundWeight> = acceptor(string, SoundWeight::one());
        let composed: VectorFst<SoundWeight> =
            compose(accept, fst.0.clone()).expect("Error in composition");
        composed
            .draw("images/accepts.out", &Default::default())
            .unwrap();
        composed.paths_iter().next().is_some()
    }

    #[test]
    fn negate_test1() {
        let fst: SoundFst = SoundFst(fst![1, 2, 3]);

        let negate = fst.negate(&[1, 2, 3]);

        let str = vec![1, 2, 3];
        assert!(accepts(&fst, &str));
        assert!(!accepts(&negate, &str));
    }
    #[test]
    fn negate_test_multiple_strings() {
        // FST that accepts [1,2,3] and [4,5,6]
        let mut fst1: SoundFst = SoundFst(fst![1, 2, 3]);
        let fst2: SoundFst = SoundFst(fst!(4, 5, 6));
        let alpha = vec![1, 2, 3, 4, 5, 6];
        fst1.union(&fst2);

        let det_union_fst: SoundFst = fst1;

        let negate_fst = SoundFst::negate(&det_union_fst, &alpha);
        //:dbg!(negate_fst.get_trs(8).unwrap().len());
        //negate_fst.draw("image.txt", &DrawingConfig::default());
        negate_fst.d(line!());

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
        let fst: SoundVec = fst![1, 2, 3];
        let alpha = vec![1, 2, 3, 4, 5, 6];

        let det_fst: SoundFst = SoundFst(fst);

        let negate_fst = SoundFst::negate(&det_fst, &alpha);

        let input = vec![1, 2, 3];
        assert!(accepts(&det_fst, &input));
        assert!(!accepts(&negate_fst, &input));

        let other_input = vec![4, 5, 6];
        assert!(!accepts(&det_fst, &other_input));
        assert!(accepts(&negate_fst, &other_input));
    }
    #[test]
    fn negate_test_ignore_0_label() {
        // FST that accepts [1,2,3]
        let fst: SoundVec = fst![1, 2, 3];
        let fst = SoundFst(fst);
        let alpha = vec![1, 2, 3, 4, 5, 6];
        let alpha2 = vec![0, 1, 2, 3, 4, 5, 6];

        let negate_fst1 = SoundFst::negate(&fst, &alpha);
        let negate_fst2 = SoundFst::negate(&fst, &alpha2);

        assert_eq!(negate_fst1, negate_fst2)
    }
    #[test]
    fn negate_test_empty_string() {
        // FST that accepts the empty string
        let mut fst: SoundVec = SoundVec::new();
        let s0 = fst.add_state();
        fst.set_start(s0).unwrap();
        fst.set_final(s0, SoundWeight::one()).unwrap();

        let alpha = vec![1];

        let det_fst: SoundFst = SoundFst(fst);

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
        let mut fst: SoundVec = SoundVec::new();
        let s0 = fst.add_state();
        fst.set_start(s0).unwrap();
        fst.set_final(s0, SoundWeight::one()).unwrap();
        let alpha = vec![1, 2, 3];

        for &label in &alpha {
            fst.add_tr(s0, Tr::new(label, label, SoundWeight::one(), s0))
                .unwrap();
        }

        let det_fst: SoundFst = SoundFst(fst);

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
        let mut fst: SoundVec = SoundVec::new();
        let s0 = fst.add_state();
        fst.set_start(s0).unwrap();
        // No final states are set
        let alpha = vec![1, 2, 3];

        let det_fst: SoundFst = SoundFst(fst);

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
        let mut fst: SoundVec = fst![1];
        let alpha = vec![1];
        fst.emplace_tr(0, 1, 1, SoundWeight::one(), 0).unwrap();
        fst.emplace_tr(1, 1, 1, SoundWeight::one(), 1).unwrap();

        let fst = SoundFst(fst);

        let negate_fst = fst.negate(&alpha);
        dbg!(&negate_fst);

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
        let mut fst: SoundVec = fst![1];
        let alpha = vec![1, 2];
        fst.emplace_tr(0, 1, 1, SoundWeight::one(), 0).unwrap();
        fst.emplace_tr(1, 1, 1, SoundWeight::one(), 1).unwrap();

        let fst = SoundFst(fst);

        let negate_fst = fst.negate(&alpha);
        dbg!(&negate_fst);

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

    #[test]
    fn star_then_star_4_alphabet() {
        // FST that accepts no strings
        let fst: SoundVec = fst![1, 2, 3];
        let alpha = vec![1, 2, 3, 4];
        let mut star: SoundVec = epsilon_machine().unwrap();
        for letter in alpha.iter() {
            let letter = *letter;
            star.emplace_tr(0, letter, letter, SoundWeight::one(), 0)
                .unwrap();
        }

        let star2 = star.clone();
        concat(&mut star, &fst).unwrap();
        concat(&mut star, &star2).unwrap();

        let star = SoundFst(star);

        let negate_fst = star.negate(&alpha);
        dbg!(&negate_fst);

        let fst = star;

        let just1 = vec![1, 2, 3];
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
