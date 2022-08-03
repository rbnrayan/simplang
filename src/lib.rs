pub enum Token {
    Ident,
    Keyword,
    Operator,
    Literal,
    Punctuation,
}

pub struct Lexer {
    src: Vec<u8>,
    col: usize,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            src: input.into_bytes(),
            col: 1,
            line: 1,
        }
    }

    pub fn process(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let operators = [b'+', b'-', b'*', b'/'];

        while let Some(byte) = self.peek() {
            if byte == b'\n' {
                self.line += 1;
                self.col = 1;
                continue;
            }

            if byte.is_ascii_whitespace() {
                // do nothing
            } else if byte.is_ascii_digit() {
                // Number literal : 1324
                let num_literal = self.get_literal("NUMBER").unwrap();
                tokens.push(num_literal);
            } else if byte == b'"' {
                // String literal : "abcde"
                let str_literal = self.get_literal("STRING").unwrap();
                tokens.push(str_literal);
            } else if operators.contains(&byte) {
                // Operator : +
                let op = self.get_operator().unwrap();
                tokens.push(op);
            } else if byte == b',' {
                // Punctuation : ,
                let puncuation = self.get_punctuation().unwrap();
                tokens.push(puncuation);
            } else if byte == b'_' { 
                // Identifier : _SomeIdent
                let ident = self.get_ident().unwrap();
                tokens.push(ident);
            } else if byte.is_ascii_alphabetic() {
                if self.next_is_keyword() {
                    // Keyword: Begin | Int8 | String | ...
                    let keyword = self.get_keyword().unwrap();
                    tokens.push(keyword);
                } else if self.next_is_literal() {
                    // Boolean : true
                    let literal = self.get_literal("BOOL").unwrap();
                    tokens.push(literal);
                } else {
                    // Identifier : SomeIdent
                    let ident = self.get_ident().unwrap();
                    tokens.push(ident);
                }
            }

            self.col += 1;
        }

        tokens
    }

    fn get_ident(&mut self) -> Option<Token> { None }
    fn get_keyword(&mut self) -> Option<Token> { None }
    fn get_operator(&mut self) -> Option<Token> { None }
    fn get_literal(&mut self, _kind: &str) -> Option<Token> { None }
    fn get_punctuation(&mut self) -> Option<Token> { None }
    fn next_is_keyword(&mut self) -> bool { false }
    fn next_is_literal(&mut self) -> bool { false }
    fn peek(&self) -> Option<u8> {
        if self.col * self.line + 1 < self.src.len() {
            return Some(self.src[self.col * self.line + 1]);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lex() {
        let input = r#"
            Main Begin
                Int8 SomeNumber 10
                Printf "%i", SomeNumber
            Main End
        "#;
        /*
         * Lexer output:
         *   [ Ident("Main"),
         *     Keyword("Begin"),
         *     Keyword("Int8"),
         *     Ident("SomeNumber"),
         *     Literal("10"),
         *     Ident("Printf"),
         *     Literal("\"%i\""),
         *     Punctuation(','),
         *     Ident("SomeNumber"),
         *     Ident("Main"),
         *     Keyword("End") ]
         */
        let lex = Lexer::new(input.to_owned());
        lex.process();
    }
}
