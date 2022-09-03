extern crate serde_json;
extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct User {
    first_name: String,
    last_name: String,
    id: String
}

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn concat_names_from_rust() -> String {
    let users: Vec<User> = vec![
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
        User { first_name: String::from("John"), last_name: String::from("Doe"), id: String::from("1")},
    ];

    users
        .iter()
        .map(|user| user.first_name.to_string())
        .collect::<Vec<String>>()
        .join(":")
}

#[wasm_bindgen]
pub fn concat_names_from_js(users: &JsValue) -> String {
    let user_vec: Vec<User> = users.into_serde().unwrap();
    user_vec
        .iter()
        .map(|user| user.first_name.to_string())
        .collect::<Vec<String>>()
        .join(":")
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
