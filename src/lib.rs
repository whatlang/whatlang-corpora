use whatlang::Lang;
use std::path::{Path, PathBuf};

pub struct Corpus {
    lang: Lang,
    text: String
}

impl Corpus {
    pub fn load(lang: Lang) -> Self {
        let file_path: PathBuf = Path::new(file!())
            .canonicalize().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .join("corpora")
            .join(format!("{}.txt", lang.code()));

        let text = std::fs::read_to_string(file_path).unwrap();

        Self::new(lang, text)
    }

    fn new(lang: Lang, text: String) -> Self {
        Self { lang, text }
    }

    pub fn sentences(&self) ->  impl Iterator<Item=&str> {
        self.text.split('\n')
    }

    pub fn lang(&self) -> Lang {
        self.lang
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corpus() {
        let corpus = Corpus::load(Lang::Ukr);
        assert_eq!(corpus.sentences().count(), 100_000);
    }

    // #[test]
    // fn test_print_all() {
    //     for lang in Lang::values()  {
    //         let code = lang.code();
    //         println!("Lang::{:?} => include_str!(\"../corpora/{}.txt\"),", lang, lang.code());
    //     }
    // }
}
