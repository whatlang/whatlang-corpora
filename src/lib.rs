use whatlang::Lang;

const UKR: &'static str = include_str!("../corpora/ukr.txt");
const RUS: &'static str = include_str!("../corpora/rus.txt");
const ENG: &'static str = include_str!("../corpora/eng.txt");
const CMN: &'static str = include_str!("../corpora/cmn.txt");
const EPO: &'static str = include_str!("../corpora/epo.txt");
const SPA: &'static str = include_str!("../corpora/spa.txt");

pub struct Corpus {
    lang: Lang,
    text: &'static str
}

impl Corpus {
    pub fn find(lang: Lang) -> Self {
        let text = match lang {
            Lang::Ukr => UKR,
            Lang::Eng => ENG,
            Lang::Rus => RUS,
            Lang::Cmn => CMN,
            Lang::Epo => EPO,
            Lang::Spa => SPA,
            _ => panic!(format!("No corpuse for {}", lang))
        };
        Self::new(lang, text)
    }

    fn new(lang: Lang, text: &'static str) -> Self {
        Self { lang, text }
    }

    pub fn sentences(&self) ->  impl Iterator<Item=&'static str> {
        self.text.split('\n')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corpus() {
        let corpus = Corpus::find(Lang::Ukr);

        assert_eq!(corpus.sentences().count(), 100_000);

        for sentence in corpus.sentences().take(3) {
            println!("{}", sentence);
        }
    }
}
