use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, Literal, LexError, TokenStream, TokenTree};

pub trait IsToken: Sized {
    fn match_token(tt: TokenTree) -> Option<Self>;
}

impl IsToken for Ident {
    fn match_token(tt: TokenTree) -> Option<Self> {
        if let TokenTree::Ident(ident) = tt {
            Some(ident)
        } else {
            None
        }
    }
}

impl IsToken for Group {
    fn match_token(tt: TokenTree) -> Option<Self> {
        if let TokenTree::Group(group) = tt {
            Some(group)
        } else {
            None
        }
    }
}

impl IsToken for Punct {
    fn match_token(tt: TokenTree) -> Option<Self> {
        if let TokenTree::Punct(punct) = tt {
            Some(punct)
        } else {
            None
        }
    }
}

impl IsToken for Literal {
    fn match_token(tt: TokenTree) -> Option<Self> {
        if let TokenTree::Literal(lit) = tt {
            Some(lit)
        } else {
            None
        }
    }
}