
use std::str::FromStr;

use proc_macro::{Ident, Span, TokenStream};
use quote::{quote, ToTokens, format_ident};
use syn::{parse_macro_input, Data, DeriveInput, ItemFn};
use ctor::ctor;

static IDENT_ENDIAN: &str = "endian";
static IDENT_BITS: &str = "bits";

enum Endian {
    Little,
    Big,
}

impl std::str::FromStr for Endian {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "le" | "little" => Ok(Endian::Little),
            "be" | "big"    => Ok(Endian::Big),
            _ => Err("Invalid endian string"),
        }
    }
}

#[proc_macro_derive(BitPacked, attributes(bits, endian))]
pub fn bit_packed_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
	println!("{:#?}", input);
	// parse_endian_input(&input);
    let struct_name = &input.ident;

    let data = match &input.data {
        Data::Struct(data) => data,
        _ => panic!("#[BitPacked] can only be used on structs."),
    };

	let mut bit_cursor = 0;
	let mut pack_code = vec![];
    let mut unpack_code = vec![];
	let mut unpack_name = vec![];


    // Debug print the fields
    for field in &data.fields {
		let mut bits = 0;
		let field_name = &field.ident;
		let field_type = &field.ty;

		let bits_attr = match field.attrs.iter().find(|attr| attr.path().is_ident(IDENT_BITS)) {
			Some(attr) => attr,
			None => continue,
		};

		if let Some(result) = parse_bits_attr(bits_attr) {
			bits = match result {
				Ok(bits) => bits,
				Err(err) => return err.to_compile_error().into(),
			}
		}

		let mut remaining = bits;
		let mut shift_from_field = 0;

		unpack_code.push(quote! {
			let mut #field_name: #field_type = 0;
		});

		unpack_name.push(quote! {
			#field_name
		});

		while remaining > 0 {
			let byte_index = bit_cursor / 8;
			let bit_offset = bit_cursor % 8;
			let available_in_byte = 8 - bit_offset;
			let bits_to_write = remaining.min(available_in_byte);
			let mask = (1u64 << bits_to_write) - 1;
			
			// println!("{:?}",remaining);
			pack_code.push(quote! {
				packed[#byte_index] |= ((self.#field_name >> #shift_from_field) as u8 & (#mask as u8)) << #bit_offset;
			});
			
			unpack_code.push(quote! {
				#field_name |= (((packed[#byte_index] as #field_type >> #bit_offset) & (#mask as #field_type))) << #shift_from_field;
			});

			shift_from_field += bits_to_write;
			bit_cursor += bits_to_write;
			remaining -= bits_to_write;
		}
	
		// println!("Field: {:#?}", field);
    }
	let mut final_byte_size = bit_cursor / 8;
	if bit_cursor % 8 != 0
	{
		final_byte_size += 1;
	}
	
	let expanded = quote! {
        impl #struct_name {
            // Method to pack the fields into a u64
            pub fn pack(&self) -> Vec<u8> {
                let mut packed: Vec<u8> = vec![0; #final_byte_size];
                #(#pack_code)*
                packed
            }

            // Method to unpack from a u64
            pub fn unpack(packed: Vec<u8>) -> Self {
				#(#unpack_code)*
				
                Self {
                    #(#unpack_name,)*
                }
            }
        }
    };
	// println!("{}", expanded.to_string());
    TokenStream::from(expanded)
}

fn parse_endian_input(input: &DeriveInput) -> Result<Endian, syn::Error>
{
	let attr = match input.attrs.iter().find(|attr| attr.path().is_ident(IDENT_ENDIAN)) {
		Some(val) => val,
		None => return Ok(Endian::Little),
	};

	let lit_str = match &attr.meta {
        syn::Meta::NameValue(mnv) => match &mnv.value {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(lit_str) => lit_str.clone(),
                _ => return Err(syn::Error::new_spanned(mnv.value.clone(), "Invalid endian attribute value")),
            },
            _ => return Err(syn::Error::new_spanned(mnv.value.clone(), "Invalid endian attribute value")),
        },
        syn::Meta::List(ml) => match ml.parse_args::<syn::LitStr>() {
            Ok(value) => value,
            Err(err) => return Err(syn::Error::new_spanned(ml, format!("Invalid endian attribute value: {}", err))),
        },
        _ => return Err(syn::Error::new_spanned(attr, "Invalid endian attribute")),
    };

	match Endian::from_str(lit_str.value().as_str()) {
		Ok(val) => Ok(val),
		Err(val) => Err(syn::Error::new_spanned(attr, val)),
	}
}

fn parse_bits_attr(attr: &syn::Attribute) -> Option<Result<usize, syn::Error>> {
    if !attr.path().is_ident("bits") {
        return None;
    }

    let lit_int = match &attr.meta {
        syn::Meta::NameValue(mnv) => match &mnv.value {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Int(lit_int) => lit_int.clone(),
                _ => return Some(Err(syn::Error::new_spanned(mnv.value.clone(), "Invalid bits attribute value"))),
            },
            _ => return Some(Err(syn::Error::new_spanned(mnv.value.clone(), "Invalid bits attribute value"))),
        },
        syn::Meta::List(ml) => match ml.parse_args::<syn::LitInt>() {
            Ok(value) => value,
            Err(err) => return Some(Err(syn::Error::new_spanned(ml, format!("Invalid bits attribute value: {}", err)))),
        },
        _ => return Some(Err(syn::Error::new_spanned(attr, "Invalid bits attribute"))),
    };

    Some(lit_int.base10_parse::<usize>().map_err(|err| syn::Error::new_spanned(lit_int, format!("Invalid bits attribute value: {}", err))))
}



#[proc_macro_attribute]
pub fn my_attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
	
	println!("Field: {:#?}", fn_name); 
	let ident = format_ident!("register_{}", fn_name_str);
    let expanded = quote! {
        #input

        #[ctor::ctor]
        fn #ident() {
            println!("Registering function: {}", #fn_name_str);
            crate::register_my_fn(#fn_name);
        }
    };

	println!("{}", expanded.to_string());
    TokenStream::from(expanded)
}