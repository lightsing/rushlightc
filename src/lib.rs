use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct RushlightParser;