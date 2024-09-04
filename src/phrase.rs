use super::*;


#[derive(Clone, PartialEq, Eq)]
pub struct Phrase {
    words: Vec<Word>
    // TODO: Add vec for delimeters to reconstruct properly
}

impl Phrase {

    pub fn new() -> Self {
        Self {
            words: Vec::new()
        }
    }

    pub fn with_words(words: Vec<Word>) -> Self {
        Self {
            words
        }
    }

    pub fn from_str<S: AsRef<str>>(stringlike: S) -> Self {
        let original = stringlike.as_ref().trim();
        if original.is_empty() {
            return Phrase::new()
        }

        // TODO:
        // Need to handle conditions where multiple dividers present!
        //original.match_indices(" ")
        Self::with_words(split_phrase_str_into_words(original))
    }

    pub fn original(&self) -> String {
        todo!("IMPL ORIGINAL IN PHRASE")
    }

    pub fn words(&self) -> Vec<Word> {
        self.words.clone()
    }

    pub fn dotless(&self) -> String {
        let strs: Vec<String> = self
            .words
            .iter()
            .map(|w| w.dotless()).collect();

        // TODO: Probably want to preserve whatever is passed in 
        // instead of just joining it back with a space, use
        // the original section delimeters
        strs.join(" ")
    }

}

pub fn split_phrase_str_into_words<S: AsRef<str>>(
    stringlike: S,
) -> Vec<Word> {
    //let delimiters: &[&str] = &[" ", "-"];
    let delimiters: &[&str] = &[" "];
    let matches = match_indices_multi(stringlike, delimiters);
    let things = matches
        .iter()
        .map(|(_,v)| {
        Word::from_str(v)
    })
    .collect();

    // TODO: Save delimeters as they all can be different
    // maybe save in separate vec and paralell reconstruct

    things
}

pub fn match_indices_multi<S: AsRef<str>>(
    stringlike: S,
    pats: &[&str]
) -> Vec<(usize, String)> {
    // TODO: Use of a regex would probably be cleaner here
    let mut ms: Vec<_> = pats
        .iter()
        .map(|p| { 
            stringlike
                .as_ref()
                .match_indices(p)
                .collect::<Vec<(usize,&str)>>()
        })
        .flatten()
        .map(|(i,c)| { (i,c.to_owned()) })
        .collect();

    if ms.len() == 0 { // Case where 1 or less words in passed string
        return vec![(0, stringlike.as_ref().to_owned())]
    }
    // TODO: Might need to do something more complicated 
    // here instead of depending on default
    ms.sort();

    ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thing() {

        let original = "test-here too-much:after";
        //let matches = match_indices_multi(original, &[" ", "-"]);
        let _ = match_indices_multi(original, &[" "]);

        //panic!("{:#?}", matches);

    }
}

