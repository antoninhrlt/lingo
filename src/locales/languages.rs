// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use super::alphabet::Alphabet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Language {
    pub code: LanguageCode,
    alphabet: Alphabet,
}

impl Language {
    pub fn new(code: LanguageCode) -> Language {
        Language {
            code,
            alphabet: Alphabet::Unspecified,
        }
    }

    pub fn with_alphabet(code: LanguageCode, alphabet: Alphabet) -> Language {
        Language { code, alphabet }
    }

    pub fn to_code(&self) -> String {
        if self.alphabet != Alphabet::Unspecified {
            format!("{}_{}", self.code.to_code(), self.alphabet.to_code(),)
        } else {
            format!("{}", self.code.to_code())
        }
    }
}

use strum_macros::EnumString;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, EnumString)]
pub enum LanguageCode {
    ar,
    r#as,
    asa,
    az,
    bas,
    be,
    bem,
    bez,
    bg,
    bm,
    bn,
    bo,
    br,
    brx,
    bs,
    ca,
    ce,
    cgg,
    chr,
    cs,
    cy,
    da,
    dav,
    de,
    dje,
    dsb,
    dua,
    dyo,
    dz,
    ebu,
    ee,
    el,
    en,
    eo,
    es,
    et,
    eu,
    ewo,
    fa,
    ff,
    fi,
    fil,
    fo,
    fr,
    fur,
    fy,
    ga,
    gd,
    gl,
    gsw,
    gu,
    guz,
    gv,
    ha,
    haw,
    he,
    hi,
    hr,
    hsb,
    hu,
    hy,
    ig,
    ii,
    r#in,
    is,
    it,
    iw,
    ja,
    jgo,
    jmc,
    ka,
    kab,
    kam,
    kde,
    kea,
    khq,
    ki,
    kk,
    kkj,
    kl,
    kln,
    km,
    kn,
    ko,
    kok,
    ks,
    ksb,
    ksf,
    ksh,
    kw,
    ky,
    lag,
    lt,
    lb,
    lv,
    lg,
    mk,
    ms,
    mt,
    nl,
    no,
    pl,
    pt,
    ro,
    ru,
    sk,
    sl,
    sq,
    sr,
    sv,
    th,
    tr,
    uk,
    vi,
    zh,
    zu,
}

impl LanguageCode {
    pub fn to_full_name(&self) -> &str {
        use LanguageCode::*;

        match self {
            ar => "Arabic",
            r#as => "Assamese",
            asa => "Asu",
            az => "Azerbaijani",
            bas => "Basaa",
            be => "Belarusian",
            bem => "Bemba",
            bez => "Bena",
            bg => "Bulgarian",
            bm => "Bambara",
            bn => "Bangla / Bengali",
            bo => "Tibetan",
            br => "Breton",
            brx => "Bodo",
            bs => "Bosnian",
            ca => "Catalan",
            ce => "Chechen",
            cgg => "Chiga",
            chr => "Cherokee",
            cs => "Czech",
            cy => "Welsh",
            da => "Danish",
            dav => "Taita",
            de => "German",
            dje => "Zarma ",
            dsb => "Lower Sorbian",
            dua => "Duala",
            dyo => "Jola-Fonyi",
            dz => "Dzongkha",
            ebu => "Embu",
            ee => "Ewe",
            el => "Greek",
            en => "English",
            eo => "Esperanto",
            es => "Spanish",
            et => "Estonian",
            eu => "Basque",
            ewo => "Ewondo",
            fa => "Persian",
            ff => "Fulah",
            fi => "Finnish",
            fil => "Filipino",
            fo => "Faroese",
            fr => "French",
            fur => "Friulian",
            fy => "Western Frisian",
            ga => "Irish",
            gd => "Scottish Gaelic",
            gl => "Galician",
            gsw => "Swiss German",
            gu => "Gujarati",
            guz => "Gusii",
            gv => "Manx",
            ha => "Hausa",
            haw => "Hawaiian",
            he => "Hebrew",
            hi => "Hindi",
            hr => "Croatian",
            hsb => "Upper Sorbian",
            hu => "Hungarian",
            hy => "Armenian",
            ig => "Igbo",
            ii => "Sichuan Yi",
            r#in => "Indonesian",
            is => "Icelandic",
            it => "Italian",
            iw => "Hebrew",
            ja => "Japanese",
            jgo => "Ngomba",
            jmc => "Machame ",
            ka => "Georgian",
            kab => "Kabyle",
            kam => "Kamba",
            kde => "Makonde",
            kea => "Kabuverdianu",
            khq => "Koyra Chiini",
            ki => "Kikuyu",
            kk => "Kazakh",
            kkj => "Kako",
            kl => "Kalaallisut",
            kln => "Kalenjin",
            km => "Khmer",
            kn => "Kannada",
            ko => "Korean",
            kok => "Konkani",
            ks => "Kashmiri",
            ksb => "Shambala",
            ksf => "Bafia",
            ksh => "Colognian",
            kw => "Cornish",
            ky => "Kyrgyz",
            lag => "Langi",
            lt => "Lithuanian",
            lb => "Luxembourgish",
            lv => "Latvian",
            lg => "Ganda",
            mk => "Macedonian",
            ms => "Malay",
            mt => "Maltese",
            nl => "Dutch",
            no => "Norwegian",
            pl => "Polish",
            pt => "Portuguese",
            ro => "Romanian",
            ru => "Russian",
            sk => "Slovak",
            sl => "Slovenian",
            sq => "Albanian",
            sr => "Serbian",
            sv => "Swedish",
            th => "Thai",
            tr => "Turkish",
            uk => "Ukrainian",
            vi => "Vietnamese",
            zh => "Chinese",
            zu => "Zulu",
        }
    }

    pub fn to_code(&self) -> String {
        format!("{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language() {
        println!("{}", Language::new(LanguageCode::az).to_code());

        println!(
            "{}",
            Language::with_alphabet(LanguageCode::az, Alphabet::Cyrillic).to_code()
        );
    }

    #[test]
    fn language_full_name() {
        println!("{}", LanguageCode::ar.to_full_name())
    }
}
