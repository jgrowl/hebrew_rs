use super::letter_constants::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StrumEnumString, StrumDisplay)]
pub enum NonEndWord {
    אָלֶף,
    בֵּית,    // ALT: תיבֵ
    גִימֵל,   // ALT : למֵיגִּ
    דָלֶת,    // ALT:  תלֶדָּ
    הֵא,
    וָו,
    זַיִן,
    חֵית,
    טֵית,
    יוֹד,
    כַף,
    לָמֶד,
    מֵם,
    נוּן,
    סָמֶךְ,
    עַיִן,
    פֵה,
    צָדִי,
    קוֹף,
    רֵישׁ,
    שִׂין,    // ALT:  ןישִׁ
    תָו      // ALT: ותָּ
}

impl NonEndWord {}

use crate::*;

pub const LETTER_LETTER_WORDS: &'static LetterLetterWordPair = { 
    use NonEndWord::*;
    &[
        (א, אָלֶף), 
        (ב, בֵּית),
        (ג, גִימֵל),
        (ד, דָלֶת),
        (ה, הֵא),
        (ו, וָו),
        (ז, זַיִן),
        (ח, חֵית),
        (ט, טֵית),
        (י, יוֹד),
        (כ, כַף),
        (ל, לָמֶד),
        (מ, מֵם),
        (נ, נוּן),
        (ס, סָמֶךְ),
        (ע, עַיִן),
        (פ, פֵה),
        (צ, צָדִי),
        (ק, קוֹף),
        (ר, רֵישׁ),
        (ש, שִׂין),
        (ת, תָו)
    ]
};

