
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Normal,
    Str,
    Comment,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Keyword {
    If,
    Let,
    Else,
    While,
    Return,
    Use,
    Function,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Type {
    Void,
    Int,
    Char,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    And,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Comparison {
    Equal,
    NotEqual,
    Bigger,
    Smaller,
    Nop,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Type(Type),
    Ident(String),
    Int(usize),
    Str(String),
    Operator(Operator),
    Comparison(Comparison),

    OpenBrace,
    CloseBrace,

    OpenParen,
    CloseParen,

    OpenBracket,
    CloseBracket,

    Semi,
    Comma,
    Colon,
    Equal,

    Newline,
}

const STR_KEYWORDS: [&str; 33] = [
    "[",
    "]",
    "{",
    "}",
    "(",
    ")",
    ";",
    ",",
    "=",
    "==",
    "!=",
    "<",
    ">",
    "+",
    "-",
    "*",
    "/",
    ":",
    "//",
    "&",
    "return",
    "#include",
    "#link",
    "if",
    "else",
    "while",
    "fn",
    "let",
    "int",
    "char",
    "void",
    "\n",
    " ",
];

pub type SourceLocation = (usize, usize);

fn to_string(char_code: &Result<u8, std::io::Error>) -> String {
    String::from_utf8(vec![*char_code.as_ref().unwrap()]).unwrap()
}

fn lex_token(token: &str) -> Token {
    match token {
        "[" => Token::OpenBracket,
        "]" => Token::CloseBracket,
        "{" => Token::OpenBrace,
        "}" => Token::CloseBrace,
        "(" => Token::OpenParen,
        ")" => Token::CloseParen,
        ";" => Token::Semi,
        ":" => Token::Colon,
        "," => Token::Comma,
        "=" => Token::Equal,
        "+" => Token::Operator(Operator::Add),
        "-" => Token::Operator(Operator::Sub),
        "*" => Token::Operator(Operator::Mul),
        "/" => Token::Operator(Operator::Div),
        "&" => Token::Operator(Operator::And),
        "==" => Token::Comparison(Comparison::Equal),
        "!=" => Token::Comparison(Comparison::NotEqual),
        "<" => Token::Comparison(Comparison::Smaller),
        ">" => Token::Comparison(Comparison::Bigger),
        "if" => Token::Keyword(Keyword::If),
        "let" => Token::Keyword(Keyword::Let),
        "else" => Token::Keyword(Keyword::Else),
        "while" => Token::Keyword(Keyword::While),
        "return" => Token::Keyword(Keyword::Return),
        "use" => Token::Keyword(Keyword::Use),
        "fn" => Token::Keyword(Keyword::Function),
        "int" => Token::Type(Type::Int),
        "char" => Token::Type(Type::Char),
        "void" => Token::Type(Type::Void),
        _ => {
            let int = token.parse::<usize>();
            if int.is_err() {
                return Token::Ident(token.to_string());
            } else {
                return Token::Int(int.unwrap());
            }
        },
    }
}

pub fn tokenize(source: &str) -> Vec<(Token, SourceLocation)> {
    let mut mode = Mode::Normal;
    let mut tokens: Vec<(Token, SourceLocation)> = Vec::new();
    let mut token = String::new();
    let mut index = 0;
    let mut line = 1;
    let mut column = 1;
    let bytes = source.chars().collect::<Vec<char>>();

    while index < source.len() {
        let character = &bytes[index].to_string();
        if &(index + 1) < &bytes.len() {
            if mode == Mode::Normal {
                if character.as_str() == "=" {
                    if &bytes[index + 1].to_string() == "=" {
                        token = token + "==";
                        index += 1;
                    } else {
                        token = token + &character;
                    }
                } else if character.as_str() == "!" {
                    if &bytes[index + 1].to_string() == "=" {
                        token = token + "!=";
                        index += 1;
                    } else {
                        token = token + &character;
                    }
                } else if character.as_str() == "/" {
                    if &bytes[index + 1].to_string() == "/" {
                        mode = Mode::Comment;
                        index += 1;
                    } else {
                        token = token + &character;
                    }
                } else if character.as_str() == "\"" {
                    mode = Mode::Str;
                } else if character != " " {
                    token = token + &character;
                }
                if STR_KEYWORDS.contains(&bytes[index + 1].to_string().as_str()) || STR_KEYWORDS.contains(&token.as_str()) {
                    if token == "\n" {
                        token = String::new();
                    } else if token != "" {
                        tokens.push((lex_token(&token), (line, column - (token.len() - 1))));
                        token = String::new();
                    }
                }
            } else if mode == Mode::Str {
                if character.as_str() == "\"" {
                    mode = Mode::Normal;
                    tokens.push((Token::Str(token.clone()), (line, column)));
                    token = String::new();
                } else if character.as_str() == "\\" {
                    if &bytes[index + 1].to_string() == "n" {
                        token = token + "\n";
                        index += 1;
                    } else {
                      token = token + &character;
                    }
                } else {
                    token = token + &character;
                }
            }
        }
        if character.as_str() == "\n" {
            tokens.push((Token::Newline, (line, column)));
            line += 1;
            column = 1;
            if mode != Mode::Str {
                mode = Mode::Normal;
            }
        } else {
            column += 1;
        }
        index += 1;
    }
    return tokens;
}


