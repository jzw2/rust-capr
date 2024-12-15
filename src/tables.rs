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

pub fn xsampa_ascii() -> SymbolTable {
    let xsampa_chars: Vec<String> = (' '..='~')
        .map(|x| x.to_string())
        .collect();

    let mut table = SymbolTable::new();
    table.add_symbols(xsampa_chars);
    table
}
