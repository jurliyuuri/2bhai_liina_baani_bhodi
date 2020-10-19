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

pub fn write_page(linzi: &str, l: LinziPortion, h: Hoge) -> Result<(), Box<dyn std::error::Error>> {
    let (toc, cont) = hoge(l, h);
    write_page_raw(linzi, toc, cont.to_string())
}

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

pub fn generate_toc<S>(toc: Vec<(S, Vec<&str>)>) -> String
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

pub struct LinziPortion {
    pub init: Vec<Bar>,
    pub v1: Vec<(&'static str, Bar)>,
    pub grau_prua_yr: &'static str,
    pub v2: Vec<(&'static str, Bar)>,
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
pub struct Hoge(pub Vec<LangHoge>);

pub struct LangHoge {
    pub lang: Lang,
    pub contents: Vec<(&'static str, Bar)>,
}

fn hoge(l: LinziPortion, dat: Hoge) -> (String, Foo) {
    let v1_entries: Vec<&'static str> = l.v1.iter().map(|(k, _)| *k).collect();
    let v2_entries: Vec<&'static str> = l.v2.iter().map(|(k, _)| *k).collect();

    let mut toc = vec![(S("燐字"), [&v1_entries[..], &v2_entries[..]].concat())];

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
