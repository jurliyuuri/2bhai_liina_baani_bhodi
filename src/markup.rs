use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bar {
    DivText(String),
    DivTexts(Vec<String>),
    List { ordered: bool, content: Vec<String> },
}

impl Into<Vec<IndentedStr>> for Bar {
    fn into(self) -> Vec<IndentedStr> {
        match self {
            Bar::DivText(ref s) => vec![IndentedStr::Line(format!("<div>{}</div>", s))],
            Bar::DivTexts(ss) => ss
                .iter()
                .map(|s| IndentedStr::Line(format!("<div>{}</div>", s)))
                .collect(),
            Bar::List {
                ordered,
                content: v,
            } => vec![IndentedStr::c(
                if ordered { "ol" } else { "ul" },
                v.iter()
                    .map(|a| IndentedStr::Line(format!("<li>{}</li>", a)))
                    .collect(),
            )],
        }
    }
}

pub fn render_lang_entry_(lang_entry: &LangEntry, toc_num: &mut usize) -> String {
    render_lang_entry(lang_entry, toc_num).to_string()
}

fn render_lang_entry(lang_entry: &LangEntry, toc_num: &mut usize) -> IndentedStr {
    let LangEntry { lang, contents } = lang_entry;
    *toc_num += 1;
    let mut ans = vec![
        IndentedStr::with_toc(
            "h2",
            *toc_num,
            &format!("<a href=\"{}\">{}</a>", &lang.url(), &lang.ja()),
        ),
        IndentedStr::c1("div", IndentedStr::ls("<hr>")),
    ];
    for (title, b) in contents {
        // if the title is empty, the h3 tag should not be listed in the table of contents
        if title == "" {
            ans.push(IndentedStr::Line(S("<h3></h3>")));
        } else {
            *toc_num += 1;
            ans.push(IndentedStr::with_toc("h3", *toc_num, &title));
        }
        ans.append(&mut b.clone().into());
    }
    ans.append(&mut Bar::DivText(S("<br>")).into());
    IndentedStr::c("section", ans)
}

pub fn write_page(linzi: &str, article: Article) -> Result<(), Box<dyn std::error::Error>> {
    let Article { l, dat } = article;

    // if the title is empty, the h3 tag should not be listed in the table of contents
    let v1_entries: Vec<String> =
        l.v1.iter()
            .filter_map(|(k, _)| if k == "" { None } else { Some(k.to_owned()) })
            .collect();
    let v2_entries: Vec<String> =
        l.v2.iter()
            .filter_map(|(k, _)| if k == "" { None } else { Some(k.to_owned()) })
            .collect();

    let mut toc = vec![(S("燐字"), [&v1_entries[..], &v2_entries[..]].concat())];

    for LangEntry { lang, contents } in &dat {
        toc.push((lang.ja(), contents.iter().map(|a| a.0.clone()).collect()));
    }

    let mut toc_num = 0;

    let linzi_portion = l.render(&mut toc_num);

    let mut sections = vec![IndentedStr::c("section", linzi_portion)];
    for lang_entry in dat {
        sections.push(render_lang_entry(&lang_entry, &mut toc_num))
    }
    let cont = IndentedStr::c("article", sections);

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
    pub fn render_(self, toc_num: &mut usize) -> String {
        let section = IndentedStr::c("section", self.render(toc_num));
        section.to_string()
    }
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
        for a in init {
            ans.append(&mut a.into());
        }
        for (a, b) in v1 {
            *ind += 1;
            ans.push(IndentedStr::with_toc("h3", *ind, &a));
            ans.append(&mut b.into());
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
            ans.append(&mut b.into());
        }
        ans.append(&mut Bar::DivText(S("<br>")).into());
        ans
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub l: LinziPortion,
    pub dat: Vec<LangEntry>,
}

impl Lenticular for Article {
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>> {
        let Article { l, dat } = self.clone();
        let (l, dat) = (l, dat).lenticular_to_link()?;
        Ok(Article { l, dat })
    }
}

impl Lenticular for LangEntry {
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>> {
        let LangEntry { lang, contents } = self.clone();
        let (lang, contents) = (lang, contents).lenticular_to_link()?;
        Ok(LangEntry { lang, contents })
    }
}

impl Lenticular for Lang {
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>> {
        Ok(self.clone())
    }
}

impl Lenticular for Bar {
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>> {
        match self.clone() {
            Bar::DivText(s) => Ok(Bar::DivText(s.lenticular_to_link()?)),
            Bar::DivTexts(ss) => Ok(Bar::DivTexts(ss.lenticular_to_link()?)),
            Bar::List { ordered, content } => Ok(Bar::List {
                ordered,
                content: content.lenticular_to_link()?,
            }),
        }
    }
}

impl Lenticular for LinziPortion {
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>> {
        let LinziPortion {
            init,
            v1,
            grau_prua_yr,
            v2,
        } = self.clone();
        let ((init, v1), (grau_prua_yr, v2)) =
            ((init, v1), (grau_prua_yr, v2)).lenticular_to_link()?;
        Ok(LinziPortion {
            init,
            v1,
            grau_prua_yr,
            v2,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LangEntry {
    pub lang: Lang,
    pub contents: Vec<(String, Bar)>,
}
