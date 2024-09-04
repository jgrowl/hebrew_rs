use super::*;
use NonEnd::*;

#[derive(Clone, PartialEq, Eq, Hash, StrumEnumString, StrumDisplay)]
pub enum NonEnd {
    א, ב, ג, ד, ה, ו, ז, ח, ט, י, כ, ל, מ, נ, ס, ע, פ, צ, ק, ר, ש, ת
}

pub static POSITION_ORDER: &'static [&'static NonEnd; 22] = {
    &[ &א, &ב, &ג, &ד, &ה, &ו, &ז, &ח, &ט, &י, 
        &כ, &ל, &מ, &נ, &ס, &ע, &פ, &צ, 
        &ק, &ר, &ש, &ת ]
};

impl NonEnd {

    pub fn index(&self) -> usize {
        Self::position_order().iter().position(|&r| r == self).unwrap()
    }

    pub fn position(&self) -> usize {
        self.index() + 1
    }

    pub fn position_order() -> &'static [&'static Self; 22] {
        POSITION_ORDER
    }

    pub fn end_letter(&self) -> Option<super::end::End>{
        use super::end::End::{ך, ם, ן, ף, ץ};
        use NonEnd::{כ, מ, נ, פ, צ};

        match self {
            כ => Some(ך), 
            מ => Some(ם), 
            נ => Some(ן), 
            פ => Some(ף), 
            צ => Some(ץ), 
            _ => None
        }
    }
}

impl fmt::Debug for NonEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NonEnd")
            .field("char", &self)
            .field("position", &self.position())
            .finish()
    }
}

