use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, Literal, LexError, TokenStream, TokenTree};
use crate::IsToken;

pub struct TokenSoup {
    pub tokens: Vec<TokenTree>, // store the stream
}

impl TokenSoup {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            tokens: stream.into_iter().collect(),
        }
    }
	pub fn get_struct_name(&self, ) -> Option<Ident> {
		let mut iter = self.tokens.clone().into_iter();
	
		while let Some(tt) = iter.next() {
			if let TokenTree::Ident(ref keyword) = tt {
				if keyword.to_string() == "struct" {
					// Next token should be the struct name
					if let Some(TokenTree::Ident(name)) = iter.next() {
						return Some(name);
					}
				}
			}
		}
	
		None
	}

    pub fn find_ident(&self, keyword: &str) -> Vec<&Ident> {
        self.tokens.iter().filter_map(|tt| {
            if let TokenTree::Ident(ident) = tt {
                if ident.to_string().contains(keyword) {
                    Some(ident)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect()
    }

	pub fn find_idents(&self, keyword: &str) -> Vec<Ident> {
        fn walk(tokens: impl Iterator<Item = TokenTree>, keyword: &str, found: &mut Vec<Ident>) {
            for tt in tokens {
                match tt {
                    TokenTree::Ident(ident) => {
                        if ident.to_string().contains(keyword) {
                            found.push(ident);
                        }
                    }
                    TokenTree::Group(group) => {
                        walk(group.stream().into_iter(), keyword, found);
                    }
                    _ => {}
                }
            }
        }

        let mut found = vec![];
        walk(self.tokens.clone().into_iter(), keyword, &mut found);
        found
    }

	/// Recursively find all tokens of a given type T
	pub fn find_all<T: IsToken>(&self) -> Vec<T> {
		fn walk<T: IsToken>(tokens: impl Iterator<Item = TokenTree>, out: &mut Vec<T>) {
			for tt in tokens {
				if let Some(hit) = T::match_token(tt.clone()) {
					out.push(hit);
				}
				if let TokenTree::Group(group) = tt {
					walk(group.stream().into_iter(), out);
				}
			}
		}

		let mut found = vec![];
		walk::<T>(self.tokens.clone().into_iter(), &mut found);
		found
	}
}