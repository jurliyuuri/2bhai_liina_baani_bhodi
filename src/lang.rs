use super::*;

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
