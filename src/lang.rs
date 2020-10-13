use super::*;

pub fn linmarn() -> Vec<Foo> {
    vec![
        Foo::ls(r##"<h2><a name="TOC--"></a>燐字</h2>"##),
        Foo::c1("div", Foo::ls("<hr>")),
        Foo::ls(r##"<div><img src="linzi/在.png"
          border="0"></div>"##),
        Foo::ls(r##"<div>総画：4</div>"##),
        Foo::ls(r##"<div>筆順：丶ノ一一</div>"##),
        h3(1, "字源"),
        Foo::c1("ul",
            Foo::ls(r##"<li>象形指事。「<a href="処%20-%20燐字海.html">処</a>」を強調したもの。
          </li>"##)
        ),
        Foo::c("div", vec![
            Foo::C("div", S(r#" style="font-size:13.3333px">"#), vec![
                h3(2, "キャスカ・ファルザーの字源"),
                Foo::C("div", S(r#" style="font-size:13.3333px">"#), vec![Foo::ls(r##"<ul></ul>"##)])
            ]),
            Foo::C("div", S(r##" style="font-size:13.3333px">"##), vec![
                Foo::c1("ul",
                    Foo::c1("li",
                        Foo::ls("呪術において使われる祭壇に乗せられる器を表す。器に供え物を置くという行為が、文化的な観点で強く「存在」を表したために、一般的な存在の意に転義した。")
                    )
                )
            ])
        ]),
        Foo::c1("div",
            Foo::c("div", vec![
                Foo::ls(r##"<div></div>"##),
                Foo::ls(r##"<div><img
                src="grau_prua_yr/在.png" width="200" height="91" border="0">
            </div>"##)
            ])
        ),
        Foo::ls(r##"<div></div>"##),
        h3(3, "意義"),
        Foo::c1("div", Foo::c1("ol", Foo::ls(r##"<li>在る。</li>"##))),
        Foo::ls(r##"<div><br></div>"##),
    ]
}

pub fn pekzep() -> Vec<Foo> {
    vec![
        Lang::Paige.h2(11),
        Foo::c1("div", Foo::ls("<hr>")),
        h3(12, "発音"),
        Foo::c(
            "ul",
            vec![
                Foo::ls(
                    r##"<li><span
              style="font-size:10pt;background-color:transparent">標準パイグ語：aim2</span>
          </li>"##,
                ),
                Foo::ls(
                    r##"<li><span
              style="font-size:10pt;background-color:transparent">アイツォ語：aim2</span>
          </li>"##,
                ),
                Foo::ls(
                    r##"<li><span
              style="font-size:10pt;background-color:transparent">古音：raim</span>
          </li>"##,
                ),
                Foo::ls(
                    r##"<li><span
              style="font-size:10pt;background-color:transparent">韻図音：冠在素</span>
          </li>"##,
                ),
            ],
        ),
        h3(13, "名詞"),
        Foo::ls(r##"<div>存在。</div>"##),
        h3(14, "動詞"),
        Foo::ls(r##"<div>在る。</div>"##),
        h3(15, "定詞"),
        Foo::ls(r##"<div>～している。</div>"##),
        h3(16, "叫詞"),
        Foo::ls(r##"<div>はい。</div>"##),
        Foo::ls(r##"<div><br></div>"##),
    ]
}

pub fn takang() -> Vec<Foo> {
    vec![
        Lang::Takang.h2(17),
        Foo::c1("div", Foo::ls("<hr>")),
        h3(18, "発音"),
        Foo::ls(
            r##"<ul>
          <li><span style="background-color:transparent">
              <font size="2">皇音：えま、え-む</font>
            </span></li>
          <li>
            <font size="2"><span
                style="background-color:transparent">牌音</span><span
                style="background-color:transparent">　古音：アイ　</span><span
                style="background-color:transparent">新音：エン</span></font>
          </li>
        </ul>"##,
        ),
        h3(19, "名詞"),
        Foo::ls(
            r##"<div style="font-size:13.3333px">（えま<span
          style="font-size:small;background-color:transparent">）</span><span
          style="font-size:13.3333px;background-color:transparent">存在。</span>
      </div>"##,
        ),
        h3(20, "動詞"),
        Foo::ls(
            r##"<div>
        <font size="2">（え-む）ある。</font><span
          style="font-size:13.3333px;background-color:transparent">～している。</span>
      </div>"##,
        ),
    ]
}

pub fn ezzia() -> Vec<Foo> {
    vec![
        Lang::Ezzia.h2(21),
        Foo::c1("div", Foo::ls("<hr>")),
        h3(22, "発音"),
        Foo::ls(
            r##"<ul>
              <li><span style="background-color:transparent">
                  <font size="2">光音：あいま</font>
                </span></li>
              <li><span style="background-color:transparent">
                  <font size="2">皇音：え、えむ</font>
                </span></li>
              <li>
                <font size="2"><span
                    style="background-color:transparent">牌音　</span><span
                    style="background-color:transparent">古音：ラン　</span><span
                    style="background-color:transparent">現音：アン</span></font>
              </li>
            </ul>"##,
        ),
        h3(23, "名詞"),
        Foo::ls(r##"<div>存在、あること</div>"##),
        h3(24, "動詞"),
        Foo::ls(r##"<div>（えま、アン）在る、存在する　（あいま）行う、実行する</div>"##),
    ]
}

pub enum Lang {
    Bhat,
    Lineparine,
    Takang,
    Ezzia,
    Paige,
    Air,
    Proto,
}

impl Lang {
    pub fn url(&self) -> String {
        match self {
            Lang::Bhat => S("http://jurliyuuri.github.io/bhaataan/grammar.html"),
            Lang::Lineparine => S("https://sites.google.com/site/3tvalineparine/home"),
            Lang::Takang => S("https://sites.google.com/site/syxobo/takan"),
            Lang::Ezzia => S("https://sites.google.com/site/riparaincangku/yuesureone-ren-gong-shi-jie-she-ding/pai-sheng-yu-fang-yan/lkurftlessd-air/etz"),
            Lang::Paige => S("https://sites.google.com/site/syxobo/paigu-yu"),
            Lang::Air => S("https://sites.google.com/site/riparaincangku/yuesureone-ren-gong-shi-jie-she-ding/pai-sheng-yu-fang-yan/lkurftlessd-air"),
            Lang::Proto => S("https://sites.google.com/site/syxobo/raneme-zu-yu")
        }
    }

    pub fn ja(&self) -> String {
        match self {
            Lang::Bhat => S("バート語"),
            Lang::Lineparine => S("リパライン語"),
            Lang::Takang => S("タカン語"),
            Lang::Ezzia => S("エッツィア語"),
            Lang::Paige => S("パイグ語"),
            Lang::Air => S("アイル語"),
            Lang::Proto => S("ラネーメ祖語"),
        }
    }

    pub fn h2(&self, toc_num: usize) -> Foo {
        Foo::L(format!(
            r##"<h2><a name="TOC--{}"></a><a href="{}">{}</a></h2>"##,
            toc_num,
            &self.url(),
            &self.ja()
        ))
    }
}
