use std::str::Chars;

pub struct Scanner {
    string: Box<[char]>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            string: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn at_end(&self) -> bool {
        self.string[self.current] == '\0'
    }
    fn fwd(&mut self) -> char {
        self.current += 1;
        self.string[self.current]
    }
    fn bck(&mut self) -> char {
        self.current += 1;
        self.string[self.current]
    }
    fn curr(&self) -> char {
        self.string[self.current]
    }
    fn skip_whitespace(&mut self) {
        loop {
            let c = self.curr();
            match c {
                ' ' | '\r' | '\t' => {
                    self.fwd();
                    break;
                }
                '\n' => {
                    self.line += 1;
                    self.fwd();
                    break;
                }
                '#' => {
                    while self.curr() != '\n' && !self.at_end() {
                        self.fwd();
                    }
                    break;
                }
                _ => break,
            }
        }
    }
    fn check_keyword(&mut self, start: usize, rest: &str, token: TokenEnum) -> TokenEnum {
        if self.string[self.start + start..rest.len()]
            .iter()
            .collect::<String>()
            == rest
        {
            return token;
        }
        TokenEnum::TOKEN_IDENTIFIER
    }
    fn identifier(&mut self) -> Token {
        while self.curr().is_alphabetic() || self.curr().is_numeric() {
            self.fwd();
        }
        Token::new(&self, {
            match self.string[self.start] {
                'a' => self.check_keyword(1, "nd", TokenEnum::TOKEN_AND),
                'c' => self.check_keyword(1, "lass", TokenEnum::TOKEN_CLASS),
                'e' => self.check_keyword(1, "lse", TokenEnum::TOKEN_CLASS),
                'f' => {
                    if self.current - self.start > 1 {
                        match self.string[self.start] {
                            'a' => self.check_keyword(2, "lse", TokenEnum::TOKEN_FALSE),
                            'o' => self.check_keyword(2, "r", TokenEnum::TOKEN_FOR),
                            'u' => self.check_keyword(2, "n", TokenEnum::TOKEN_FUN),
                            _ => TokenEnum::TOKEN_IDENTIFIER,
                        }
                    } else {
                        TokenEnum::TOKEN_IDENTIFIER
                    }
                }
                'i' => self.check_keyword(1, "f", TokenEnum::TOKEN_IF),
                'n' => self.check_keyword(1, "f", TokenEnum::TOKEN_NIL),
                'o' => self.check_keyword(1, "r", TokenEnum::TOKEN_OR),
                'p' => self.check_keyword(1, "rint", TokenEnum::TOKEN_PRINT),
                'r' => self.check_keyword(1, "eturn", TokenEnum::TOKEN_RETURN),
                's' => self.check_keyword(1, "uper", TokenEnum::TOKEN_SUPER),
                't' => {
                    if self.current - self.start > 1 {
                        match self.string[self.start] {
                            'h' => self.check_keyword(2, "is", TokenEnum::TOKEN_THIS),
                            'r' => self.check_keyword(2, "ue", TokenEnum::TOKEN_TRUE),
                            _ => TokenEnum::TOKEN_IDENTIFIER,
                        }
                    } else {
                        TokenEnum::TOKEN_IDENTIFIER
                    }
                }
                'v' => self.check_keyword(1, "ar", TokenEnum::TOKEN_VAR),
                'w' => self.check_keyword(1, "hile", TokenEnum::TOKEN_WHILE),
                _ => TokenEnum::TOKEN_IDENTIFIER,
            }
        })
    }
    fn number(&mut self) {
        while self.curr().is_numeric() {
            self.fwd();
        }
    }
}

pub struct Token {
    token: TokenEnum,
    start: usize,
    length: usize,
    line: usize,
}

impl Token {
    fn new(scanner: &Scanner, token: TokenEnum) -> Token {
        Token {
            token,
            start: scanner.start,
            length: scanner.current - scanner.start,
            line: scanner.line,
        }
    }
}

pub enum TokenEnum {
    // Single Character
    TOKEN_LEFT_PAREN,
    TOKEN_RIGHT_PAREN,
    TOKEN_LEFT_BRACE,
    TOKEN_RIGHT_BRACE,
    TOKEN_COMMA,
    TOKEN_DOT,
    TOKEN_MINUS,
    TOKEN_PLUS,
    TOKEN_SEMICOLON,
    TOKEN_SLASH,
    TOKEN_STAR,

    // One or two characters
    TOKEN_BANG,
    TOKEN_BANG_EQUAL,
    TOKEN_EQUAL,
    TOKEN_EQUAL_EQUAL,
    TOKEN_GREATER,
    TOKEN_GREATER_EQUAL,
    TOKEN_LESS,
    TOKEN_LESS_EQUAL,

    // Literals
    TOKEN_IDENTIFIER,
    TOKEN_STRING,
    TOKEN_NUMBER,

    // Keywords
    TOKEN_AND,
    TOKEN_CLASS,
    TOKEN_ELSE,
    TOKEN_FALSE,
    TOKEN_FOR,
    TOKEN_FUN,
    TOKEN_IF,
    TOKEN_NIL,
    TOKEN_OR,
    TOKEN_PRINT,
    TOKEN_RETURN,
    TOKEN_SUPER,
    TOKEN_THIS,
    TOKEN_TRUE,
    TOKEN_VAR,
    TOKEN_WHILE,

    TOKEN_EOF,
    TOKEN_NONE,
}
