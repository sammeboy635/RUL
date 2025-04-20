use TokenSoup::TokenSoup; // crate name = package name from Cargo.toml

use proc_macro2::{Group, Ident, TokenStream};

fn ts(input: &str) -> TokenSoup {
	let token = input.parse().unwrap();
	println!("\n{:#?}\n", token);
	TokenSoup::new(token)
}

#[test]
fn test_get_struct_name() {
	let soup: TokenSoup = ts(r#"
struct Test {
	#[bits = 4]
	field1: u8,
	field2: u16,
}
"#);
	let struct_ident = soup.get_struct_name().unwrap();
	assert_eq!(struct_ident.to_string(), "Test".to_string());
}


#[test]
fn test_find_all() {
	let soup: TokenSoup = ts(r#"
	fn goodbye() {
		println!("Goodbye from marked function!");
	}
"#);
	let groups = soup.find_all::<Group>();
	println!("{:#?}", groups);
	// assert_eq!(struct_ident.to_string(), "Test".to_string());
}