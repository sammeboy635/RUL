// lib.rs or somewhere accessible

use once_cell::sync::Lazy;
use std::sync::Mutex;

type MyFn = fn();

pub static MY_FN_REGISTRY: Lazy<Mutex<Vec<MyFn>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn register_my_fn(f: MyFn) {
    MY_FN_REGISTRY.lock().unwrap().push(f);
}
