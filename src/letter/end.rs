use super::*;

// ך"פצנמ   
// mantzapach

pub static FINAL_CHAR_ORDER: &'static [&'static char; 5] = {
    &[&'ך', &'ם', &'ן', &'ף', &'ץ']
};

pub static FINAL_POSITION_ORDER: &'static [&'static End; 5] = {
    use End::*;
    &[&ך, &ם, &ן, &ף, &ץ]
};

pub static END_MISPAR_KATAN_ORDER: &'static [&'static i32; 5] = {
    &[ &5, &6, &7, &8, &9 ]
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, StrumEnumString, StrumDisplay)]
pub enum End {
    ך, ם, ן, ף, ץ
}

impl End {
    pub fn primitive_letter(&self) -> super::NonEnd {
        use super::end::End::{ך, ם, ן, ף, ץ};
        use super::non_end::NonEnd::{כ, מ, נ, פ, צ};

        match self {
            ך => כ, 
            ם => מ, 
            ן => נ, 
            ף => פ, 
            ץ => צ, 
        }
    }

    pub fn chars() -> &'static [&'static char; 5] {
        FINAL_CHAR_ORDER
    }

    pub fn position_order() -> &'static [&'static Self; 5] {
        FINAL_POSITION_ORDER
    }

    pub fn index(&self) -> usize {
        Self::position_order().iter().position(|&r| r == self).unwrap()
    }

    pub fn char(&self) -> &'static char {
        let i = self.index();
        Self::chars()[i]
    }
}

