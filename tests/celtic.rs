use ipa_translate::xsampa_to_ipa;
use rust_capr::{
    regex::RegexFst,
    sound_law::{SoundLaw, SoundLawComposition},
    tables::ipa,
};
use rustfst::SymbolTable;

// represent the laryngals as h, x, q

fn a1() -> SoundLawComposition {
    let data = common_setup();

    let mut from = RegexFst::new_from_ipa(xsampa_to_ipa("he"));

    let to = RegexFst::new_from_ipa("e".into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut from = RegexFst::new_from_ipa(xsampa_to_ipa("xe"));

    let to = RegexFst::new_from_ipa("a".into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut from = RegexFst::new_from_ipa(xsampa_to_ipa("qe"));

    let to = RegexFst::new_from_ipa("o".into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law3 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut total = SoundLawComposition::new();
    total.add_law(&law1);
    total.add_law(&law2);
    total.add_law(&law3);
    total
}
fn a2() -> SoundLawComposition {
    let data = common_setup();

    let mut from = RegexFst::new_from_ipa(xsampa_to_ipa("eh"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("e:").into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut from = RegexFst::new_from_ipa(xsampa_to_ipa("eq"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("a:"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut from = RegexFst::new_from_ipa(xsampa_to_ipa("eq"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("o:"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law3 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut total = SoundLawComposition::new();
    total.add_law(&law1);
    total.add_law(&law2);
    total.add_law(&law3);
    total
}

fn mini_consonants() -> Vec<&'static str> {
    "p l n r".split(' ').collect()
}

fn small_resonants() -> Vec<&'static str> {
    "l r".split(' ').collect()
}

#[test]
fn celtic_small_stop_resonant_laryngeal_stop() {
    let table = ipa();

    let consonants = xsampa_disjoint(&mini_consonants());
    let resonants = xsampa_disjoint(&small_resonants());
    let laryngeals = xsampa_disjoint(&pie_laryngeals()); // reuse full laryngeals

    let from = RegexFst::new_from_ipa("".into());
    let to = RegexFst::new_from_ipa("a".into());

    let mut left = consonants.clone();
    left.concat(&resonants);

    let mut right = laryngeals.clone();
    right.concat(&consonants);

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &table);

    for (input, expected) in [
        ("plhno", "plano"),
        ("grhno", "grano"),
        ("grxno", "grano"),
        ("grqno", "grano"),
    ] {
        let output = law.transduce_text(&xsampa_to_ipa(input));
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].replace(" ", ""), xsampa_to_ipa(expected));
    }
}
#[test]
fn celtic_laryngeal_to_a_between_cons() {
    let data = common_setup();

    let from = xsampa_disjoint(&["h", "x", "q"]);
    let to = RegexFst::new_from_ipa("a".into());

    let left = &data.consonants;
    let right = &data.consonants;

    let law = SoundLaw::create_with_arbitrary_regex(left, right, &from, &to, &data.table);

    let pxter = xsampa_to_ipa("pxter");
    let pater = law.transduce_text(&pxter);
    assert_eq!(pater.len(), 1);
    assert_eq!(pater[0].replace(" ", ""), xsampa_to_ipa("pater"));

    let d_hugxter = xsampa_to_ipa("d_hugxte:r");
    let daughter = law.transduce_text(&d_hugxter);
    assert_eq!(daughter.len(), 1);
    assert_eq!(daughter[0].replace(" ", ""), xsampa_to_ipa("d_hugate:r"));
}
fn grano_modified_test() {
    let data = common_setup();

    let from = RegexFst::new_from_ipa("h".into());
    let to = RegexFst::new_from_ipa("a".into());

    let mut left = data.consonants.clone();
    left.concat(&data.resonants);
    let right = data.consonants.clone();

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let transduced = law.transduce_text(&xsampa_to_ipa("grhno"));
    assert_eq!(transduced.len(), 1);
    assert_eq!(transduced[0].replace(" ", ""), xsampa_to_ipa("grano"));
}
#[test]
fn krissu() {
    let data = common_setup();

    let mut from = data.coronals.clone();
    from.concat(&data.coronals); // Important: concat, not extend

    let to = RegexFst::new_from_ipa("ss".into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let transduced = law.transduce_text(&xsampa_to_ipa("krdtu"));
    assert_eq!(transduced.len(), 1);
    assert_eq!(transduced[0].replace(" ", ""), xsampa_to_ipa("krssu"));
}

fn pie_stops() -> Vec<&'static str> {
    "p t k b d g b_h d_h g_h k_w g_w g_w_h".split(' ').collect()
}

fn pie_resonants() -> Vec<&'static str> {
    "m n l r".split(' ').collect()
}

fn pie_glides() -> Vec<&'static str> {
    "w y".split(' ').collect()
}

fn pie_laryngeals() -> Vec<&'static str> {
    "h x q".split(' ').collect()
}

fn pie_consonants() -> Vec<&'static str> {
    "p t k b d g b_h d_h g_h k_w g_w g_w_h m n l r w y"
        .split(' ')
        .collect()
}

fn coronal_stops() -> Vec<&'static str> {
    "t d d_h".split(' ').collect()
}

struct CommonData {
    table: SymbolTable,
    consonants: RegexFst,
    stops: RegexFst,
    resonants: RegexFst,
    laryngeals: RegexFst,
    coronals: RegexFst,
}

fn common_setup() -> CommonData {
    let table = ipa();
    let consonants = xsampa_disjoint(&pie_consonants());
    let stops = xsampa_disjoint(&pie_stops());
    let resonants = xsampa_disjoint(&pie_resonants());
    let laryngeals = xsampa_disjoint(&pie_laryngeals());
    let coronals = xsampa_disjoint(&coronal_stops());

    CommonData {
        table,
        consonants,
        stops,
        resonants,
        laryngeals,
        coronals,
    }
}
#[test]
fn celtic_stop_resonant_laryngeal_stop() {
    let table = ipa();

    let pie_stops = "p t k b d g b_h d_h g_h k_w g_w g_w_h"
        .split(" ")
        .collect::<Vec<_>>();
    let pie_resonants = "m n l r".split(" ").collect::<Vec<_>>();
    let pie_glides = "w y".split(" ").collect::<Vec<_>>();
    let pie_laryngeals = "h x q".split(" ").collect::<Vec<_>>();

    let pie_consonants = "p t k b d g b_h d_h g_h k_w g_w g_w_h m n l r w y"
        .split(" ")
        .collect::<Vec<_>>();

    let disjoint_stops = xsampa_disjoint(&pie_stops);
    let disjoint_consonants = xsampa_disjoint(&pie_consonants);
    let disjoint_laryngeals = xsampa_disjoint(&pie_laryngeals);
    let disjoint_resonants = xsampa_disjoint(&pie_resonants);

    let from = RegexFst::new_from_ipa("".into());
    let to = RegexFst::new_from_ipa("a".into());

    let mut left = disjoint_consonants.clone();
    left.concat(&disjoint_resonants);

    let mut right = disjoint_laryngeals.clone();
    right.concat(&disjoint_consonants);

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &table);

    for (input, expected) in [
        ("plhno", "plano"),
        ("grhno", "grano"),
        ("grxno", "grano"),
        ("grqno", "grano"),
    ] {
        let transduced = law.transduce_text(&xsampa_to_ipa(input));
        assert_eq!(transduced.len(), 1);
        assert_eq!(transduced[0].replace(" ", ""), xsampa_to_ipa(expected));
    }
}

#[test]
fn gwow() {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("g_w"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("b"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let transduced = law.transduce_text(&xsampa_to_ipa("g_wow"));
    assert_eq!(transduced.len(), 1);
    assert_eq!(transduced[0].replace(" ", ""), xsampa_to_ipa("bow"));
}

fn xsampa_disjoint(pie_consonants: &[&str]) -> RegexFst {
    let mut consonant_fsts = pie_consonants
        .iter()
        .map(|c| RegexFst::new_from_ipa(xsampa_to_ipa(c)))
        .collect::<Vec<_>>();

    let mut disjoint_consonants = consonant_fsts.pop().unwrap();
    for fst in consonant_fsts {
        disjoint_consonants.disjoint(&fst)
    }
    disjoint_consonants
}

#[test]
fn test_add() {
    assert_eq!(3 + 2, 5);
}
