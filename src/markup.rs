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

pub trait Lenticular
where
    Self: std::marker::Sized,
{
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>>;
}

#[derive(Debug, Clone)]
pub enum LenticularError {
    MismatchedRightLenticular(String),
    MismatchedLeftLenticular(String),
    LenticularInsideLenticular(String),
}

impl<T, U> Lenticular for (T, U)
where
    T: Lenticular + Clone,
    U: Lenticular + Clone,
{
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>> {
        let (a, b) = self;
        match (a.lenticular_to_link(), b.lenticular_to_link()) {
            (Ok(a), Ok(b)) => Ok((a, b)),
            (Ok(_), Err(b)) => Err(b),
            (Err(b), Ok(_)) => Err(b),
            (Err(mut a), Err(mut b)) => {
                a.append(&mut b);
                Err(a)
            }
        }
    }
}

impl<T> Lenticular for Vec<T>
where
    T: Lenticular + Clone,
{
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>> {
        let mut ans = Vec::new();
        let mut errors = Vec::new();
        for i in self.iter() {
            match i.clone().lenticular_to_link() {
                Ok(l) => ans.push(l),
                Err(mut e) => errors.append(&mut e),
            }
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(ans)
        }
    }
}

impl Lenticular for String {
    /// Convert lenticular bracket into a link
    fn lenticular_to_link(&self) -> Result<Self, Vec<LenticularError>> {
        let mut inside_lenticular = false;
        let mut ans = String::new();
        for c in self.chars() {
            if inside_lenticular {
                match c {
                    '【' => {
                        return Err(vec![LenticularError::LenticularInsideLenticular(
                            self.clone(),
                        )])
                    }
                    '】' => {
                        inside_lenticular = false;
                    }
                    linzi => {
                        ans += &format!(
                            "<a href=\"{linzi}%20-%20燐字海.html\">{linzi}</a>",
                            linzi = linzi
                        );
                    }
                }
            } else {
                match c {
                    '【' => {
                        inside_lenticular = true;
                    }
                    '】' => {
                        return Err(vec![LenticularError::MismatchedRightLenticular(
                            self.clone(),
                        )])
                    }
                    linzi => ans.push(linzi),
                }
            }
        }

        if inside_lenticular {
            return Err(vec![LenticularError::MismatchedLeftLenticular(
                self.clone(),
            )]);
        }

        Ok(ans)
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
