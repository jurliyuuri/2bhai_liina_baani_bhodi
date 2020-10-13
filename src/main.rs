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

fn generate_toc(toc: Vec<(&str, Vec<&str>)>) -> String {
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

fn content(langs: Vec<Vec<Foo>>) -> String {
    Foo::c(
        "article",
        langs
            .into_iter()
            .map(|lang| Foo::c("section", lang))
            .collect(),
    )
    .to_string()
}

use lang::*;

mod lang;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(format!("docs/{} - 燐字海.html", "在"))?;
    write!(
        file,
        "{}",
        LinzklarTemplate {
            linzi: "在",
            toc: &generate_toc(vec![
                ("燐字", vec!["字源", "キャスカ・ファルザーの字源", "意義"]),
                ("ラネーメ祖語", vec!["発音", "名詞", "述詞"]),
                ("アイル語", vec!["発音", "動詞"]),
                ("パイグ語", vec!["発音", "名詞", "動詞", "定詞", "叫詞"]),
                ("タカン語", vec!["発音", "名詞", "動詞"]),
                ("エッツィア語", vec!["発音", "名詞", "動詞"]),
                ("バート語", vec!["発音", "動詞", "無変化動詞"]),
                ("リパライン語", vec!["発音", "名詞", "動詞", "熟語"])
            ]),
            content: &content(vec![
                linmarn(),
                vec![
                    lang::Lang::Proto.h2(4),
                    Foo::c1("div", Foo::ls("<hr>")),
                    h3(5, "発音"),
                    Foo::ls(r##"<div>aimq</div>"##),
                    h3(6, "名詞"),
                    Foo::ls(r##"<div>存在。</div>"##),
                    h3(7, "述詞"),
                    Foo::ls(r##"<div>在る。～している。</div>"##),
                ],
                vec![
                    Lang::Air.h2(8),
                    Foo::c1("div", Foo::ls("<hr>")),
                    h3(9, "発音"),
                    Foo::ls(r##"<div>aima</div>"##),
                    h3(10, "動詞"),
                    Foo::ls(r##"<div>在る。</div>"##),
                ],
                pekzep(),
                takang(),
                ezzia(),
                vec![
                    Lang::Bhat.h2(25),
                    Foo::c1("div", Foo::ls("<hr>")),
                    h3(26, "発音"),
                    Foo::ls(r##"<div>hemúl, hem</div>"##),
                    h3(27, "動詞"),
                    Foo::ls(r##"<div>(hemúl) ある。</div>"##),
                    h3(28, "無変化動詞"),
                    Foo::ls(r##"<div>(hem) 完了の無変化動詞。〜である。</div>"##),
                    Foo::ls(r##"<div><br></div>"##),
                ],
                vec![
                    Lang::Lineparine.h2(29),
                    Foo::c1("div", Foo::ls("<hr>")),
                    h3(30, "発音"),
                    Foo::ol(&[S("es e\'i"), S("teles"), S("mol"), S("molo"), S("molerl")]),
                    h3(31, "名詞"),
                    Foo::ls("<div>在ること、存在</div>"),
                    h3(32, "動詞"),
                    Foo::ls(
                        r##"行う、存在する（行うの文脈の場合、目的語があるならtelesで、無い場合はes e'iで訓読する。）"##,
                    ),
                    h3(33, "熟語"),
                    Foo::ol(&[S(
                        r##"<a href="真%20-%20燐字海.html">真</a>在　xinien la deliume　＜本分、本来の義務＞"##,
                    )]),
                ],
            ])
        }
        .render()
        .unwrap()
    )?;

    Ok(())
}
