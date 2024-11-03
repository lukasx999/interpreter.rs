#[derive(Debug)]
pub enum TokenType {
    Identifier(String),
    IntegerLiteral(i32),
    StringLiteral(String),

    Plus,
    Minus,
    Asterisk,
    Slash,

    Lparen,
    Rparen,

    Semicolon,

    Invalid,
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

    // Returns Option::None if token should be ignored (eg: whitespace, newlines...)
    fn next_token(&mut self) -> Option<Token> {

        let char: char = self.source.chars().nth(self.position).unwrap();
        let mut ignore: bool = false;

        let mut kind = TokenType::Invalid;

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
            _    => {

                if char.is_numeric() {

                    let mut skip = 0usize;
                    for (index, c) in self.source.chars().skip(self.position).enumerate() {
                        dbg!(index);
                        if !c.is_numeric() {
                            skip = index;
                            break;
                        }
                    };

                    self.position += skip;

                    kind = TokenType::IntegerLiteral(1);


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
