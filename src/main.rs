#![allow(dead_code)]
#![allow(unused_imports)]

use bitpack::{BitPacked, my_attr};
use my_lib::{register_my_fn, MY_FN_REGISTRY};

use ctor::{BitPackedd, hello_macro, HelperAttr};

// #[derive(BitPacked, Debug)]
// #[endian("be")]
// struct MyPackedStruct {
//     #[bits = 4]
//     field1: u8,
//     #[bits = 4]
//     field2: u8,
//     #[bits = 15]
//     field3: u16,
// 	#[bits = 33]
//     field4: u64,
// }

#[derive(HelperAttr)]
struct Struct {
    #[helper] field: ()
}

#[derive(BitPackedd, Debug)]
struct MyOtherPacked {
    #[bits = 4]
    field1: u8,
    #[bits = 4]
    field2: u8,
    #[bits = 15]
    field3: u16,
	#[bits = 33]
    field4: u64,
}

// #[hello_macro]
// fn goodbye() {
//     println!("Goodbye from marked function!");
// }

// #[ctor::ctor]
// fn goodbye_test() {
//     println!("Goodbye from marked function!");
// }

fn main() {

	// let my_struct = MyOtherPacked {
    //     field1: 15,
    //     field2: 10,
    //     field3: 0xFFFF,
    //     field4: 0xFFFFFFFF,
    // };
	// let packed = my_struct.pack();
	// println!("{:?}", packed);
	// let unpack = MyPackedStruct::unpack(packed);
	// println!("{:#?}", unpack);



	// let funcs = MY_FN_REGISTRY.lock().unwrap();
    // for f in funcs.iter() {
    //     f(); // Call the function
    // }
}

