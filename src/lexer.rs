#[derive(Debug)]
pub enum TokenType {
    Identifier(String),
    IntegerLiteral(i32),
    StringLiteral(String),

    Plus,
    Minus,
    Asterisk,
    Slash,

    Equals,
    DoubleEquals,

    Lparen,
    Rparen,

    Semicolon,

    Invalid(String),
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    // TODO: additional token information
    // position: i32,
}

impl Token {
    pub fn new(kind: TokenType) -> Self {
        Self { kind }
    }
}



pub struct Tokenizer {
    position:   usize,
    pub source: String,
}


impl Tokenizer {

    pub fn new(source: String) -> Self {
        Self { position: 0, source }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {

        let mut tokenlist = Vec::new();

        while !self.is_at_end() {

            if let Some(token) = self.next_token() {
                tokenlist.push(token);
            }

            self.position += 1;

        }

        tokenlist

    }

    fn is_at_end(&self) -> bool {
        self.position + 1 > self.source.len()
    }




    fn lookahead(&self) -> std::iter::Enumerate<std::iter::Skip<std::str::Chars<'_>>> {
        self.source.chars().skip(self.position+1).enumerate()
    }


    // Skips to the end of an integer literal and returns it
    fn lookahead_integerliteral(&mut self) -> i32 {

        let mut skip = 0usize;
        for (index, c) in self.lookahead() {
            if !c.is_numeric() {
                skip = index;
                break;
            }
        };

        self.position += skip;

        let slice: &str = &self.source[self.position-skip..self.position+1];
        slice.parse().unwrap()

    }


    // if the next char is equal to `c`, return true and skip it, else return false
    fn lookahead_operator_double(&mut self, c: char) -> bool {

        if self.source.chars().nth(self.position+1).unwrap() == c {
            self.position += 1;
            true
        }
        else {
            false
        }

    }


    // checks if `c` is the first character in an identifier
    fn char_is_identfier_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn char_is_identifier(c: char) -> bool {
        c.is_ascii_alphabetic() || c.is_numeric() || c == '_'
    }



    // Returns Option::None if token should be ignored (eg: whitespace, newlines...)
    fn next_token(&mut self) -> Option<Token> {

        let char: char = self.source.chars().nth(self.position).unwrap();
        let mut ignore: bool = false;

        let mut kind = TokenType::Invalid("".to_string());

        match char {
            '('  => kind = TokenType::Lparen,
            ')'  => kind = TokenType::Rparen,
            '+'  => kind = TokenType::Plus,
            '-'  => kind = TokenType::Minus,
            '*'  => kind = TokenType::Asterisk,
            '/'  => kind = TokenType::Slash,
            ';'  => kind = TokenType::Semicolon,
            '\n' => ignore = true,
            ' '  => ignore = true,

            '='  => {

                if self.lookahead_operator_double('=') {
                    kind = TokenType::DoubleEquals;
                }
                else {
                    kind = TokenType::Equals;
                }

            }

            value => {

                // Integer literal
                if char.is_numeric() {
                    let literal = self.lookahead_integerliteral();
                    kind = TokenType::IntegerLiteral(literal);
                }

                else if Self::char_is_identfier_start(char) {


                    let mut skip = 0usize;
                    for (index, c) in self.source.chars().skip(self.position+1).enumerate() {
                        if !Self::char_is_identifier(c) {
                            skip = index;
                            break;
                        }
                    };

                    self.position += skip;

                    let slice: &str = &self.source[self.position-skip..self.position+1];
                    kind = TokenType::Identifier(slice.to_string());



                }
                // Unknown Symbol
                else {
                    kind = TokenType::Invalid(value.to_string());
                }


            },
        }


        if ignore {
            Option::None
        }
        else {
            Option::from(Token::new(kind))
        }


    }

}
