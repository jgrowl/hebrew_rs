# hebrew_rs

Hebrew alephbet primatives and parsing library for rust.

The aim for this library is to fully support all unicode supported Hebrew characters and
nikkud. Final and non-final versions are also distinguished. Simple word and phrases are
also supported, but there is still some work to be done.

Actual hebrew letters are used in enums as primary variants, but I intend to add constant
values or possibly convertable enums in english variations for users that are not comfortable
or find it preferable to use those instead.

As a general structure there are two main base enums for regular letters and final variants:

NonEnd & End

Then there is a higher level `Letter` struct that consolidates both versions. This struct
also provides any nikkud to be attached as Vector of `Dot`. Most unicode symbols are supported
but there may still be some that need to be accounted for.


# TODO

* Properly re-construct phrases using input instead of reconstructing with spaces


# Non Endings
א, ב, ג, ד, ה, ו, ז, ח, ט, י, כ, ל, מ, נ, ס, ע, פ, צ, ק, ר, ש, ת,

# Endings 
ך, ם, ן, ף, ץ

# Nikkud
    SHEVA
 ֱ     HATEF SEGOL
 ֲ     HATEF PATAH
 ֳ     HATEF QAMATS
 ִ     HIRIQ
 ֵ     TSERE
 ֶ     SEGOL
 ַ     PATAH
 ָ     QAMATS
 ֹ     HOLAM
 ֻ     QUBUTS
 ּ     DAGESH, MAPIQ, or SHURUQ
 ֽ     METEG
 ֿ     RAFE
 ׁ     SHIN DOT
 ׂ     SIN DOT
 ׄ     MARK UPPER DOT
 ׅ     MARK LOWER DOT
