use error::Error;
use value::{FloatType, IntType};

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // Single character tokens
    // Arithmetic
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Logic
    Eq,
    Neq,
    Gt,
    Lt,
    Geq,
    Leq,
    And,
    Or,
    Not,

    // Precedence
    LBrace,
    RBrace,
    Whitespace,

    // Complex tokens
    Identifier(String),
    Float(FloatType),
    Int(IntType),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PartialToken {
    Token(Token),
    Literal(String),
    Eq,
    ExclamationMark,
    Gt,
    Lt,
    Ampersand,
    VerticalBar,
}

// Make this a const fn as soon as match gets stable (issue #57563)
fn char_to_partial_token(c: char) -> PartialToken {
    match c {
        '+' => PartialToken::Token(Token::Plus),
        '-' => PartialToken::Token(Token::Minus),
        '*' => PartialToken::Token(Token::Star),
        '/' => PartialToken::Token(Token::Slash),
        '%' => PartialToken::Token(Token::Percent),

        '=' => PartialToken::Eq,
        '!' => PartialToken::ExclamationMark,
        '>' => PartialToken::Gt,
        '<' => PartialToken::Lt,
        '&' => PartialToken::Ampersand,
        '|' => PartialToken::VerticalBar,

        '(' => PartialToken::Token(Token::LBrace),
        ')' => PartialToken::Token(Token::RBrace),

        c => {
            if c.is_whitespace() {
                PartialToken::Token(Token::Whitespace)
            } else {
                PartialToken::Literal(c.to_string())
            }
        }
    }
}

impl Token {
    // Make this a const fn as soon as match gets stable (issue #57563)
    pub fn is_value(&self) -> bool {
        match self {
            Token::Plus => false,
            Token::Minus => false,
            Token::Star => false,
            Token::Slash => false,
            Token::Percent => false,

            Token::Eq => false,
            Token::Neq => false,
            Token::Gt => false,
            Token::Lt => false,
            Token::Geq => false,
            Token::Leq => false,
            Token::And => false,
            Token::Or => false,
            Token::Not => false,

            Token::LBrace => false,
            Token::RBrace => true,
            Token::Whitespace => false,

            Token::Identifier(_) => true,
            Token::Float(_) => true,
            Token::Int(_) => true,
            Token::Boolean(_) => true,
        }
    }
}

/// Converts a string to a vector of partial tokens.
fn str_to_tokens(string: &str) -> Vec<PartialToken> {
    let mut result = Vec::new();
    for c in string.chars() {
        let partial_token = char_to_partial_token(c);

        let if_let_successful =
            if let (Some(PartialToken::Literal(last)), PartialToken::Literal(literal)) =
                (result.last_mut(), &partial_token)
            {
                last.push_str(literal);
                true
            } else {
                false
            };

        if !if_let_successful {
            result.push(partial_token);
        }
    }
    result
}

/// Resolves all partial tokens by converting them to complex tokens.
fn resolve_literals(mut tokens: &[PartialToken]) -> Result<Vec<Token>, Error> {
    let mut result = Vec::new();
    while tokens.len() > 0 {
        let first = tokens[0].clone();
        let second = tokens.get(1).cloned();
        let mut cutoff = 2;

        result.push(match first {
            PartialToken::Token(token) => {
                cutoff = 1;
                token
            }
            PartialToken::Literal(literal) => {
                cutoff = 1;
                if let Ok(number) = literal.parse::<IntType>() {
                    Token::Int(number)
                } else if let Ok(number) = literal.parse::<FloatType>() {
                    Token::Float(number)
                } else if let Ok(boolean) = literal.parse::<bool>() {
                    Token::Boolean(boolean)
                } else {
                    Token::Identifier(literal.to_string())
                }
            }
            PartialToken::Eq => match second {
                Some(PartialToken::Eq) => Token::Eq,
                _ => return Err(Error::unmatched_partial_token(first, second)),
            },
            PartialToken::ExclamationMark => match second {
                Some(PartialToken::Eq) => Token::Eq,
                _ => {
                    cutoff = 1;
                    Token::Not
                }
            },
            PartialToken::Gt => match second {
                Some(PartialToken::Eq) => Token::Geq,
                _ => {
                    cutoff = 1;
                    Token::Gt
                }
            },
            PartialToken::Lt => match second {
                Some(PartialToken::Eq) => Token::Leq,
                _ => {
                    cutoff = 1;
                    Token::Lt
                }
            },
            PartialToken::Ampersand => match second {
                Some(PartialToken::Ampersand) => Token::And,
                _ => return Err(Error::unmatched_partial_token(first, second)),
            },
            PartialToken::VerticalBar => match second {
                Some(PartialToken::VerticalBar) => Token::Or,
                _ => return Err(Error::unmatched_partial_token(first, second)),
            },
        });

        tokens = &tokens[cutoff..];
    }
    Ok(result)
}

pub fn tokenize(string: &str) -> Result<Vec<Token>, Error> {
    resolve_literals(&str_to_tokens(string))
}
