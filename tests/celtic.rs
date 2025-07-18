use ipa_translate::xsampa_to_ipa;
use rust_capr::{
    regex::RegexFst,
    sound_law::{transduce_from_labels, SoundLaw, SoundLawComposition},
    tables::ipa,
};
use rustfst::{Label, SymbolTable};

// boundry symbols
fn boundry(labels: &[Label], table: &SymbolTable) -> Vec<Label> {
    let boundry = table
        .get_label("boundry")
        .expect("Failed to find boundry label");
    let mut ret = labels.to_vec();
    ret.insert(0, boundry);
    ret
}

/// puts it after the string
fn noninitial(labels: &[Label], table: &SymbolTable) -> Vec<Label> {
    let noninitial_index = table
        .get_label("noninitial")
        .expect("Failed to find noninitial label");
    let vowels = "a e i o u"
        .split(" ")
        .map(|x| table.get_label(x).expect("Failed to find label"))
        .collect::<Vec<_>>();
    let mut new = Vec::new();
    let mut first = true;
    for c in labels {
        new.push(*c);

        if vowels.contains(c) {
            if first {
                first = false;
            } else {
                new.push(noninitial_index);
            }
        }
    }
    new
}

//the labels are in ipa
fn xsampa_to_labels(xsampa: &str) -> Vec<Label> {
    let table = ipa();
    let s: Vec<_> = xsampa_to_ipa(xsampa)
        .chars()
        .map(|x| table.get_label(x.to_string()).unwrap())
        .collect();
    s
}

fn labels_to_ipa(labels: &[Label]) -> String {
    let table = ipa();
    let s: String = labels
        .iter()
        .map(|x| table.get_symbol(*x).unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    s
}

#[test]
fn noninitial_test() {
    let table = ipa();
    let labels = xsampa_to_labels("d_hugxte:r");
    let labels = noninitial(&labels, &table);
    let ipa = labels_to_ipa(&labels);

    assert_eq!(ipa, "d ʰ u ɡ x t e noninitial ː r");
}

fn preprocess(s: &[Label], table: &SymbolTable) -> Vec<Label> {
    let s = noninitial(s, table);

    boundry(&s, table)
}

// represent the laryngals as h, x, q

fn a1() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("he"));

    let to = RegexFst::new_from_ipa("e".into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("xe"));

    let to = RegexFst::new_from_ipa("a".into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("qe"));

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

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("eh"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("e:"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("ex"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("a:"));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("eq"));

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
fn a4() -> SoundLawComposition {
    let data = common_setup();

    let mut from = data.laryngeals.clone();
    from.concat(&RegexFst::new_from_label(
        data.table.get_label("noninitial").unwrap(),
    ));
    let to = RegexFst::new_from_ipa("".into());

    let left = &data.stops.clone();
    let right = &data.stops.clone();

    let law = SoundLaw::create_with_arbitrary_regex(left, right, &from, &to, &data.table);
    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}

fn a5() -> SoundLawComposition {
    let data = common_setup();

    let mut from = data.coronals.clone();
    from.concat(&data.coronals);

    let to = RegexFst::new_from_ipa("ss".into());
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}

// a6 epenthisis
fn a6() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("h"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("a:"));
    let mut left = data.consonants.clone();
    left.concat(&data.resonants.clone());
    let right = data.consonants.clone();

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("x"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("a:"));
    let mut left = data.consonants.clone();
    left.concat(&data.resonants.clone());
    let right = data.consonants.clone();

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("q"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("a:"));
    let mut left = data.consonants.clone();
    left.concat(&data.resonants.clone());
    let right = data.consonants.clone();

    let law3 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut total = SoundLawComposition::new();
    total.add_law(&law1);
    total.add_law(&law2);
    total.add_law(&law3);
    total
}
// a7 requires stress, and also seems to fail to anotate it in all periods

// a8 requires knowing the begin and end of a word
fn a8() -> SoundLawComposition {
    let data = common_setup();

    let from = data.laryngeals.clone();

    let to = RegexFst::new_from_ipa("a".into());
    let mut left = RegexFst::new_from_label(data.table.get_label("#").unwrap());
    left.concat(&data.liquids.clone());
    let right = data.consonants.clone();

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
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

fn b3() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa("r".to_string());
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("ri"));
    let left = data.consonants.clone();
    let right = data.stops.clone();

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa("l".to_string());
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("li"));
    let left = data.consonants.clone();
    let right = data.stops.clone();

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law1);
    comp.add_law(&law2);
    comp
}
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

fn b5() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa("r".to_string());
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("ar"));
    let left = data.consonants.clone();
    let right = data.consonants.clone();

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa("l".to_string());
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("al"));
    let left = data.consonants.clone();
    let right = data.consonants.clone();

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let from = RegexFst::new_from_ipa("n".to_string());
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("an"));
    let left = data.consonants.clone();
    let right = data.consonants.clone();

    let law3 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let from = RegexFst::new_from_ipa("m".to_string());
    let to = RegexFst::new_from_ipa(xsampa_to_ipa("am"));
    let left = data.consonants.clone();
    let right = data.consonants.clone();

    let law4 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law1);
    comp.add_law(&law2);
    comp.add_law(&law3);
    comp.add_law(&law4);
    comp
}
// b6 requires knowing how syllable boundries for the laryngeals
// I'm just going to implenent unconditional dropping
fn b6() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("x"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let from = RegexFst::new_from_ipa(xsampa_to_ipa("h"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law2 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);
    let from = RegexFst::new_from_ipa(xsampa_to_ipa("q"));
    let to = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let left = RegexFst::new_from_ipa("".into());
    let right = RegexFst::new_from_ipa("".into());

    let law3 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law1);
    comp.add_law(&law2);
    comp.add_law(&law3);
    comp
}
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
    //from.disjoint(&RegexFst::new_from_ipa("s".into()));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("x"));
    let left = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let mut right = data.stops.clone();
    right.disjoint(&RegexFst::new_from_ipa("s".into()));

    let law = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law);
    comp
}
fn c2() -> SoundLawComposition {
    // must be incorrect or something
    // original has untrestrcited context
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("p"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("b"));
    let left = data.vowels.clone();
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
fn d5() -> SoundLawComposition {
    let data = common_setup();

    let from = data.laryngeals.clone();

    let to = RegexFst::new_from_ipa(xsampa_to_ipa(""));
    let mut left = data.vowels.clone();
    left.concat(&RegexFst::new_from_ipa("y".into()));
    let mut right = data.coronals.clone();
    right.concat(&data.coronals);

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law1);
    comp
}
fn nasal_assimiliation() -> SoundLawComposition {
    let data = common_setup();

    let from = RegexFst::new_from_ipa(xsampa_to_ipa("m"));

    let to = RegexFst::new_from_ipa(xsampa_to_ipa("n"));
    let mut left = RegexFst::new_from_ipa("".to_string());
    let mut right = data.coronals.clone();

    let law1 = SoundLaw::create_with_arbitrary_regex(&left, &right, &from, &to, &data.table);

    let mut comp = SoundLawComposition::new();
    comp.add_law(&law1);
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

#[test]
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

fn pie_laryngeals() -> Vec<&'static str> {
    "h x q".split(' ').collect()
}

fn pie_consonants() -> Vec<&'static str> {
    "p t k b d g b_h d_h g_h k_w g_w g_w_h m n l r w y s h x q"
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
    let mut table = ipa();
    table.add_symbol("noninitial");

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

#[test]
fn giant_test() {
    let mut laws = vec![
        a1(),
        a2(),
        a4(),
        a6(),
        // a7(),
        a8(),
        // a9(),
        b1(),
        b2(),
        b3(),
        a5(), // conflict with b3
        b4(),
        b5(),
        a3(), // CHC > CaC
        b6(), // laryngeal deletion
        // b7(),
        b8(),
        // b9(),
        //b10(),
        b11(),
        c1(),
        c2(), // seems just wrong, likely restricted to non initially
        c3(),
        c4(),
        c5(),
        c6(),
        c7(),
        c8(),
        d1(),
        d2(),
        //d3(),
        d4(),
        d5(),
        nasal_assimiliation(),
    ];

    println!("Finish building sound laws");
    let mut total = laws.remove(0);
    for l in laws {
        total.append(l);
    }

    let test_cases = [
        ("phte:r", "fati:r"),
        ("krdtu", "krissu"),
        ("plhno", "fla:no"),
        ("grhno", "gra:no"),
        ("grxno", "gra:no"),
        ("grqno", "gra:no"),
        ("g_wow", "bow"),
        ("krd", "krid"),
        ("b_hero", "bero"),
        ("terhtro", "taratro"),
        ("dnt", "dant"), //original danto
        ("b_hrso", "barso"),
        ("klheto", "kaleto"),
        ("wlho", "walo"),
        ("hre:g", "ri:g"), // b8
        ("hepirom", "efirom"),
        ("xwehnto", "winto"),   // b11
        ("sixmdo", "sindo"),    // b11
        ("septm", "sextam"),    // might glitch out if I don't provide the laryngeal deletion
        ("prptu", "frixtu"),    // c1
        ("mrg_wto", "mrixto"),  // c1
        ("xmlgto", "mlixto"),   // c1
        ("piprqse", "fibrase"), // c2
        ("supno", "sowno"),     // p is not accounted for
        ("deqno", "da:no"),     // c5
        ("reyd", "re:d"),
        ("newyo", "nowyo"),
        ("supno", "sowno"),
        // random ones
        ("potr", "fatar"), // cannot account for a
        ("phte:r", "fati:r"),
        ("pelhu", "filu"), // says it preserves e grade, but don't know where i comes from
        ("plew", "flow"),
        ("kapr", "gabro"),      // initial consonant is irregular
        ("d_hg_hesi", "gdesi"), //metathesis
        ("d_hg_ho:m", "gdon"),
        ("gexr", "ga:r"),    // original ga:ri
        ("g_hexns", "gans"), //original gansi
        ("g_helq", "gel"),   // original gelo
        ("genhos", "genos"),
        ("genu", "genu"),
        ("g_hyemo", "gyemo"),
        ("k_wend_h", "k_wend"), // so element at the end seems tob e an addition kwendso
        ("k_wey", "k_we:"),     //original kwe:s // s includes the nomintive ending
        ("k_wetwores", "k_wetwores"),
        ("k_wid", "k_wid"),
        ("kwo:n", "k_won"), // not sure baout u element
        ("kmti", "kanti"),
        ("kewxro", "kawaro"),
        ("klexro", "kla:ro"),
        ("klewos", "kluwos"),
        ("krewx", "kru:"),
        ("leg_h", "leg"),     // original lego/ legyo
        ("link_w", "link_w"), // oroginal link_wo // original leyk_w, but n infix added
        ("leyd", "loydo"),
        ("lewko", "lowko"),
        ("hlud_h", "lud"),
        ("mexk", "mak"), // original mako
        ("mexte:r", "ma:ti:"),
        ("med_hyo", "medyo"),
        ("mel", "mall"),
        ("nexu", "na:wa:"),
        ("nem", "nem"), // original nemo
        ("neb_hos", "nemos"),
        ("newos", "nowyo"),
        ("nok_wt", "noxt"),
        ("qektoh", "oxtu:"),
        ("qeyno", "oyno"),
        ("reyd_h", "re:d"), // original re:do
        ("reyh", "re:no"),
        ("re:g", "ri:g"),
        ("re:gnix", "ri:gani:"),
        ("hrewd_h", "rowd"), //oroginal rowdo
        ("sexg", "sagyo"),
        ("sexl", "salano"),
        ("sxl", "sal"), //  original saltro
        ("smxel", "samali"),
        ("sext", "sa:t"), //original sa:ti
        ("sed", "sed"),   //original sedo
        ("seno", "seno"),
        ("sent", "sent"), //original sentu
        ("soru", "serwa:"),
        ("spelhg_h", "sfelga:"),
        ("sperxg", "sfraxto"),
        ("sisk_wo", "sisku"),
        ("stex", "sista"),
        ("sek_w", "sk_wetlo"),
        ("skeqt", "ska:t"), // original ska:to
        ("skrib_h", "skri:bba:"),
        ("slunk", "slunko"), //original slewk
        ("snex", "sna:"),
        ("sneyg_w_h", "snig_w"),
        ("srewm", "srowman"),
        ("stomn", "stamna:"),
        ("xste:r", "stera:"),
        ("sterk", "stronko"),
        ("hsu", "su"),
        ("swexdu", "swa:du"),
        ("tenh", "torano"), // metathesis
        ("treyes", "tri:s"),
        ("tuh", "tu"),
        ("uper", "ufor"),
        ("wid_hu", "widu"),
        ("xwehnto", "winto"),
        ("yemho", "yemono"),
    ];

    let mut failures = vec![];

    for (input, expected) in test_cases {
        let transduced = total.transduce_text(&xsampa_to_ipa(input));

        if transduced.len() != 1 {
            failures.push(format!(
                "Input {:?}: expected 1 result, got {}",
                input,
                transduced.len()
            ));
            // continue; // Skip further checks for this input if the length is wrong
            let actual = transduced[1].replace(" ", "");
            let expected_ipa = xsampa_to_ipa(expected);
            println!(
                "{}: {} -> {}, expected {}",
                actual == expected_ipa,
                input,
                actual,
                expected_ipa
            );
        }

        let actual = transduced[0].replace(" ", "");
        let expected_ipa = xsampa_to_ipa(expected);
        println!(
            "{}: {} -> {}, expected {}",
            actual == expected_ipa,
            input,
            actual,
            expected_ipa
        );

        if actual != expected_ipa {
            failures.push(format!(
                "Input {:?}: expected {:?}, got {:?}",
                input, expected_ipa, actual
            ));
        }
    }
    let table = ipa();
    let input = "wirho";
    let mut labels: Vec<_> = "wi"
        .chars()
        .map(|c| table.get_label(c.to_string()).unwrap())
        .collect();
    let pretonic = table.get_label("pretonic").unwrap();
    labels.push(pretonic);
    let mut rest = "hro"
        .chars()
        .map(|c| table.get_label(c.to_string()).unwrap())
        .collect();
    labels.append(&mut rest);
    let transduced = transduce_from_labels(total.get_final_fst(), &table, &labels);

    let actual = transduced[0].replace(" ", "");
    let expected = "wiro";
    let expected_ipa = xsampa_to_ipa(expected);
    println!(
        "{}: {} -> {}, expected {}",
        actual == expected_ipa,
        input,
        actual,
        expected_ipa
    );

    if !failures.is_empty() {
        panic!("Some transductions failed:\n{}", failures.join("\n"));
    }
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
