pub mod end_word;
pub mod non_end_word;

use super::*;

use unicode_segmentation::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Word {
    pub original: String,
    letters: Vec<Letter>, 
}

impl Word {
    pub fn new(original: String, letters: Vec<Letter>) -> Self {
        Self { original, letters }
    }

    pub fn from_str<S: AsRef<str>>(stringlike: S) -> Self {
        let original = stringlike.as_ref().to_owned();
        assert!(!original.is_empty());

        let graphemes = stringlike.as_ref().graphemes(true);
        let letters: Vec<Letter> = graphemes
            .into_iter()
            .map(|x| {
                Letter::from_str(x)
            }).collect();

        Self::new(original, letters)
    }

    pub fn letters(&self) -> Vec<Letter> {
        self.letters.clone()
    }

    pub fn original(&self) -> String {
        let original = self.original.to_owned();
        // TODO: String returned is escaped unicode. Is there a way
        // to return the grapheme?
        original
    }

    pub fn dotless(&self) -> String {
        self.letters.iter().map(|x| x.base.to_string()).collect()
    }

    // The representation will place in a final character if it is 
    // available at the end of a word. It will also remove a final
    // letter that is not at the end of a word.
    pub fn dotless_proper_str(&self) -> String {
        self.dotless_proper()
            .letters
            .into_iter()
            .map(|x| {
                x.base.to_string()
            })
            .collect()
    }

    pub fn dotless_proper(&self) -> Self {
        let letters = &self.letters;
        if letters.len() == 0 { return self.clone() }
        let split_last = self.letters.split_last();

        if split_last.is_none() { 
            let msg = "Unsplitable string: ";
            panic!("{}{:#?}", msg, self);
        }

        let (last, elements) = split_last.unwrap();

        let endified = last.endify();

        let mut rest: Vec<Letter> = elements
            .into_iter()
            .map(|x| x.clone())
            .collect();

        rest.push(endified);
        Word::new(self.original.to_owned(), rest)
    }

    pub fn first_letter(&self) -> Letter {
        let first = self.letters.get(0).unwrap();
        first.clone()
    }
}

pub trait ToHebrewWord: Display {
    fn to_heb_word(&self) -> Word {
        let word_string = self.to_string();
        Word::from_str(word_string)
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Word")
            //.field("letters", &self.letters)
            //.field("string_final", &self.string_final())
            //.field("dotless_proper", &self.dotless_proper())
            .field("original", &self.original())
            .field("dotless", &self.dotless())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heb_word;

    #[test]
    fn test_dotless_proper() {
        let jonathan: Word = heb_word!(יונתנ);
        assert_eq!("יונתן", jonathan.dotless_proper_str());
    }

}

