use rustfst::SymbolTable;

// figure out how to maek this static
pub fn lower_case_latin() -> SymbolTable {
    let alphabet: Vec<_> = "abcedfghijklmnopqrstuvwxyz"
        .chars()
        .map(|x| x.to_string())
        .collect();
    // let alphabet: Vec<_> = "abcx".split("").collect();
    let mut table = SymbolTable::new();
    table.add_symbols(alphabet);
    table
}
