use rustfst::SymbolTable;

// figure out how to maek this static
// requires a type change, otherwise I basically have to clone, which has minimal performance gains
pub fn ipa() -> SymbolTable {
    let mut table = SymbolTable::new();
    let chars: Vec<String> = ('a'..='z').map(|x| x.to_string()).collect();

    table.add_symbols(chars);
    let chars: Vec<String> = (0x0250..=0x02AF) // ipa section in unicode
        .map(|x| char::from_u32(x).unwrap().to_string())
        .collect();

    table.add_symbols(chars);
    let spacing: Vec<String> = (0x02B0..=0x02FF) // ipa section in unicode
        .map(|x| char::from_u32(x).unwrap().to_string())
        .collect();

    table.add_symbols(spacing);
    table.add_symbol(" "); //becuase it's annoying when I make a typo
    table
}

// likely uneeded, but I think I use it in some test cases
pub fn xsampa_ascii() -> SymbolTable {
    let xsampa_chars: Vec<String> = (' '..='~')
        .filter(|x| *x != '"') // breaks dot when debugging
        .map(|x| x.to_string())
        .collect();

    let mut table = SymbolTable::new();
    table.add_symbols(xsampa_chars);
    table
}

// unneeded again due to the approach not actually being useful
pub fn single_character_class() -> SymbolTable {
    let chars = vec!["C", "NC", "<", ">"];

    let mut table = SymbolTable::new();
    table.add_symbols(chars);
    table
}
