use simplang::*;

fn main() {
    let input = r#"
        Main Begin
            Int8 SomeNumber 10
            Printf "%i", SomeNumber
        Main End
    "#;
    Lexer::new(input.to_owned()).process();
}
