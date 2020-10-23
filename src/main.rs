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
use lang::*;

mod markup;

use markup::*;

mod lenticular;
use lenticular::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_LOG", "warn");
    env_logger::init();

    for linzi in vec![
        "一", "七", "万", "三", "上", "下", "与", "中", "之", "乎", "九", "二", "互", "五", "亦",
        "人", "位", "低", "何", "使", "倉", "値", "光", "党", "入", "八", "六", "兵", "内", "再",
        "冠", "処", "出", "刀", "別", "力", "加", "勿", "北", "南", "友", "受", "口", "古", "右",
        "同", "名", "味", "哩", "唯", "四", "字", "心", "手", "水", "火", "無", "皇", "神", "筆",
        "行", "言", "足", "闇",
    ] {
        let json_path = format!("{i}/{linzi}_{i}.json", linzi = linzi, i = 1);
        let s = std::fs::read_to_string(json_path.clone())
            .expect(&format!("{path} not found", path = json_path.clone()));

        let linzi_portion = serde_json::from_str::<LinziPortion>(&s)
            .expect(&(S("failed to parse LinziPortion JSON in ") + &json_path));
        let mut dat = Vec::new();
        for i in 2..=8 {
            let json_path = format!("{i}/{linzi}_{i}.json", linzi = linzi, i = i);
            let s = std::fs::read_to_string(json_path.clone())
                .expect(&format!("{path} not found", path = json_path.clone()));
            let lang_entry = serde_json::from_str::<LangEntry>(&s).unwrap();
            dat.push(lang_entry);
        }

        write_page(
            linzi,
            Article {
                l: linzi_portion,
                dat,
            }
            .lenticular_to_link()
            .unwrap(),
        )?;
    }

    write_page(
        "在",
        serde_json::from_str::<Article>(
            &std::fs::read_to_string(format!("{}.json", "在")).unwrap(),
        )
        .unwrap()
        .lenticular_to_link()
        .unwrap(),
    )
}

fn write_page_raw(
    linzi: &str,
    toc: String,
    cont: String,
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
