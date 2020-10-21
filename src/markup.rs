use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bar {
    DivText(String),
    List { ordered: bool, content: Vec<String> },
}

impl Into<IndentedStr> for Bar {
    fn into(self) -> IndentedStr {
        match self {
            Bar::DivText(ref s) => IndentedStr::Line(format!("<div>{}</div>", s)),
            Bar::List {
                ordered,
                content: v,
            } => IndentedStr::c(
                if ordered { "ol" } else { "ul" },
                v.iter()
                    .map(|a| IndentedStr::Line(format!("<li>{}</li>", a)))
                    .collect(),
            ),
        }
    }
}

pub fn write_page(linzi: &str, article: Article) -> Result<(), Box<dyn std::error::Error>> {
    let Article { l, dat } = article;
    let v1_entries: Vec<String> = l.v1.iter().map(|(k, _)| k.to_owned()).collect();
    let v2_entries: Vec<String> = l.v2.iter().map(|(k, _)| k.to_owned()).collect();

    let mut toc = vec![(S("燐字"), [&v1_entries[..], &v2_entries[..]].concat())];

    for LangEntry { lang, contents } in &dat {
        toc.push((lang.ja(), contents.iter().map(|a| a.0.clone()).collect()));
    }

    let mut toc_num = 0;

    let linzi_portion = l.render(&mut toc_num);

    let mut vv = vec![linzi_portion];
    for LangEntry { lang, contents } in dat {
        toc_num += 1;
        let mut ans = vec![
            IndentedStr::with_toc(
                "h2",
                toc_num,
                &format!("<a href=\"{}\">{}</a>", &lang.url(), &lang.ja()),
            ),
            IndentedStr::c1("div", IndentedStr::ls("<hr>")),
        ];
        for (a, b) in contents {
            toc_num += 1;
            ans.push(IndentedStr::with_toc("h3", toc_num, &a));
            ans.push(b.into());
        }
        ans.push(Bar::DivText(S("<br>")).into());

        vv.push(ans)
    }
    let cont = IndentedStr::c(
        "article",
        vv.into_iter()
            .map(|lang| IndentedStr::c("section", lang))
            .collect(),
    );

    write_page_raw(linzi, generate_toc(toc), cont.to_string())
}

#[derive(Clone, Debug)]
enum IndentedStr {
    Line(String),
    Tag(&'static str, String, Vec<IndentedStr>),
}

impl IndentedStr {
    pub fn with_toc(tagname: &'static str, ind: usize, t: &str) -> IndentedStr {
        IndentedStr::Line(format!(
            "<{tagname}><a name=\"TOC--{index}\"></a>{content}</{tagname}>",
            index = ind,
            content = t,
            tagname = tagname
        ))
    }

    pub fn ls(s: &'static str) -> IndentedStr {
        IndentedStr::Line(S(s))
    }
    pub fn c(tagname: &'static str, v: Vec<IndentedStr>) -> IndentedStr {
        IndentedStr::Tag(tagname, S(">"), v)
    }

    pub fn c1(tagname: &'static str, v: IndentedStr) -> IndentedStr {
        IndentedStr::Tag(tagname, S(">"), vec![v])
    }

    pub fn strs(&self) -> Vec<String> {
        match self {
            IndentedStr::Line(s) => vec![s.clone()],
            IndentedStr::Tag(tagname, tag_remaining, t) => {
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

impl std::fmt::Display for IndentedStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.strs().join("\n"))
    }
}

pub fn generate_toc<S, T>(toc: Vec<(S, Vec<T>)>) -> String
where
    S: Into<String>,
    T: Into<String>,
{
    let mut global_ind = 0;
    IndentedStr::Tag(
        "ol",
        S(" class=\"goog-toc\">"),
        toc.into_iter()
            .enumerate()
            .map(|(sec_num_minus_1, t)| {
                IndentedStr::Tag(
                    "li",
                    format!(
                        " class=\"goog-toc\"><a href=\"#TOC--{}\"><strong>{} </strong>{}</a>",
                        if global_ind == 0 {
                            S("")
                        } else {
                            format!("{}", global_ind)
                        },
                        sec_num_minus_1 + 1,
                        t.0.into()
                    ),
                    vec![IndentedStr::Tag("ol", S(" class=\"goog-toc\">"), {
                        let mut v = vec![];
                        global_ind += 1;
                        let mut subsec_num = 1;
                        for a in t.1 {
                            v.push(IndentedStr::Line(format!(
                                "<li class=\"goog-toc\"><a href=\"#TOC--{}\"><strong>{}.{}\n          </strong>{}</a></li>",
                                global_ind,
                                sec_num_minus_1 + 1,
                                subsec_num,
                                a.into()
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinziPortion {
    pub init: Vec<Bar>,
    pub v1: Vec<(String, Bar)>,
    pub grau_prua_yr: String,
    pub v2: Vec<(String, Bar)>,
}

impl LinziPortion {
    fn render(self, ind: &mut usize) -> Vec<IndentedStr> {
        let LinziPortion {
            init,
            v1,
            grau_prua_yr,
            v2,
        } = self;
        let mut ans = vec![
            IndentedStr::ls("<h2><a name=\"TOC--\"></a>燐字</h2>"),
            IndentedStr::c1("div", IndentedStr::ls("<hr>")),
        ];
        ans.append(&mut init.iter().map(|a| (*a).clone().into()).collect());
        for (a, b) in v1 {
            *ind += 1;
            ans.push(IndentedStr::with_toc("h3", *ind, &a));
            ans.push(b.into());
        }

        ans.push(IndentedStr::ls("<div></div>"));
        ans.push(IndentedStr::Line(format!(
            "<div><img src=\"{}\" width=\"200\" height=\"91\" border=\"0\"></div>",
            grau_prua_yr
        )));
        ans.push(IndentedStr::ls("<div></div>"));

        for (a, b) in v2 {
            *ind += 1;
            ans.push(IndentedStr::with_toc("h3", *ind, &a));
            ans.push(b.into());
        }
        ans.push(Bar::DivText(S("<br>")).into());
        ans
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub l: LinziPortion,
    pub dat: Vec<LangEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangEntry {
    pub lang: Lang,
    pub contents: Vec<(String, Bar)>,
}
