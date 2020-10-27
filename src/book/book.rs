use serde::{Deserialize};

#[derive(Debug,Clone,Deserialize)]
pub struct Book {
    pub title: String,
    //Optional vector of strings
    pub authors: Option<Vec<String>>,
}


