const HYPHEN_DASH_CHARACTERS: &[char] = &[
    '\u{002D}', // hyphen-minus U+002D
    '\u{02D7}', // modifier letter minus sign U+02D7
    '\u{05BE}', // hebrew punctuation maqaf U+05BE
    '\u{058A}', // armenian hyphen U+058A
    '\u{1400}', // canadian syllabics carrier khaki U+1400
    '\u{1806}', // mongolian 't'odo soft hyphen U+1806
    '\u{2010}', // hyphen U+2010
    '\u{2011}', // non-breaking hyphen U+2011
    '\u{2012}', // figure dash U+2012
    '\u{2013}', // en dash U+2013
    '\u{2014}', // em dash U+2014
    '\u{2015}', // horizontal bar U+2015
    '\u{2043}', // hyphen bullet U+2043
    '\u{2053}', // swung dash U+2053
    '\u{207B}', // superscript minus U+207B
    '\u{208B}', // subscript minus U+208B
    '\u{2212}', // minus sign U+2212
    '\u{23AF}', // horizontal line extension U+23AF
    '\u{23BA}', // horizontal scan line-1 U+23BA
    '\u{23BB}', // horizontal scan line-3 U+23BB
    '\u{23BC}', // horizontal scan line-7 U+23BC
    '\u{23E4}', // straightness U+23E4
    '\u{2500}', // box drawings light horizontal U+2500
    '\u{2501}', // box drawings heavy horizontal U+2501
    '\u{254C}', // box drawings light double dash horizontal U+254C
    '\u{254D}', // box drawings heavy double dash horizontal U+254D
    '\u{2574}', // box drawings light left U+2574
    '\u{2576}', // box drawings light right U+2576
    '\u{2578}', // box drawings heavy left U+2578
    '\u{257A}', // box drawings heavy right U+257A
    '\u{2796}', // heavy minus sign U+2796
    '\u{29FF}', // right-pointing curved angle bracket U+29FF
    '\u{2E3A}', // two-em dash U+2E3A
    '\u{2E3B}', // three-em dash U+2E3B
    '\u{301C}', // wave dash U+301C
    '\u{FE58}', // small em dash U+FE58
    '\u{FE63}', // small hyphen-minus U+FE63
    '\u{FF0D}', // fullwidth hyphen-minus U+FF0D
];

const UNDERSCORE_CHARACTERS: &[char] = &[
    '\u{005F}',  // low line U+005F
    '\u{02CD}',  // modifier letter low macron U+02CD
    '\u{FF3F}',  // fullwidth low line U+FF3F
    '\u{1BC96}', // duployan affix low line U+1BC96
    '\u{0332}',  // combining low line U+0332
    '\u{0333}',  // combining double low line U+0333
    '\u{2017}',  // double low line U+2017
    '\u{203F}',  // undertie U+203F
    '\u{2581}',  // lower one eighth block U+2581
    '\u{23B5}',  // bottom square bracket U+23B5
    '\u{23BD}',  // horizontal scan line-9 U+23BD
    '\u{FE4D}',  // dashed low line U+FE4D
    '\u{FE4E}',  // centreline low line U+FE4E
    '\u{FE4F}',  // wavy low line U+FE4F
];

const INVISIBLE_CHARACTERS: &[char] = &[
    '\u{00AD}', // Soft Hyphen
    '\u{034F}', // Combining Grapheme Joiner
    '\u{180E}', // Mongolian Vowel Separator
    '\u{200B}', // Zero-Width Space
    '\u{200C}', // Zero-Width Non-Joiner
    '\u{200D}', // Zero-Width Joiner
    '\u{2060}', // Word Joiner
    '\u{2062}', // Invisible Times
    '\u{2063}', // Invisible Separator
    '\u{2064}', // Invisible Plus
    '\u{FEFF}', // Zero-Width No-Break Space (BOM)
];

pub fn is_invisible_char(c: &char) -> bool {
    INVISIBLE_CHARACTERS.contains(c)
}

pub fn is_underscore_char(c: &char) -> bool {
    UNDERSCORE_CHARACTERS.contains(c)
}

pub fn is_hyphen_dash_char(c: &char) -> bool {
    HYPHEN_DASH_CHARACTERS.contains(c)
}
