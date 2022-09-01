// extern crate serde_json;
// extern crate wasm_bindgen;
// extern crate web_sys;
// use wasm_bindgen::prelude::*;

// #[macro_use]
// extern crate serde_derive;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
