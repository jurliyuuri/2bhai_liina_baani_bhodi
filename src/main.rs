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
        let cont = {
            let mut ans = String::new();
            for i in 1..100 {
                match std::fs::read_to_string(format!("src/contents/a/{}_{}.html", linzi, i)) {
                    Ok(s) => {
                        ans += "<section>\n";
                        ans += &s;
                        ans += "\n</section>\n"
                    }
                    Err(_) => break,
                }
            }
            ans
        };
        let toc = generate_toc(toc);
        write_page_raw(linzi, toc, cont)?;
    }

    write_page(
        "在",
        serde_json::from_str(r#"{"l": {
            "init": [
                "<img src=\"linzi/在.png\" border=\"0\">",
                "総画：4",
                "筆順：丶ノ一一"
            ],
            "v1": [
                [
                    "字源", 
                    {
                        "ordered": false,
                        "content": [
                            "象形指事。「<a href=\"処%20-%20燐字海.html\">処</a>」を強調したもの。"
                        ]
                    }
                ],
                [
                    "キャスカ・ファルザーの字源", 
                    {
                        "ordered": false,
                        "content": [
                            "呪術において使われる祭壇に乗せられる器を表す。器に供え物を置くという行為が、文化的な観点で強く「存在」を表したために、一般的な存在の意に転義した。"
                        ]
                    }
                ]
            ],
            "grau_prua_yr": "grau_prua_yr/在.png", 
            "v2": [
                [
                    "意義", 
                    {
                        "ordered": true,
                        "content": ["在る。"]
                    }
                ]
            ]
        }, "dat": [
            {
                "lang": "ラネーメ祖語",
                "contents": [
                    ["発音", "aimq"],
                    ["名詞", "存在。"],
                    ["述詞", "在る。～している。"]
                ]
            }, {
                "lang": "アイル語",
                "contents": [
                    ["発音", "aima"],
                    ["動詞", "在る。"]
                ]
            }, {
                "lang": "パイグ語",
                "contents": [
                    [
                        "発音",
                        {
                            "ordered": false,
                            "content": [
                                "標準パイグ語：aim2",
                                "アイツォ語：aim2",
                                "古音：raim",
                                "韻図音：冠在素"
                            ]
                        }
                    ],
                    ["名詞", "存在。"],
                    ["動詞", "在る。"],
                    ["定詞", "～している。"],
                    ["叫詞", "はい。"]
                ]
            }, {
                "lang": "タカン語",
                "contents":[
                    [
                        "発音", 
                        {
                            "ordered": false,
                            "content": [
                                "皇音：えま、え-む",
                                "牌音　古音：アイ　新音：エン"
                            ]
                        }
                    ],
                    ["名詞", "（えま）存在。"],
                    ["動詞", "（え-む）ある。～している。"]
                ]
            }, {
                "lang": "エッツィア語",
                "contents": [
                    [
                        "発音",
                        {
                            "ordered": false,
                            "content": [
                                "光音：あいま",
                                "皇音：え、えむ",
                                "牌音　古音：ラン　現音：アン"
                            ]
                        }
                    ],
                    [
                        "名詞", "存在、あること"
                    ],
                    [
                        "動詞",
                        "（えま、アン）在る、存在する　（あいま）行う、実行する"
                    ]
                ]
            }, {
                "lang": "バート語",
                "contents": [
                    ["発音", "hemúl, hem"],
                    ["動詞", "(hemúl) ある。"],
                    ["無変化動詞", "(hem) 完了の無変化動詞。〜である。"]
                ]
            }, {
                "lang": "リパライン語",
                "contents": [
                    [
                        "発音",
                        {
                            "ordered": true,
                            "content": [
                                "es e'i", "teles", "mol", "molo", "molerl"
                            ]
                        }
                    ],
                    ["名詞", "在ること、存在"],
                    [
                        "動詞",
                        
                            "行う、存在する（行うの文脈の場合、目的語があるならtelesで、無い場合はes e'iで訓読する。）"
                        
                    ],
                    [
                        "熟語",
                        {
                            "ordered": true,
                            "content": [
                                "<a href=\"真%20-%20燐字海.html\">真</a>在　xinien la deliume　＜本分、本来の義務＞"
                            ]
                        }
                    ]
                ]
            }
        ]}"#).unwrap(),
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
