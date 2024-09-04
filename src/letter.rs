pub use super::*;

pub mod non_end;
use non_end::*;

pub mod end;
pub use end::*;

pub mod dot;
use dot::*;

pub mod base;
use base::Base;

pub mod base_constants;
pub mod letter_constants;
//pub mod tl_letter_constants;



#[derive(Debug, Clone, PartialEq, Eq, Hash, )]
pub struct Letter {
    pub base: Base, 
    pub dots: Vec<Dot>,
}

impl Letter {

    pub const fn new(base: Base) -> Self {
        Self::with_dots(base, Vec::new())
    }

    pub const fn with_dots(base: Base, dots: Vec<Dot>) -> Self {
        Self {
            base,
            dots,
        }
    }

    pub fn from_str<S: AsRef<str>>(stringlike: S) -> Self {
        let str_ref = stringlike.as_ref();
        let count = str_ref.chars().count();
        assert!(count >= 1);

        let chars = str_ref.chars().collect::<Vec<char>>();
        if let Some((base, rest)) = chars.split_first() {
            let base = Base::from_str(&base.to_string());

            // assert that all characters are hebrew dots
            let dots: Vec<Dot> = rest.iter().map(|x| {
                let dot = Dot::from_str(&x.to_string());
                //assert!(dot.is_ok());
                if let Ok(d) = dot {
                    return d; 
                } else {
                    panic!("Unknown Hebrew dots!: {:#?}", chars);
                }
            }).collect();

            return Self::with_dots(base, dots)
        }

        panic!("Unable to convert `{}`", str_ref);
    }

    // Turns a letter into it's final form if one is available.
    // Otherwise it will remain same non final version.
    pub fn endify(&self) -> Letter {
        let base = self.base.endify();
        let dot = self.dots.clone();
        Letter::with_dots(base, dot)
    }

    pub fn unendify(&self) -> Letter {
        let base = self.base.unendify();
        let dot = self.dots.clone();
        Letter::with_dots(base, dot)
    }

    pub fn base(&self) -> &Base {
        &self.base
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    //use crate::types::*;

    //#[test]
    //fn test_yo() {
    //    let non_final_variant = Letter::new(Base::כ);
    //    let final_variant = non_final_variant.final_variant().unwrap();
    //    let expected = Letter::new(Base::ך);
    //    assert_eq!(expected, final_variant);
    //}

    #[test]
    fn test_new_non_end() {
        Letter::new(base_constants::א);
    }

    #[test]
    fn test_new_end() {
        Letter::new(base_constants::ץ);
    }

    #[test]
    fn test_new_non_end_without_dots_from_str() {
        let stringlike = "כ"; 
        Letter::from_str(stringlike);
    }

    #[test]
    fn test_new_end_without_dots_from_str() {
        let stringlike = "ך"; 
        Letter::from_str(stringlike);
    }

    #[test]
    fn test_new_end_with_multi_dots_from_str() {
        let stringlike = "שָׁ"; 
        Letter::from_str(stringlike);
    }

    #[test]
    fn test_new_non_end_with_dots_from_str() {
        let stringlike = "בַ"; 
        Letter::from_str(stringlike);
    }

    // TODO: FIND A FINAL WITH DOTS
    //#[test]
    //fn test_new_end_with_dots_from_str() {
    //    let stringlike = ""; 
    //    Letter::from_str(stringlike);
    //}

}

