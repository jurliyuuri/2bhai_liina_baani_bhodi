use super::*;

pub fn linmarn() -> Vec<Foo> {
    vec![
        Foo::ls(r##"<h2><a name="TOC--"></a>燐字</h2>"##),
        Foo::c1("div", Foo::ls(r##"<hr>"##)),
        Foo::ls(r##"<div><img src="linzi/在.png"
          border="0"></div>"##),
        Foo::ls(r##"<div>総画：4</div>"##),
        Foo::ls(r##"<div>筆順：丶ノ一一</div>"##),
        Foo::ls(r##"<h3><a name="TOC--1"></a>字源</h3>"##),
        Foo::c1("ul",
            Foo::ls(r##"<li>象形指事。「<a href="処%20-%20燐字海.html">処</a>」を強調したもの。
          </li>"##)
        ),
        Foo::c("div", vec![
            Foo::C("div", S(r#" style="font-size:13.3333px">"#), vec![
                Foo::ls(r##"<h3><a name="TOC--2"></a>キャスカ・ファルザーの字源</h3>"##),
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
        Foo::ls(r##"<h3><a name="TOC--3"></a>意義</h3>"##),
        Foo::c1("div", Foo::c1("ol", Foo::ls(r##"<li>在る。</li>"##))),
        Foo::ls(r##"<div><br></div>"##),
    ]
}

pub fn proto() -> Vec<Foo> {
    vec![
        Foo::ls(
            r##"<h2><a name="TOC--4"></a><a
          href="https://sites.google.com/site/syxobo/raneme-zu-yu">ラネーメ祖語</a>
      </h2>"##,
        ),
        Foo::c1(
            "div",
            Foo::ls(
                r##"<h3><a name="TOC--5"></a>
          <hr>発音</h3>"##,
            ),
        ),
        Foo::ls(r##"<div>aimq</div>"##),
        Foo::ls(r##"<h3><a name="TOC--6"></a>名詞</h3>"##),
        Foo::ls(r##"<div>存在。</div>"##),
        Foo::ls(r##"<h3><a name="TOC--7"></a>述詞</h3>"##),
        Foo::ls(r##"<div>在る。～している。</div>"##),
    ]
}

pub fn air() -> Vec<Foo> {
    vec![
        Foo::ls(
            r##"<h2><a name="TOC--8"></a><a
          href="https://sites.google.com/site/riparaincangku/yuesureone-ren-gong-shi-jie-she-ding/pai-sheng-yu-fang-yan/lkurftlessd-air">アイル語</a>
      </h2>"##,
        ),
        Foo::c1("div", Foo::ls(r##"<hr>"##)),
        Foo::ls(r##"<h3><a name="TOC--9"></a>発音</h3>"##),
        Foo::ls(r##"<div>aima</div>"##),
        Foo::ls(r##"<h3><a name="TOC--10"></a>動詞</h3>"##),
        Foo::ls(r##"<div>在る。</div>"##),
    ]
}

pub fn pekzep() -> Vec<Foo> {
    vec![
        Foo::ls(
            r##"<h2><a name="TOC--11"></a><a
          href="https://sites.google.com/site/syxobo/paigu-yu">パイグ語</a></h2>"##,
        ),
        Foo::c(
            "div",
            vec![
                Foo::ls("<hr>"),
                Foo::ls(r#"<h3><a name="TOC--12"></a>発音</h3>"#),
            ],
        ),
        Foo::c1(
            "div",
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
        ),
        Foo::c1("div", Foo::ls(r##"<h3><a name="TOC--13"></a>名詞</h3>"##)),
        Foo::ls(r##"<div>存在。</div>"##),
        Foo::ls(r##"<h3><a name="TOC--14"></a>動詞</h3>"##),
        Foo::ls(r##"<div>在る。</div>"##),
        Foo::ls(r##"<h3><a name="TOC--15"></a>定詞</h3>"##),
        Foo::ls(r##"<div>～している。</div>"##),
        Foo::ls(r##"<h3><a name="TOC--16"></a>叫詞</h3>"##),
        Foo::ls(r##"<div>はい。</div>"##),
        Foo::ls(r##"<div><br></div>"##),
    ]
}

pub fn takang() -> Vec<Foo> {
    vec![
        Foo::ls(
            r##"<h2><a name="TOC--17"></a><a
          href="https://sites.google.com/site/syxobo/takan">タカン語</a></h2>"##,
        ),
        Foo::c1("div", Foo::ls(r##"<hr>"##)),
        Foo::ls(
            r##"<div style="font-size:13.3333px">
        <h3><a name="TOC--18"></a>発音</h3>
      </div>"##,
        ),
        Foo::ls(
            r##"<div>
        <ul>
          <li><span style="background-color:transparent">
              <font size="2">皇音：えま、え-む</font>
            </span></li>
          <li>
            <font size="2"><span
                style="background-color:transparent">牌音</span><span
                style="background-color:transparent">　古音：アイ　</span><span
                style="background-color:transparent">新音：エン</span></font>
          </li>
        </ul>
      </div>"##,
        ),
        Foo::ls(
            r##"<div style="font-size:13.3333px">
        <h3><a name="TOC--19"></a>名詞</h3>
      </div>"##,
        ),
        Foo::ls(
            r##"<div style="font-size:13.3333px">（えま<span
          style="font-size:small;background-color:transparent">）</span><span
          style="font-size:13.3333px;background-color:transparent">存在。</span>
      </div>"##,
        ),
        Foo::ls(
            r##"<h3><a name="TOC--20"></a>
        <font size="3">動詞</font>
      </h3>"##,
        ),
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
        Foo::ls(
            r##"<div style="font-size:13.3333px">
          <h2><a name="TOC--21"></a><a
              href="https://sites.google.com/site/riparaincangku/yuesureone-ren-gong-shi-jie-she-ding/pai-sheng-yu-fang-yan/lkurftlessd-air/etz">エッツィア語</a>
          </h2>
          <div>
            <hr>
          </div>
        </div>"##,
        ),
        Foo::c(
            "div",
            vec![
                Foo::ls(
                    r##"<div style="font-size:13.3333px">
            <h3><a name="TOC--22"></a>発音</h3>
          </div>"##,
                ),
                Foo::ls(
                    r##"<div>
            <ul>
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
            </ul>
          </div>"##,
                ),
                Foo::ls(
                    r##"<div style="font-size:13.3333px">
            <h3><a name="TOC--23"></a>名詞</h3>
          </div>"##,
                ),
                Foo::ls(r##"<div>存在、あること</div>"##),
            ],
        ),
        Foo::ls(
            r##"<div>
        <h3><a name="TOC--24"></a>動詞</h3>
      </div>"##,
        ),
        Foo::ls(r##"<div>（えま、アン）在る、存在する　（あいま）行う、実行する</div>"##),
    ]
}

pub fn bhat() -> Vec<Foo> {
    vec![
        Foo::C(
            "div",
            S(r##" style="font-size:13.3333px">"##),
            vec![
                Foo::ls(
                    r##"<h2><a name="TOC--25"></a><a
            href="http://jurliyuuri.github.io/bhaataan/grammar.html">バート語</a>
        </h2>"##,
                ),
                Foo::c("div", vec![Foo::ls("<hr>")]),
            ],
        ),
        Foo::c(
            "div",
            vec![
                Foo::ls(
                    r##"<h3><a name="TOC--26"></a>
        <font size="3">発音</font>
      </h3>"##,
                ),
                Foo::ls(r##"<div>hemúl, hem</div>"##),
            ],
        ),
        Foo::ls(r##"<h3><a name="TOC--27"></a>動詞</h3>"##),
        Foo::ls(r##"<div>(hemúl) ある。</div>"##),
        Foo::c(
            "div",
            vec![Foo::ls(r##"<h3><a name="TOC--28"></a>無変化動詞</h3>"##)],
        ),
        Foo::ls(r##"<div>(hem) 完了の無変化動詞。〜である。</div>"##),
        Foo::ls(r##"<div><br></div>"##),
    ]
}

pub fn lip_zep() -> Vec<Foo> {
    vec![
        Foo::C(
            "div",
            S(r##" style="font-size:13.3333px">"##),
            vec![
                Foo::ls(
                    r##"<h2><a name="TOC--29"></a><a
          href="https://sites.google.com/site/3tvalineparine/home">リパライン語</a></h2>"##,
                ),
                Foo::c("div", vec![Foo::ls("<hr>")]),
            ],
        ),
        Foo::ls(r##"<h3><a name="TOC--30"></a>発音</h3>"##),
        Foo::c(
            "div",
            vec![Foo::c(
                "ol",
                vec![
                    Foo::ls("<li>es e\'i</li>"),
                    Foo::ls("<li>teles</li>"),
                    Foo::ls("<li>mol</li>"),
                    Foo::ls("<li>molo</li>"),
                    Foo::ls("<li>molerl</li>"),
                ],
            )],
        ),
        Foo::ls(r##"<h3><a name="TOC--31"></a>名詞</h3>"##),
        Foo::ls("<div>在ること、存在</div>"),
        Foo::c(
            "div",
            vec![Foo::ls(r##"<h3><a name="TOC--32"></a>動詞</h3>"##)],
        ),
        Foo::ls(
            r##"行う、存在する（行うの文脈の場合、目的語があるならtelesで、無い場合はes e'iで訓読する。）"##,
        ),
        Foo::ls(r##"<h3><a name="TOC--33"></a>熟語</h3>"##),
        Foo::c1(
            "ol",
            Foo::ls(
                r##"<li><a href="真%20-%20燐字海.html">真</a>在　xinien
        la deliume　＜本分、本来の義務＞</li>"##,
            ),
        ),
    ]
}
