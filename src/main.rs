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

    for (linzi, toc) in vec![
        ("一", include!("toc/一_toc.rs")),
        ("七", include!("toc/七_toc.rs")),
        ("万", include!("toc/万_toc.rs")),
        ("三", include!("toc/三_toc.rs")),
        ("上", include!("toc/上_toc.rs")),
        ("下", include!("toc/下_toc.rs")),
        ("与", include!("toc/与_toc.rs")),
        ("中", include!("toc/中_toc.rs")),
        ("之", include!("toc/之_toc.rs")),
        ("乎", include!("toc/乎_toc.rs")),
        ("九", include!("toc/九_toc.rs")),
        ("二", include!("toc/二_toc.rs")),
        ("互", include!("toc/互_toc.rs")),
        ("五", include!("toc/五_toc.rs")),
        ("亦", include!("toc/亦_toc.rs")),
        ("人", include!("toc/人_toc.rs")),
        ("位", include!("toc/位_toc.rs")),
        ("低", include!("toc/低_toc.rs")),
        ("何", include!("toc/何_toc.rs")),
        ("使", include!("toc/使_toc.rs")),
        ("倉", include!("toc/倉_toc.rs")),
        ("値", include!("toc/値_toc.rs")),
        ("光", include!("toc/光_toc.rs")),
        ("党", include!("toc/党_toc.rs")),
        ("入", include!("toc/入_toc.rs")),
        ("八", include!("toc/八_toc.rs")),
        ("六", include!("toc/六_toc.rs")),
        ("兵", include!("toc/兵_toc.rs")),
        ("内", include!("toc/内_toc.rs")),
        ("再", include!("toc/再_toc.rs")),
        ("冠", include!("toc/冠_toc.rs")),
        ("処", include!("toc/処_toc.rs")),
        ("出", include!("toc/出_toc.rs")),
        ("刀", include!("toc/刀_toc.rs")),
        ("別", include!("toc/別_toc.rs")),
        ("力", include!("toc/力_toc.rs")),
        ("加", include!("toc/加_toc.rs")),
        ("勿", include!("toc/勿_toc.rs")),
        ("北", include!("toc/北_toc.rs")),
        ("南", include!("toc/南_toc.rs")),
        ("友", include!("toc/友_toc.rs")),
        ("受", include!("toc/受_toc.rs")),
        ("口", include!("toc/口_toc.rs")),
        ("古", include!("toc/古_toc.rs")),
        ("右", include!("toc/右_toc.rs")),
        ("同", include!("toc/同_toc.rs")),
        ("名", include!("toc/名_toc.rs")),
        ("味", include!("toc/味_toc.rs")),
        ("哩", include!("toc/哩_toc.rs")),
        ("唯", include!("toc/唯_toc.rs")),
        ("四", include!("toc/四_toc.rs")),
        ("字", include!("toc/字_toc.rs")),
        ("心", include!("toc/心_toc.rs")),
        ("手", include!("toc/手_toc.rs")),
        ("水", include!("toc/水_toc.rs")),
        ("火", include!("toc/火_toc.rs")),
        ("無", include!("toc/無_toc.rs")),
        ("皇", include!("toc/皇_toc.rs")),
        ("神", include!("toc/神_toc.rs")),
        ("筆", include!("toc/筆_toc.rs")),
        ("行", include!("toc/行_toc.rs")),
        ("言", include!("toc/言_toc.rs")),
        ("足", include!("toc/足_toc.rs")),
        ("闇", include!("toc/闇_toc.rs")),
    ] {
        let mut toc_num = 0;
        let mut cont = String::from("<article>\n");
        {
            let json_path = format!("{i}/{linzi}_{i}.json", linzi = linzi, i = 1);
            let s = std::fs::read_to_string(json_path.clone())
                .expect(&format!("{path} not found", path = json_path.clone()));

            let linzi_portion = serde_json::from_str::<LinziPortion>(&s)
                .expect(&(S("failed to parse LinziPortion JSON in ") + &json_path));
            cont += &textwrap::indent(
                &linzi_portion
                    .lenticular_to_link()
                    .unwrap()
                    .render_(&mut toc_num),
                "  ",
            );
        }
        for i in 2..=8 {
            let json_path = format!("{i}/{linzi}_{i}.json", linzi = linzi, i = i);
            let s = std::fs::read_to_string(json_path.clone())
                .expect(&format!("{path} not found", path = json_path.clone()));
            let lang_entry = serde_json::from_str::<LangEntry>(&s).unwrap();
            cont += &textwrap::indent(
                &render_lang_entry_(&lang_entry.lenticular_to_link().unwrap(), &mut toc_num),
                "  ",
            );
        }
        cont += "</article>";

        write_page_raw(linzi, generate_toc(toc), cont)?;
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
