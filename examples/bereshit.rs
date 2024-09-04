//use hebrew::macros::*;
//use hebrew::heb_phrase;
//use hebrew::heb_word;
//use hebrew::word::Word;
use hebrew::phrase::Phrase;

fn main() {
    // TODO: this doesn't currently work right
    let bereshit_str = "בְּרֵאשִׁית, בָּרָא אֱלֹהִים, אֵת הַשָּׁמַיִם, וְאֵת הָאָרֶץ.";
    let bereshit = Phrase::from_str(bereshit_str);
    println!("{:#?}", bereshit.dotless());
}

