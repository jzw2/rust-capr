use ipa_translate::xsampa_to_ipa;
use rust_capr::{
    regex::RegexFst,
    sound_law::{SoundLaw, SoundLawComposition},
    tables::{ipa, xsampa_ascii},
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

fn a3() -> SoundLawComposition {
    let data = common_setup();

    let from = xsampa_disjoint(&["h", "x", "q"]);
    let to = RegexFst::new_from_ipa("a".into());

    let left = &data.consonants;
    let right = &data.consonants;

    let law = SoundLaw::create_with_arbitrary_regex(left, right, &from, &to, &data.table);
    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}

// a4 requires suprasegmental for stress

fn a5() -> SoundLawComposition {
    let data = common_setup();

    let mut from = data.coronals.clone();
    from.concat(&data.coronals); // Important: concat, not extend

    let to = RegexFst::new_from_ipa("ss".into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}

// a6 epenthisis

// a7 requires stress

// a8 requires knowing the begin and end of a word

// a9 can be made easier by just assuming that they started merged, since no sound law depends on the palatals
//
//
//
fn b1() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("g_w"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("b"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}

fn b2() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("b_h"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("b"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let from = RegexFst::new_from_ipa(xsampa_to_ipa("d_h"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("d"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("g_w_h"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("g_w"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law3 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("g_h"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("g"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law4 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law1);
    comp.add_law(&law2);
    comp.add_law(&law3);
    comp.add_law(&law4);
    comp
}

// b3 has epenthesis

fn b4() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("e"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("a"));
    let left = RegexFst::new_from_ipa("".into());
    let mut right = data.resonants.clone();
    right.concat(&RegexFst::new_from_ipa("a".into()));

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
// b5 is an epenthesis rule

// b6 requires knowing how syllable boundries for the laryngeals

// b7 is complicated

fn b8() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("e:"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("i:"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}

// b9 requires final syllable knowledge
//
// b10 literally doesn't exist
//
// b11

fn b11() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa(":"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let left = data.vowels.clone();
    let mut right = data.resonants.clone();
    right.concat(&data.consonants);

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}

fn c1() -> SoundLawComposition {
    let data = common_setup();

    let mut from = data.stops.clone();
    from.disjoint(&RegexFst::new_from_ipa("s".into()));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("x"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = data.stops.clone();

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
fn c2() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("p"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("b"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = data.liquids.clone();

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}

fn c3() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("p"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("w"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = data.nasals.clone();

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
fn c4() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("p"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("f"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = RegexFst::new_from_ipa(xsampa_to_ipa(""));

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
fn c5() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("o:"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("a:"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = RegexFst::new_from_ipa(xsampa_to_ipa(""));

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
fn c6() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("ey"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("e:"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = RegexFst::new_from_ipa(xsampa_to_ipa(""));

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
fn c7() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("ew"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("ow"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = RegexFst::new_from_ipa(xsampa_to_ipa(""));

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
fn c8() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("uw"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("ow"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = data.consonants.clone();

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
fn d1() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("rf"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("rr"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = data.consonants.clone();

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("lf"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("ll"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = data.consonants.clone();

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    // this needs to come first so it doesn't bleed
    let from = RegexFst::new_from_ipa(xsampa_to_ipa("rst"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("rt"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = data.consonants.clone();

    let law3 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("rs"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("rr"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = data.consonants.clone();

    let law4 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law1); // 3 needs to come before
    comp.add_law(&law2); // 3 needs to come before
    comp.add_law(&law3); // 3 needs to come before
    comp.add_law(&law4); // 3 needs to come before
    comp
}
fn d2() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("mw"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("ww"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let right = RegexFst::new_from_ipa(xsampa_to_ipa(""));

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
// since lengthening is essentally epenthesis, d3 is hard right now

fn d4() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("ar"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("ra"));
    let left = data.labials.clone();
    let mut right = data.coronals.clone();
    right.concat(&data.coronals);

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("al"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("la"));
    let left = data.labials.clone();
    let mut right = data.coronals.clone();
    right.concat(&data.coronals);

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let mut comp = SoundLawComposition::new();
    comp.add_law(&law1);
    comp.add_law(&law2);
    comp
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
    let law = a3();

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
    let law = a5();

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
    aspirate_stops: RegexFst,
    vowels: RegexFst,
    liquids: RegexFst,
    nasals: RegexFst,
    labials: RegexFst,
}

fn common_setup() -> CommonData {
    let table = ipa();
    let consonants = xsampa_disjoint(&pie_consonants());
    let stops = xsampa_disjoint(&pie_stops());
    let resonants = xsampa_disjoint(&pie_resonants());
    let laryngeals = xsampa_disjoint(&pie_laryngeals());
    let coronals = xsampa_disjoint(&coronal_stops());
    let aspirate_stops = xsampa_disjoint(&"b_h d_h g_h g_w_h g_h".split(" ").collect::<Vec<_>>());
    let vowels = xsampa_disjoint(&"a e i o u".split(" ").collect::<Vec<_>>());
    let liquids = xsampa_disjoint(&"l r".split(" ").collect::<Vec<_>>());
    let nasals = xsampa_disjoint(&"n m".split(" ").collect::<Vec<_>>());
    let labials = xsampa_disjoint(&"p b m w".split(" ").collect::<Vec<_>>());

    CommonData {
        table,
        consonants,
        stops,
        resonants,
        laryngeals,
        coronals,
        aspirate_stops,
        vowels,
        liquids,
        nasals,
        labials,
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
    let law = b1();
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
