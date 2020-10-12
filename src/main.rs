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

const TOC: &str = r##"<ol class="goog-toc">
  <li class="goog-toc"><a href="#TOC--"><strong>1 </strong>燐字</a>
    <ol class="goog-toc">
      <li class="goog-toc"><a href="#TOC--1"><strong>1.1
          </strong>字源</a></li>
      <li class="goog-toc"><a href="#TOC--2"><strong>1.2
          </strong>キャスカ・ファルザーの字源</a></li>
      <li class="goog-toc"><a href="#TOC--3"><strong>1.3
          </strong>意義</a></li>
    </ol>
  </li>
  <li class="goog-toc"><a href="#TOC--4"><strong>2 </strong>ラネーメ祖語</a>
    <ol class="goog-toc">
      <li class="goog-toc"><a href="#TOC--5"><strong>2.1
          </strong>発音</a></li>
      <li class="goog-toc"><a href="#TOC--6"><strong>2.2
          </strong>名詞</a></li>
      <li class="goog-toc"><a href="#TOC--7"><strong>2.3
          </strong>述詞</a></li>
    </ol>
  </li>
  <li class="goog-toc"><a href="#TOC--8"><strong>3 </strong>アイル語</a>
    <ol class="goog-toc">
      <li class="goog-toc"><a href="#TOC--9"><strong>3.1
          </strong>発音</a></li>
      <li class="goog-toc"><a href="#TOC--10"><strong>3.2
          </strong>動詞</a></li>
    </ol>
  </li>
  <li class="goog-toc"><a href="#TOC--11"><strong>4 </strong>パイグ語</a>
    <ol class="goog-toc">
      <li class="goog-toc"><a href="#TOC--12"><strong>4.1
          </strong>発音</a></li>
      <li class="goog-toc"><a href="#TOC--13"><strong>4.2
          </strong>名詞</a></li>
      <li class="goog-toc"><a href="#TOC--14"><strong>4.3
          </strong>動詞</a></li>
      <li class="goog-toc"><a href="#TOC--15"><strong>4.4
          </strong>定詞</a></li>
      <li class="goog-toc"><a href="#TOC--16"><strong>4.5
          </strong>叫詞</a></li>
    </ol>
  </li>
  <li class="goog-toc"><a href="#TOC--17"><strong>5 </strong>タカン語</a>
    <ol class="goog-toc">
      <li class="goog-toc"><a href="#TOC--18"><strong>5.1
          </strong>発音</a></li>
      <li class="goog-toc"><a href="#TOC--19"><strong>5.2
          </strong>名詞</a></li>
      <li class="goog-toc"><a href="#TOC--20"><strong>5.3
          </strong>動詞</a></li>
    </ol>
  </li>
  <li class="goog-toc"><a href="#TOC--21"><strong>6
      </strong>エッツィア語</a>
    <ol class="goog-toc">
      <li class="goog-toc"><a href="#TOC--22"><strong>6.1
          </strong>発音</a></li>
      <li class="goog-toc"><a href="#TOC--23"><strong>6.2
          </strong>名詞</a></li>
      <li class="goog-toc"><a href="#TOC--24"><strong>6.3
          </strong>動詞</a></li>
    </ol>
  </li>
  <li class="goog-toc"><a href="#TOC--25"><strong>7 </strong>バート語</a>
    <ol class="goog-toc">
      <li class="goog-toc"><a href="#TOC--26"><strong>7.1
          </strong>発音</a></li>
      <li class="goog-toc"><a href="#TOC--27"><strong>7.2
          </strong>動詞</a></li>
      <li class="goog-toc"><a href="#TOC--28"><strong>7.3
          </strong>無変化動詞</a></li>
    </ol>
  </li>
  <li class="goog-toc"><a href="#TOC--29"><strong>8
      </strong>リパライン語</a>
    <ol class="goog-toc">
      <li class="goog-toc"><a href="#TOC--30"><strong>8.1
          </strong>発音</a></li>
      <li class="goog-toc"><a href="#TOC--31"><strong>8.2
          </strong>名詞</a></li>
      <li class="goog-toc"><a href="#TOC--32"><strong>8.3
          </strong>動詞</a></li>
      <li class="goog-toc"><a href="#TOC--33"><strong>8.4
          </strong>熟語</a></li>
    </ol>
  </li>
</ol>"##;

const CONTENT: &str = r##"<div>
  <div style="display:block;text-align:left">
    <div style="display:block;text-align:left">
      <div style="display:block;text-align:left">
        <div style="display:block;text-align:left">
          <div style="display:block;text-align:left">
            <div style="display:block;text-align:left">
              <div style="display:block;text-align:left"><img src="linzi/在.png"
                  border="0"></div>
            </div>
          </div>
          <div style="display:block;text-align:left">総画：4</div>
          <div style="display:block;text-align:left">筆順：丶ノ一一</div>
          <h3 style="display:block;text-align:left"><a name="TOC--1"></a>字源</h3>
          <div>
            <ul>
              <li>象形指事。「<a href="処%20-%20燐字海.html">処</a>」を強調したもの。
              </li>
            </ul>
            <div>
              <div style="font-size:13.3333px">
                <h3><a name="TOC--2"></a>キャスカ・ファルザーの字源</h3>
                <div style="font-size:13.3333px">
                  <ul></ul>
                </div>
              </div>
              <div style="font-size:13.3333px">
                <ul>
                  <li>
                    呪術において使われる祭壇に乗せられる器を表す。器に供え物を置くという行為が、文化的な観点で強く「存在」を表したために、一般的な存在の意に転義した。
                  </li>
                </ul>
              </div>
            </div>
            <div>
              <div style="display:block;text-align:left">
                <div style="display:block;text-align:left"></div>
                <div style="display:block;text-align:left"><img
                    src="grau_prua_yr/在.png" width="200" height="91" border="0">
                </div>
              </div>
            </div>
          </div>
          <div></div>
          <h3><a name="TOC--3"></a>意義</h3>
          <div>
            <ol>
              <li>在る。</li>
            </ol>
          </div>
          <div><br></div>
          <h2><a name="TOC--4"></a><a
              href="https://sites.google.com/site/syxobo/raneme-zu-yu">ラネーメ祖語</a>
          </h2>
          <div>
            <h3><a name="TOC--5"></a>
              <hr>発音</h3>
          </div>
          <div>aimq</div>
          <h3><a name="TOC--6"></a>名詞</h3>
          <div>存在。</div>
          <h3><a name="TOC--7"></a>述詞</h3>
          <div>在る。～している。</div>
          <h2><a name="TOC--8"></a><a
              href="https://sites.google.com/site/riparaincangku/yuesureone-ren-gong-shi-jie-she-ding/pai-sheng-yu-fang-yan/lkurftlessd-air">アイル語</a>
          </h2>
          <div>
            <hr>
          </div>
        </div>
        <h3 style="display:block;text-align:left"><a name="TOC--9"></a>発音</h3>
        <div>aima</div>
        <h3><a name="TOC--10"></a>動詞</h3>
        <div>在る。</div>
        <h2><a name="TOC--11"></a><a
            href="https://sites.google.com/site/syxobo/paigu-yu">パイグ語</a></h2>
        <div>
          <hr>
          <h3><a name="TOC--12"></a>発音</h3>
        </div>
        <div>
          <ul>
            <li><span
                style="font-size:10pt;background-color:transparent">標準パイグ語：aim2</span>
            </li>
            <li><span
                style="font-size:10pt;background-color:transparent">アイツォ語：aim2</span>
            </li>
            <li><span
                style="font-size:10pt;background-color:transparent">古音：raim</span>
            </li>
            <li><span
                style="font-size:10pt;background-color:transparent">韻図音：冠在素</span>
            </li>
          </ul>
        </div>
        <div>
          <h3><a name="TOC--13"></a>名詞</h3>
        </div>
        <div>存在。</div>
        <h3><a name="TOC--14"></a>動詞</h3>
        <div>在る。</div>
        <h3><a name="TOC--15"></a>定詞</h3>
        <div>～している。</div>
        <h3><a name="TOC--16"></a>叫詞</h3>
        <div>はい。</div>
        <div><br></div>
        <h2><a name="TOC--17"></a><a
            href="https://sites.google.com/site/syxobo/takan">タカン語</a></h2>
        <div>
          <hr>
        </div>
      </div>
      <div style="display:block;text-align:left">
        <div style="font-size:13.3333px">
          <h3><a name="TOC--18"></a>発音</h3>
        </div>
        <div>
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
        </div>
        <div style="font-size:13.3333px">
          <h3><a name="TOC--19"></a>名詞</h3>
        </div>
        <div style="font-size:13.3333px">（えま<span
            style="font-size:small;background-color:transparent">）</span><span
            style="font-size:13.3333px;background-color:transparent">存在。</span>
        </div>
        <h3><a name="TOC--20"></a>
          <font size="3">動詞</font>
        </h3>
        <div>
          <font size="2">（え-む）ある。</font><span
            style="font-size:13.3333px;background-color:transparent">～している。</span>
        </div>
        <div>
          <div style="font-size:13.3333px">
            <h2><a name="TOC--21"></a><a
                href="https://sites.google.com/site/riparaincangku/yuesureone-ren-gong-shi-jie-she-ding/pai-sheng-yu-fang-yan/lkurftlessd-air/etz">エッツィア語</a>
            </h2>
            <div>
              <hr>
            </div>
          </div>
          <div>
            <div style="font-size:13.3333px">
              <h3><a name="TOC--22"></a>発音</h3>
            </div>
            <div>
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
            </div>
            <div style="font-size:13.3333px">
              <h3><a name="TOC--23"></a>名詞</h3>
            </div>
            <div>存在、あること</div>
          </div>
        </div>
        <div>
          <h3><a name="TOC--24"></a>動詞</h3>
        </div>
        <div>（えま、アン）在る、存在する　（あいま）行う、実行する</div>
        <div style="font-size:13.3333px">
          <h2><a name="TOC--25"></a><a
              href="http://jurliyuuri.github.io/bhaataan/grammar.html">バート語</a>
          </h2>
          <div>
            <hr>
          </div>
        </div>
      </div>
      <div>
        <h3><a name="TOC--26"></a>
          <font size="3">発音</font>
        </h3>
        <div>hemúl, hem</div>
      </div>
      <h3><a name="TOC--27"></a>動詞</h3>
      <div>(hemúl) ある。</div>
      <div>
        <h3><a name="TOC--28"></a>無変化動詞</h3>
      </div>
      <div>(hem) 完了の無変化動詞。〜である。</div>
      <div><br></div>
    </div>
    <div style="font-size:13.3333px">
      <h2><a name="TOC--29"></a><a
          href="https://sites.google.com/site/3tvalineparine/home">リパライン語</a></h2>
      <div>
        <hr>
      </div>
    </div>
    <h3><a name="TOC--30"></a>発音</h3>
    <div>
      <ol>
        <li>es e'i</li>
        <li>teles</li>
        <li>mol</li>
        <li>molo</li>
        <li>molerl</li>
      </ol>
    </div>
    <h3><a name="TOC--31"></a>名詞</h3>
    <div>在ること、存在</div>
    <div>
      <h3><a name="TOC--32"></a>動詞</h3>
    </div>
  </div>行う、存在する（行うの文脈の場合、目的語があるならtelesで、無い場合はes e'iで訓読する。）
</div>
<div>
  <div>
    <h3><a name="TOC--33"></a>熟語</h3>
  </div>
  <ol>
    <li><a href="真%20-%20燐字海.html">真</a>在　xinien
      la deliume　＜本分、本来の義務＞</li>
  </ol>
</div><br>"##;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(format!("docs/{} - 燐字海.html", "在"))?;
    write!(
        file,
        "{}",
        LinzklarTemplate { linzi: "在", toc: TOC, content: CONTENT }.render().unwrap()
    )?;

    Ok(())
}
