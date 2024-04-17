#[derive(PartialEq, Copy, Clone, strum_macros::Display, Debug, strum_macros::EnumIter)]
pub enum Keyword {
    Mod,
    Struct,
    Impl,
    Fn,
    If,
    Elif,
    Else,
    While,
    Break,
    Continue,
    Let,
    Return,
}

pub const MOD_KEYWORD: &str = "mod";

impl Keyword {
    pub fn get_enum(name: &str) -> Option<Keyword> {
        match name {
            MOD_KEYWORD => Some(Keyword::Mod),
            "struct" => Some(Keyword::Struct),
            "impl" => Some(Keyword::Impl),
            "fn" => Some(Keyword::Fn),
            "if" => Some(Keyword::If),
            "elif" => Some(Keyword::Elif),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "break" => Some(Keyword::Break),
            "continue" => Some(Keyword::Continue),
            "let" => Some(Keyword::Let),
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}
