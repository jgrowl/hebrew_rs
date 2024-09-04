# hebrew_rs

Hebrew alephbet primatives and parsing library for rust.

The aim for this library is to fully support all unicode Hebrew characters including final forms,
nikkud dots, letters, words, and phrases (sentences).

## Progress

- [x] Support basic letter parsing 
- [x] Support final letter parsing 
- [x] Support converting letters into their final versions and vice versa
- [x] Support letters with most common nikkud 
- [x] Provide ability to remove all nikkud from a letter or word
        Note that while dots can be removed, they cannot be added to words that did not
        have them to begin with. There is no logic to determine the proper nikkud.
        Dots in the original parsed string can be preserved however.

- [ ] Support all unicode Hebrew characters (See dots.rs for what is an isn't supported)
- [x] Support word parsing 
- [ ] Support phrase (and sentence) parsing
        A naive method of just tokenizing on space characters can split phrases up into
        a vector of words, which in turn are a vec of letter structs that include a base
        and dots. A more fully featured method will hopefully be implemented in the future.
- [ ] Preserve all split characters in phrases
- [x] Provide full word representations of each letter


## Right to left (RTL) VS Left to Right (LTR)

Hebrew is written from right to left (RTL) in contrast to a language like English that is
written left to right (LTR). Many programs detect RTL languages and will display them in
LTR format on machines that have set their local language as a LTR system. This means the
actual underlying text is actually stored RTL, but will look to an end user of a LTR system as 
written as LTR. It would be difficult to write hebrew without this because you would have
to type a single character and then move the cursor to the left for every character you would
type.

This needs to be kept in mind because any parsed data will be expected to be stored in RTL as
is normal for Hebrew, even though it will be displayed LTR on LTR systems.


## Letters

Hebrew has 22 total letters. 5 of them have a special final or 'sophit' form which are used
sometimes when it occurs at the end of a word:

### Non end forms

א, ב, ג, ד, ה, ו, ז, ח, ט, י, כ, ל, מ, נ, ס, ע, פ, צ, ק, ר, ש, ת

Note that ה (Heh) looks a lot like ח (Chet) and ת (Tav)


### End forms

ך, ם, ן, ף, ץ


#### Non end forms to end forms conversion

כ ← ך

מ ← ם

נ ← ן

פ ← ף

צ ← ץ

Note that some final versions look like a different character but are not:

 Kaph (כ) looks a lot like Resh (ר), but is actually Kaph sophit (ך)
 Nun (נ) looks a lot like Vav (ו), but is actually Nun sophit (ן)

## Nikkud / Dots

In addition to the letters, Hebrew also has special diacritical marks. These marks modify how
letters are pronounced. These modifiers are often not used by native speakers as they know
how to properly pronounce words through experience.

