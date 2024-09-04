//use std::str::FromStr;

use strum_macros::EnumString;
use strum_macros::Display;


// See below for code list example:
// https://www.compart.com/en/unicode/U+05b2
// TODO: Double check first serialize= (non unicode)
// for accuracy

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Display)]
pub enum Dot {

    #[strum(serialize = "\u{590}")]
    Undefined,

    #[strum(serialize = "\u{591}")]
    AccentEtnahta,
    #[strum(serialize = "\u{592}")]
    AccentSegol,
    #[strum(serialize = "\u{593}")]
    AccentShalshelet,
    #[strum(serialize = "\u{594}")]
    AccentZaqefQatan,
    #[strum(serialize = "\u{595}")]
    AccentZaqefGadol,
    #[strum(serialize = "\u{596}")]
    AccentTipeha,
    #[strum(serialize = "\u{597}")]
    AccentRevia,
    #[strum(serialize = "\u{598}")]
    AccentZarqa,
    #[strum(serialize = "\u{599}")]
    AccentPashta,


    #[strum(serialize = "\u{5a0}")]
    AccentTelishaGedola,
    #[strum(serialize = "\u{5a1}")]
    AccentPazer,
    #[strum(serialize = "\u{5a2}")]
    AccentAtnahHafukh,
    #[strum(serialize = "\u{5a3}")]
    AccentMunah,
    #[strum(serialize = "\u{5a4}")]
    AccentMahapakh,
    #[strum(serialize = "\u{5a5}")]
    AccentMerkha,
    #[strum(serialize = "\u{5a6}")]
    AccentMerkhaKefula,
    #[strum(serialize = "\u{5a7}")]
    AccentDarga,
    #[strum(serialize = "\u{5a8}")]
    AccentQadma,
    #[strum(serialize = "\u{5a9}")]
    AccentTelishaQetana,
    #[strum(serialize = "\u{5aa}")]
    AccentYerahBenYomo,
    #[strum(serialize = "\u{5ab}")]
    AccentOle,
    #[strum(serialize = "\u{5ac}")]
    AccentIluy,
    #[strum(serialize = "\u{5ad}")]
    AccentDehi,
    #[strum(serialize = "\u{5ae}")]
    AccentZinor,
    #[strum(serialize = "\u{5af}")]
    MarkMasoraCircle,
    #[strum(serialize = " ְ", serialize = "\u{5b0}")]
    PointSheva,
    #[strum(serialize = " ֱ", serialize = "\u{5b1}")]
    PointHatafSegol,
    #[strum(serialize = " ֲ", serialize = "\u{5b2}") ]
    PointHatefPatah,
    #[strum(serialize = " ֳ", serialize = "\u{5b3}")]
    PointHatafQamats,
    #[strum(serialize = " ִ", serialize = "\u{5b4}")]
    PointHiriq,
    #[strum(serialize = " ֵ", serialize = "\u{5b5}")]
    PointTsere,
    #[strum(serialize = " ֶ", serialize = "\u{5b6}")]
    PointSegol,
    #[strum(serialize = " ַ", serialize = "\u{5b7}")]
    PointPatah,
    #[strum(serialize = " ָ", serialize = "\u{5b8}")]
    PointQamats,
    #[strum(serialize = " ֹ", serialize = "\u{5b9}")]
    PointHolam,
    #[strum(serialize = " ֺ", serialize = "\u{5ba}")]
    PointHolamHaserForVav,
    #[strum(serialize = " ֻ", serialize = "\u{5bb}")]
    PointQubuts,
    #[strum(serialize = " ּ", serialize = "\u{5bc}")]
    PointDagesh,  // Or called PointMapiq
    #[strum(serialize = " ֽ", serialize = "\u{5bd}")]
    PointMeteg,
    //#[strum(serialize = "־", serialize = "\u{5be}")]
    #[strum(serialize = "\u{5be}")]
    PunctuationMaqaf,
    #[strum(serialize = " ֿ", serialize = "\u{5bf}")]
    PointRafe,
    //#[strum(serialize = "׀", serialize = "\u{5c0}")]
    #[strum(serialize = "\u{5c0}")]
    PunctuationPaseq,
    #[strum(serialize = " ׁ", serialize = "\u{5c1}" )]
    PointShinDot,
    #[strum(serialize = " ׂ", serialize = "\u{5c2}" )]
    PointSinDot,
    #[strum(serialize = "\u{5c3}" )]
    PunctuationSofPaseq,

    //#[strum(serialize = " ׄ")]
    //MARK_UPPER_DOT,
    #[strum(serialize = "\u{5c4}" )]
    MarkUpperDot,

    //#[strum(serialize = " ׅ")]
    //MARK_LOWER_DOT,
    #[strum(serialize = "\u{5c5}" )]
    MarkLowerDot,

    #[strum(serialize = "\u{5c6}" )]
    PunctuationNunHafukha,
    #[strum(serialize = " ׇ", serialize = "\u{5c7}")]
    PointQamatsQatan,

    #[strum(serialize = "\u{5c8}")]
    UndefinedCharacter05c8,

    #[strum(serialize = "\u{5c9}")]
    UndefinedCharacter05c9,

    #[strum(serialize = "\u{5ca}")]
    UndefinedCharacter05ca,

    #[strum(serialize = "\u{5cb}")]
    UndefinedCharacter05cb,

    #[strum(serialize = "\u{5cc}")]
    UndefinedCharacter05cc,

    #[strum(serialize = "\u{5cf}")]
    UndefinedCharacter05cf,


    // TODO: Special Hebrew characters 05eb..=05ff


    // TODO:
    // I do not think these are diacriticals. 
    // Find out what characters and handle them above this section
    #[strum(serialize = " ")]
    SPACE,

    #[strum(serialize = "\u{034}")]
    JOINER,

    #[strum(serialize = "\u{34f}")]
    JOINER2,
     
}

impl Dot {}
