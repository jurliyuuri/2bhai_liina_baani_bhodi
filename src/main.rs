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

#[derive(Clone, Debug)]
pub enum Foo {
    L(String),
    C(&'static str, String, Vec<Foo>),
}

pub fn h3(ind: usize, t: &str) -> Foo {
    Foo::L(format!(r##"<h3><a name="TOC--{}"></a>{}</h3>"##, ind, t))
}

impl Foo {
    pub fn ls(s: &'static str) -> Foo {
        Foo::L(S(s))
    }
    pub fn c(s: &'static str, v: Vec<Foo>) -> Foo {
        Foo::C(s, S(">"), v)
    }

    pub fn c1(s: &'static str, v: Foo) -> Foo {
        Foo::C(s, S(">"), vec![v])
    }

    pub fn ol(k: &[String]) -> Foo {
        Foo::c(
            "ol",
            k.iter()
                .map(|a| Foo::L(format!("<li>{}</li>", a)))
                .collect(),
        )
    }

    pub fn ul(k: &[String]) -> Foo {
        Foo::c(
            "ul",
            k.iter()
                .map(|a| Foo::L(format!("<li>{}</li>", a)))
                .collect(),
        )
    }

    pub fn strs(&self) -> Vec<String> {
        match self {
            Foo::L(s) => vec![s.clone()],
            Foo::C(tagname, tag_remaining, t) => {
                let mut ans = vec![format!("<{}{}", tagname, tag_remaining)];
                for a in t {
                    let mut k: Vec<_> = a.strs().into_iter().map(|b| format!("  {}", b)).collect();
                    ans.append(&mut k);
                }
                ans.push(format!("</{}>", tagname));
                ans
            }
        }
    }
}

impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.strs().join("\n"))
    }
}

fn generate_toc<S>(toc: Vec<(S, Vec<&str>)>) -> String
where
    S: Into<String>,
{
    let mut global_ind = 0;
    Foo::C(
        "ol",
        S(r##" class="goog-toc">"##),
        toc.into_iter()
            .enumerate()
            .map(|(sec_num_minus_1, t)| {
                Foo::C(
                    "li",
                    format!(
                        r##" class="goog-toc"><a href="#TOC--{}"><strong>{} </strong>{}</a>"##,
                        if global_ind == 0 {
                            S("")
                        } else {
                            format!("{}", global_ind)
                        },
                        sec_num_minus_1 + 1,
                        t.0.into()
                    ),
                    vec![Foo::C("ol", S(r##" class="goog-toc">"##), {
                        let mut v = vec![];
                        global_ind += 1;
                        let mut subsec_num = 1;
                        for a in t.1 {
                            v.push(Foo::L(format!(
                                r##"<li class="goog-toc"><a href="#TOC--{}"><strong>{}.{}
          </strong>{}</a></li>"##,
                                global_ind,
                                sec_num_minus_1 + 1,
                                subsec_num,
                                a
                            )));
                            global_ind += 1;
                            subsec_num += 1;
                        }

                        v
                    })],
                )
            })
            .collect(),
    )
    .to_string()
}

use lang::*;

mod lang;

struct LinziPortion {
    init: Vec<Bar>,
    v1: Vec<(&'static str, Bar)>,
    grau_prua_yr: &'static str,
    v2: Vec<(&'static str, Bar)>,
}

impl LinziPortion {

pub fn render(self, ind: &mut usize) -> Vec<Foo> {
    let LinziPortion {
        init,
        v1,
        grau_prua_yr,
        v2,
    } = self;
    let mut ans = vec![
        Foo::ls(r##"<h2><a name="TOC--"></a>燐字</h2>"##),
        Foo::c1("div", Foo::ls("<hr>")),
    ];
    ans.append(&mut init.iter().map(|a| (*a).clone().into()).collect());
    for (a, b) in v1 {
        *ind += 1;
        ans.push(h3(*ind, a));
        ans.push(b.into());
    }

    ans.push(Foo::ls(r##"<div></div>"##));
    ans.push(Foo::L(format!(
        r##"<div><img src="{}" width="200" height="91" border="0"></div>"##,
        grau_prua_yr
    )));
    ans.push(Foo::ls(r##"<div></div>"##));

    for (a, b) in v2 {
        *ind += 1;
        ans.push(h3(*ind, a));
        ans.push(b.into());
    }
    ans.push(Bar::DivText(S("<br>")).into());
    ans
}

}
struct Hoge(Vec<LangHoge>);

struct LangHoge {
    lang: Lang,
    contents: Vec<(&'static str, Bar)>,
}

fn hoge(l: LinziPortion, dat: Hoge) -> (String, Foo) {
    let mut toc = vec![(
        S("燐字"),
        vec!["字源", "キャスカ・ファルザーの字源", "意義"],
    )]; // FIXME

    for LangHoge { lang, contents } in &dat.0 {
        toc.push((lang.ja(), contents.iter().map(|a| a.0).collect()));
    }

    let mut ind = 0;

    let linzi_portion = l.render(&mut ind);

    let mut v = vec![linzi_portion];
    for LangHoge { lang, contents: k } in dat.0 {
        v.push(bar(lang, k, &mut ind))
    }
    let cont = Foo::c(
        "article",
        v.into_iter().map(|lang| Foo::c("section", lang)).collect(),
    );

    (generate_toc(toc), cont)
}

mod bar {
    use super::*;

    #[derive(Debug, Clone)]
    pub enum Bar {
        DivText(String),
        Ul(Vec<String>),
        Ol(Vec<String>),
    }

    impl Into<Foo> for Bar {
        fn into(self) -> Foo {
            match self {
                Bar::DivText(ref s) => Foo::L(format!("<div>{}</div>", s)),
                Bar::Ul(v) => Foo::ul(&v),
                Bar::Ol(v) => Foo::ol(&v),
            }
        }
    }

    pub fn bar(lang: lang::Lang, v: Vec<(&'static str, Bar)>, ind: &mut usize) -> Vec<Foo> {
        *ind += 1;
        let mut ans = vec![lang.h2(*ind), Foo::c1("div", Foo::ls("<hr>"))];
        for (a, b) in v {
            *ind += 1;
            ans.push(h3(*ind, a));
            ans.push(b.into());
        }
        ans.push(Bar::DivText(S("<br>")).into());
        ans
    }
}

use bar::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        let cont = std::fs::read_to_string(format!("src/contents/{}_contents.html", linzi))?;
        let toc = generate_toc(toc);
        write_page_raw(linzi, toc, cont)?;
    }

    write_page(
        "在",
        LinziPortion{init: vec![
            Bar::DivText(S(r##"<img src="linzi/在.png" border="0">"##)).into(),
            Bar::DivText(S("総画：4")).into(),
            Bar::DivText(S("筆順：丶ノ一一")).into(),
        ], v1: vec![
            ("字源", Bar::Ul(vec![S(r##"象形指事。「<a href="処%20-%20燐字海.html">処</a>」を強調したもの。"##)]).into()),
            ("キャスカ・ファルザーの字源", Bar::Ul(vec![S("呪術において使われる祭壇に乗せられる器を表す。器に供え物を置くという行為が、文化的な観点で強く「存在」を表したために、一般的な存在の意に転義した。")]).into()),
        ], grau_prua_yr: "grau_prua_yr/在.png", v2: vec![
            ("意義", Bar::Ol(vec![S(r##"在る。"##)]).into()),
        ]},
        Hoge(vec![
            LangHoge {
                lang: Lang::Proto,
                contents: vec![
                    ("発音", Bar::DivText(S("aimq")).into()),
                    ("名詞", Bar::DivText(S("存在。")).into()),
                    ("述詞", Bar::DivText(S("在る。～している。")).into()),
                ],
            },
            LangHoge {
                lang: Lang::Air,
                contents: vec![
                    ("発音", Bar::DivText(S("aima")).into()),
                    ("動詞", Bar::DivText(S("在る。")).into()),
                ],
            },
            LangHoge {
                lang: Lang::Paige,
                contents: vec![
                    (
                        "発音",
                        Bar::Ul(vec![
                            S("標準パイグ語：aim2"),
                            S("アイツォ語：aim2"),
                            S("古音：raim"),
                            S("韻図音：冠在素"),
                        ])
                        .into(),
                    ),
                    ("名詞", Bar::DivText(S("存在。")).into()),
                    ("動詞", Bar::DivText(S("在る。")).into()),
                    ("定詞", Bar::DivText(S("～している。")).into()),
                    ("叫詞", Bar::DivText(S("はい。")).into()),
                ],
            },
            LangHoge {
                lang: Lang::Takang,
                contents: vec![
                    (
                        "発音",
                        Bar::Ul(vec![
                            S("皇音：えま、え-む"),
                            S("牌音　古音：アイ　新音：エン"),
                        ])
                        .into(),
                    ),
                    ("名詞", Bar::DivText(S("（えま）存在。")).into()),
                    ("動詞", Bar::DivText(S("（え-む）ある。～している。")).into()),
                ],
            },
            LangHoge {
                lang: Lang::Ezzia,
                contents: vec![
                    (
                        "発音",
                        Bar::Ul(vec![
                            S(r##"光音：あいま"##),
                            S(r##"皇音：え、えむ"##),
                            S(r##"牌音　古音：ラン　現音：アン"##),
                        ])
                        .into(),
                    ),
                    ("名詞", Bar::DivText(S("存在、あること")).into()),
                    (
                        "動詞",
                        Bar::DivText(S("（えま、アン）在る、存在する　（あいま）行う、実行する"))
                            .into(),
                    ),
                ],
            },
            LangHoge {
                lang: Lang::Bhat,
                contents: vec![
                    ("発音", Bar::DivText(S("hemúl, hem")).into()),
                    ("動詞", Bar::DivText(S("(hemúl) ある。")).into()),
                    (
                        "無変化動詞",
                        Bar::DivText(S("(hem) 完了の無変化動詞。〜である。")).into(),
                    ),
                ],
            },
            LangHoge {
                lang: Lang::Lineparine,
                contents: vec![
                    (
                        "発音",
                        Bar::Ol(vec![S("es e\'i"), S("teles"), S("mol"), S("molo"), S("molerl")]).into(),
                    ),
                    ("名詞", Bar::DivText(S("在ること、存在")).into()),
                    (
                        "動詞",
                        Bar::DivText(S(
                            r##"行う、存在する（行うの文脈の場合、目的語があるならtelesで、無い場合はes e'iで訓読する。）"##,
                        )).into(),
                    ),
                    (
                        "熟語",
                        Bar::Ol(vec![S(
                            r##"<a href="真%20-%20燐字海.html">真</a>在　xinien la deliume　＜本分、本来の義務＞"##,
                        )]).into(),
                    ),
                ],
            },
        ]),
    )
}

fn write_page(linzi: &str, l: LinziPortion, h: Hoge) -> Result<(), Box<dyn std::error::Error>> {
    let (toc, cont) = hoge(l,h);
    write_page_raw(linzi, toc, cont.to_string())
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
