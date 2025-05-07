use rustfst::{fst, prelude::VectorFst, utils::acceptor, Label, Semiring};

use crate::{
    tables::single_character_class,
    trans::{SoundFst, SoundWeight},
};

//does the replace thing
// old class that was used for the special case of
// creating a speed up when doing single characters and disjunctions
fn single_character_class_fst_context(
    left_context_marker: Label,
    right_context_marker: Label,
) -> SoundFst {
    let table = single_character_class();
    //let b = acceptor(labels, weight);
    let fst: VectorFst<SoundWeight> = fst![1];
    let sound_fst = SoundFst(fst);
    //sound_fst.replace_context(left_context_marker, right_context_marker, table);

    todo!()
}
