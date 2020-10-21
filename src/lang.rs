use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lang(pub String);

use big_s::S;
use log::warn;
use std::collections::HashMap;

impl Lang {
    pub fn url(&self) -> String {
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
        match HASHMAP.get(&self.0) {
            Some(u) => u.to_owned(),
            None => {
                warn!("Unknown language name `{}`; unable to create a link. If this is not a typo, please add it to config/links.tsv", self.0);
                S("")
            }
        }
    }

    pub fn ja(&self) -> String {
        self.0.to_owned()
    }
}
