#[macro_export]
macro_rules! heb_letter {
    ($($l:tt)*) => {
        $crate::letter::Letter::from_str(stringify!($($l)*))
    };
}

//#[macro_export]
//macro_rules! tl_he_letter {
//    ($($l:tt)*) => {
//        $crate::letter::Letter::from_str(stringify!($($l)*))
//    };
//}

#[macro_export]
macro_rules! heb_word {
    ($($l:tt)*) => {
	// stringify seems to introduce extra spaces between all characters
        $crate::word::Word::from_str(stringify!($($l)*))
    };
}

#[macro_export]
macro_rules! heb_phrase {
    ($($l:tt)*) => {
        $crate::phrase::Phrase::from_str(stringify!($($l)*))
    };
}

