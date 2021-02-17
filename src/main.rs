#![warn(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]
#[macro_use]
extern crate lazy_static;

use env_logger;

use std::env;

use std::fs::File;
use std::io::prelude::*;

use askama::Template;

#[derive(Template)]
#[template(path = "linzklar.html")]
struct LinzklarTemplate<'a> {
    linzi: &'a str,
    toc: &'a str,
    content: &'a str,
}

use big_s::S;

mod lang;
use lang::Lang;

mod markup;

use markup::{Article, LangEntry, LinziPortion, write_page};

mod lenticular;
use lenticular::{Lenticular, LenticularError};

use glob::glob;

use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "warn");
    env_logger::init();
    let mut linzi_already_handled = HashSet::new();
    for entry in glob("*.json")? {
        let a = entry?;
        let name = a.display().to_string();
        let linzi = name.chars().nth(0).unwrap().to_string();
        if name.chars().nth(1) == Some('.') {
            // all the info is in a single file
            write_page(
                &linzi,
                serde_json::from_str::<Article>(
                    &std::fs::read_to_string(format!("{}.json", linzi)).unwrap(),
                )
                .unwrap()
                .lenticular_to_link()
                .unwrap(),
            )?;
        } else {
            if linzi_already_handled.contains(&linzi) {
                continue;
            }

            let linzi_json_path = format!("{linzi}_燐字.json", linzi = linzi);
            let linzi_str = std::fs::read_to_string(linzi_json_path.clone())
                .expect(&format!("{path} not found", path = linzi_json_path.clone()));

            let linzi_portion = serde_json::from_str::<LinziPortion>(&linzi_str)
                .expect(&(S("failed to parse LinziPortion JSON in ") + &linzi_json_path));
            let mut dat = Vec::new();
            for lang_name in vec![
                "ラネーメ祖語",
                "アイル語",
                "パイグ語",
                "タカン語",
                "エッツィア語",
                "バート語",
                "リパライン語",
            ] {
                let json_path = format!(
                    "{linzi}_{lang_name}.json",
                    linzi = linzi,
                    lang_name = lang_name
                );
                let s = std::fs::read_to_string(json_path.clone())
                    .expect(&format!("{path} not found", path = json_path.clone()));
                let lang_entry = serde_json::from_str::<LangEntry>(&s).unwrap();
                dat.push(lang_entry);
            }

            write_page(
                &linzi,
                Article {
                    l: linzi_portion,
                    dat,
                }
                .lenticular_to_link()
                .unwrap(),
            )?;

            linzi_already_handled.insert(linzi);
        }
    }

    Ok(())
}

fn write_page_raw(
    linzi: &str,
    toc: &str,
    cont: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(format!("docs/{} - 燐字海.html", linzi))?;
    write!(
        file,
        "{}",
        LinzklarTemplate {
            linzi,
            toc: &toc,
            content: &cont
        }
        .render()
        .unwrap()
    )?;

    Ok(())
}
