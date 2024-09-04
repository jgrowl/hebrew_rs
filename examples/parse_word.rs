use hebrew::heb_word;
use hebrew::word::Word;


fn main() {
    println!("We can use a macro to create a word");
    let jonathan: Word = heb_word!(יונתנ);
    println!("{}", jonathan.dotless());

    println!("We can ensure final forms are used on words:");
    println!("{} becomes: {}", jonathan.dotless(), jonathan.dotless_proper_str());

    let bereshit_str = "בְּרֵאשִׁית";
    let bereshit = Word::from_str(bereshit_str);
    println!("We can get dotless version from a string that includes dots");
    println!("{} becomes: {}", bereshit_str, bereshit.dotless());
}

