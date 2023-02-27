#![allow(unused)]
use std::{collections::HashMap, string};
type passwords = HashMap<String, String>;
struct BadPassowrdManager {
    master_password: String,
    passwords: passwords,
}
impl BadPassowrdManager {
    //create the new password manager
    pub fn new(master_password: String) -> Self {
        BadPassowrdManager {
            master_password,
            passwords: Default::default(),
        }
    }
    pub fn list_passwords(&self) -> &passwords {
        todo!()
    }
    pub fn add_password(&mut self) {
        todo!()
    }
    pub fn encryption(&self) -> String {
        todo!()
    }
    pub fn version(&self) -> String {
        todo!()
    }
}
