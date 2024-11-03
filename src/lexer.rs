#[derive(Debug, Default)]
pub enum TokenType {
    // TODO: maybe structure this through multiple enums

    // Keywords
    KeywordFunc,
    KeywordIf,
    KeywordTrue,
    KeywordFalse,

    // Literals
    LiteralIdentifier(String),
    LiteralInteger(i32),
    LiteralString(String),

    // Arithmetic Operators
    Plus,
    Minus,
    Asterisk,
    Slash,

    // Logical Operators
    Equals,
    DoubleEquals,

    // Grouping Operators
    Lparen,
    Rparen,

    // Punctuation
    PunctSemicolon,
    PunctComma,

    // None
    #[default] None,
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



    // Skips to the current position and returns an enumerated iterator
    fn lookahead(&self) -> std::iter::Enumerate<std::iter::Skip<std::str::Chars<'_>>> {
        self.source.chars().skip(self.position+1).enumerate()
    }

    // Returns Option::None if string is unterminated
    fn lookahead_string_literal(&mut self, quote_symbol: char) -> Option<String> {
        let mut skip = 0usize;
        for (index, c) in self.lookahead() {
            if c == quote_symbol {
                skip = index+1; // skip the closing quotes
                break;
            }
        };

        self.position += skip;

        // Even if string is empty, skip will always be at least 1, to skip the closing quotes
        if skip == 0 {
            Option::None
        }
        else {
            Option::from(self.source[self.position-skip+1..self.position].to_string())
        }

    }

    fn lookahead_identifier_literal(&mut self) -> String {
        let mut skip = 0usize;
        for (index, c) in self.lookahead() {
            if !Self::char_is_identifier(c) {
                skip = index;
                break;
            }
        };

        self.position += skip;
        self.source[self.position-skip..self.position+1].to_string()

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

        if self.get_char(self.position+1) == c {
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

    // Gets a character from the source code by index
    fn get_char(&self, position: usize) -> char {
        self.source.chars().nth(position).unwrap()
    }


    fn handle_error<T: AsRef<str>>(message: T) -> ! {
        eprintln!("Lexer Error: {}", message.as_ref());
        std::process::exit(1);
    }


    // Returns Option::None if token should be ignored (eg: whitespace, newlines...)
    fn next_token(&mut self) -> Option<Token> {

        let char: char = self.get_char(self.position);
        let mut ignore = false;
        let mut kind   = TokenType::default();

        match char {
            '('  => kind   = TokenType::Lparen,
            ')'  => kind   = TokenType::Rparen,
            '+'  => kind   = TokenType::Plus,
            '-'  => kind   = TokenType::Minus,
            '*'  => kind   = TokenType::Asterisk,
            '/'  => kind   = TokenType::Slash,
            ';'  => kind   = TokenType::PunctSemicolon,
            ','  => kind   = TokenType::PunctComma,
            '\n' => ignore = true,
            ' '  => ignore = true,

            '"' => {
                kind = TokenType::LiteralString(
                    match self.lookahead_string_literal('"') {
                        Some(string) => string,
                        None => Self::handle_error("Unterminated string literal"),
                    }
                );
            }

            '\'' => {
                kind = TokenType::LiteralString(
                    match self.lookahead_string_literal('\'') {
                        Some(string) => string,
                        None => Self::handle_error("Unterminated string literal"),
                    }
                );
            }

            '='  => {
                kind =
                    if self.lookahead_operator_double('=') {
                        TokenType::DoubleEquals
                    }
                    else {
                        TokenType::Equals
                    };
            }

            value => {
                kind =
                    // Integer Literal
                    if char.is_numeric() {
                        TokenType::LiteralInteger
                            (self.lookahead_integerliteral())
                    }

                    // Reserved Words
                    // TODO: this

                    // Identifier Literal
                    else if Self::char_is_identfier_start(char) {
                        TokenType::LiteralIdentifier
                            (self.lookahead_identifier_literal())
                    }

                    // Unknown Symbol
                    else {
                        Self::handle_error(format!("Unknown symbol: {value}"));
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
