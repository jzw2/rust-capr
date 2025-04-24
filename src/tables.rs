use rustfst::SymbolTable;

// figure out how to maek this static
pub fn lower_case_latin() -> SymbolTable {
    // todo figure out how to do the boundry better
    let alphabet: Vec<_> = "#abcedfghijklmnopqrstuvwxyz"
        .chars()
        .map(|x| x.to_string())
        .collect();
    // let alphabet: Vec<_> = "abcx".split("").collect();
    let mut table = SymbolTable::new();
    table.add_symbols(alphabet);
    table
}

pub fn ipa() -> SymbolTable {
    let mut table = SymbolTable::new();
    let chars: Vec<String> = ('a'..='z').map(|x| x.to_string()).collect();

    table.add_symbols(chars);
    let chars: Vec<String> = (0x0250..=0x02AF)
        .map(|x| char::from_u32(x).unwrap().to_string())
        .collect();

    table.add_symbols(chars);
    table
}

pub fn xsampa_ascii() -> SymbolTable {
    let xsampa_chars: Vec<String> = (' '..='~')
        .filter(|x| *x != '"') // breaks dot when debugging
        .map(|x| x.to_string())
        .collect();

    let mut table = SymbolTable::new();
    table.add_symbols(xsampa_chars);
    table
}
pub fn single_character_class() -> SymbolTable {
    let chars = vec!["C", "NC", "<", ">"];

    let mut table = SymbolTable::new();
    table.add_symbols(chars);
    table
}
