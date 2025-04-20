use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, Literal, LexError, TokenStream, TokenTree};
use token_soup::{TokenSoup, IsToken};


#[proc_macro_attribute]
pub fn hello_macro(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    // optionally inspect _attr
    // return the (possibly modified) item
	// println!("{:#?}", _item);
	// let soup = TokenSoup::new(_item.clone());
	// println!("{:#?}", soup.find_idents_recursive("println"));
	// println!("{:#?}", soup.find_all::<Group>());
	_item
}

#[proc_macro_derive(BitPackedd, attributes(bits, endian))]
pub fn derive_bit_attr(_item: TokenStream) -> TokenStream {
    // input will be the struct or enum
    // You can parse and generate code here
	// println!("{:#?}", _item);
    // println!("Got input: {}", _item.to_string()); // won't print, see notes below
	// let soup = TokenSoup::new(_item.clone());
	// println!("{:#?}",soup.get_struct_name().unwrap());

    // input
	let gen_val = format!(
        "impl MyOtherPacked {{
            fn bit_info() {{
                println!(\"Bit info for MyOtherPacked\");
            }}
        }}"
    );

    gen_val.parse().unwrap()
	// TokenStream::new()
}

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper_attr(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}