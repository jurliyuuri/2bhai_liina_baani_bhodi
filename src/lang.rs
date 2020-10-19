pub struct Lang(pub String);

use std::collections::HashMap;
lazy_static! {
    static ref HASHMAP: HashMap<String, String> = include_str!("../config/links.tsv")
        .lines()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| {
            let v: Vec<&'static str> = line.splitn(2, "\t").collect();
            (v[0].to_owned(), v[1].to_owned())
        })
        .collect::<HashMap<_, _>>();
}

impl Lang {
    pub fn url(&self) -> String {
        match HASHMAP.get(&self.0) {
            Some(u) => u.to_owned(),
            None => panic!("Unknown language name {}", self.0)
        }      
    }

    pub fn ja(&self) -> String {
        self.0.to_owned()
    }
}
