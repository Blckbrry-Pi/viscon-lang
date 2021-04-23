use visconc::frontend::parsing::lexer;
//use visconc::frontend::parsing::parser;

fn main() {
    println!("{:#?}", lexer::lexically_analyze(
        r#"
        Hello <! World.
        "You\re coolio \"m8\" "
        eleven << twelve #helooooooo
        qwertyuiop
        "#
    ));
}
