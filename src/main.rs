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

fn generate_toc(toc: Vec<(String, Vec<&str>)>) -> String {
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
                        t.0
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

fn bar(lang: lang::Lang, v: Vec<(&'static str, Foo)>, ind: &mut usize) -> Vec<Foo> {
    *ind += 1;
    let mut ans = vec![lang.h2(*ind), Foo::c1("div", Foo::ls("<hr>"))];
    for (a, b) in v {
        *ind += 1;
        ans.push(h3(*ind, a));
        ans.push(b);
    }
    ans.push(Foo::ls("<div><br></div>"));
    ans
}

fn baz(
    init: Vec<Foo>,
    v1: Vec<(&'static str, Foo)>,
    grau_prua_yr: &'static str,
    v2: Vec<(&'static str, Foo)>,
    ind: &mut usize,
) -> Vec<Foo> {
    let mut ans = vec![
        Foo::ls(r##"<h2><a name="TOC--"></a>燐字</h2>"##),
        Foo::c1("div", Foo::ls("<hr>")),
    ];
    ans.append(&mut init.clone());
    for (a, b) in v1 {
        *ind += 1;
        ans.push(h3(*ind, a));
        ans.push(b);
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
        ans.push(b);
    }
    ans.push(Foo::ls("<div><br></div>"));
    ans
}

struct Hoge(Vec<LangHoge>);

struct LangHoge {
    lang: Lang,
    contents: Vec<(&'static str, Foo)>,
}

fn hoge(dat: Hoge) -> (String, Foo) {
    let mut toc = vec![
        (S("燐字"), vec!["字源", "キャスカ・ファルザーの字源", "意義"]),
    ];

    for LangHoge{ lang, contents} in &dat.0 {
        toc.push((lang.ja(), contents.iter().map(|a| a.0).collect()));
    }

    let mut ind = 0;

    let mut v = vec![
        baz(vec![
            Foo::ls(r##"<div><img src="linzi/在.png" border="0"></div>"##),
            Foo::ls(r##"<div>総画：4</div>"##),
            Foo::ls(r##"<div>筆順：丶ノ一一</div>"##),
        ], vec![
            ("字源", Foo::ul(&[S(r##"象形指事。「<a href="処%20-%20燐字海.html">処</a>」を強調したもの。"##)])),
            ("キャスカ・ファルザーの字源", Foo::ul(&[S("呪術において使われる祭壇に乗せられる器を表す。器に供え物を置くという行為が、文化的な観点で強く「存在」を表したために、一般的な存在の意に転義した。")]),),
        ], "grau_prua_yr/在.png", vec![
            ("意義", Foo::c1("div", Foo::c1("ol", Foo::ls(r##"<li>在る。</li>"##))))
        ],
        &mut ind),
    ];

    for LangHoge { lang, contents: k } in dat.0 {
        v.push(bar(lang, k, &mut ind))
    }
    let cont = Foo::c(
        "article",
        v.into_iter().map(|lang| Foo::c("section", lang)).collect(),
    );

    (generate_toc(toc), cont)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(format!("docs/{} - 燐字海.html", "在"))?;
    let (toc, cont) = hoge(Hoge(vec![
        LangHoge {
            lang: Lang::Proto,
            contents: vec![
                ("発音", Foo::ls(r##"<div>aimq</div>"##)),
                ("名詞", Foo::ls(r##"<div>存在。</div>"##)),
                ("述詞", Foo::ls(r##"<div>在る。～している。</div>"##)),
            ],
        },
        LangHoge {
            lang: Lang::Air,
            contents: vec![
                ("発音", Foo::ls(r##"<div>aima</div>"##)),
                ("動詞", Foo::ls(r##"<div>在る。</div>"##)),
            ],
        },
        LangHoge {
            lang: Lang::Paige,
            contents: vec![
                (
                    "発音",
                    Foo::ul(&[
                        S(r##"標準パイグ語：aim2"##),
                        S(r##"アイツォ語：aim2"##),
                        S(r##"古音：raim"##),
                        S(r##"韻図音：冠在素"##),
                    ]),
                ),
                ("名詞", Foo::ls(r##"<div>存在。</div>"##)),
                ("動詞", Foo::ls(r##"<div>在る。</div>"##)),
                ("定詞", Foo::ls(r##"<div>～している。</div>"##)),
                ("叫詞", Foo::ls("<div>はい。</div>")),
            ],
        },
        LangHoge {
            lang: Lang::Takang,
            contents: vec![
                (
                    "発音",
                    Foo::ul(&[
                        S(r##"皇音：えま、え-む"##),
                        S(r##"牌音　古音：アイ　新音：エン"##),
                    ]),
                ),
                ("名詞", Foo::ls(r##"（えま）存在。"##)),
                ("動詞", Foo::ls(r##"（え-む）ある。～している。"##)),
            ],
        },
        LangHoge {
            lang: Lang::Ezzia,
            contents: vec![
                (
                    "発音",
                    Foo::ul(&[
                        S(r##"光音：あいま"##),
                        S(r##"皇音：え、えむ"##),
                        S(r##"牌音　古音：ラン　現音：アン"##),
                    ]),
                ),
                ("名詞", Foo::ls(r##"<div>存在、あること</div>"##)),
                (
                    "動詞",
                    Foo::ls(
                        r##"<div>（えま、アン）在る、存在する　（あいま）行う、実行する</div>"##,
                    ),
                ),
            ],
        },
        LangHoge {
            lang: Lang::Bhat,
            contents: vec![
                ("発音", Foo::ls(r##"<div>hemúl, hem</div>"##)),
                ("動詞", Foo::ls(r##"<div>(hemúl) ある。</div>"##)),
                (
                    "無変化動詞",
                    Foo::ls(r##"<div>(hem) 完了の無変化動詞。〜である。</div>"##),
                ),
            ],
        },
        LangHoge {
            lang: Lang::Lineparine,
            contents: vec![
                (
                    "発音",
                    Foo::ol(&[S("es e\'i"), S("teles"), S("mol"), S("molo"), S("molerl")]),
                ),
                ("名詞", Foo::ls("<div>在ること、存在</div>")),
                (
                    "動詞",
                    Foo::ls(
                        r##"行う、存在する（行うの文脈の場合、目的語があるならtelesで、無い場合はes e'iで訓読する。）"##,
                    ),
                ),
                (
                    "熟語",
                    Foo::ol(&[S(
                        r##"<a href="真%20-%20燐字海.html">真</a>在　xinien la deliume　＜本分、本来の義務＞"##,
                    )]),
                ),
            ],
        },
    ]));
    write!(
        file,
        "{}",
        LinzklarTemplate {
            linzi: "在",
            toc: &toc,
            content: &cont.to_string()
        }
        .render()
        .unwrap()
    )?;

    Ok(())
}
