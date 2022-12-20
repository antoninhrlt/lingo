// This file is part of "lingo"
// Under the MIT License
// Copyright (c) 2022 Antonin Hérault

use strum_macros::EnumString;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash, EnumString)]
pub enum CountryCode {
    AE,
    BH,
    DJ,
    DZ,
    EG,
    EH,
    ER,
    IL,
    IQ,
    JO,
    KM,
    KW,
    LB,
    LY,
    MA,
    MR,
    OM,
    PS,
    QA,
    SA,
    SD,
    SO,
    SS,
    SY,
    TD,
    TN,
    YE,
    IN,
    TZ,
    AZ,
    CM,
    BY,
    ZM,
    BG,
    ML,
    BD,
    FR,
    BA,
    AD,
    ES,
    IT,
    RU,
    UG,
    US,
    CZ,
    GB,
    DK,
    KE,
    AT,
    BE,
    CH,
    DE,
    LI,
    LU,
    NE,
    SN,
    BT,
    GH,
    TG,
    CY,
    GR,
    AG,
    AI,
    AS,
    AU,
    BB,
    BI,
    BM,
    BS,
    BW,
    BZ,
    CA,
    CC,
    CK,
    CX,
    DG,
    DM,
    FI,
    FJ,
    FK,
    FM,
    GD,
    GG,
    GI,
    GM,
    GU,
    GY,
    HK,
    IE,
    IM,
    IO,
    JE,
    JM,
    KI,
    KN,
    KY,
    LC,
    LR,
    LS,
    MG,
    MH,
    MO,
    MP,
    MS,
    MT,
    MU,
    MW,
    MY,
    NA,
    NF,
    NG,
    NL,
    NR,
    NU,
    NZ,
    PG,
    PH,
    PK,
    PN,
    PR,
    PW,
    SB,
    SC,
    SE,
    SG,
    SH,
    SI,
    SL,
    SX,
    SZ,
    TC,
    TK,
    TO,
    TT,
    TV,
    UM,
    VC,
    VG,
    VI,
    VU,
    WS,
    ZA,
    ZW,
    AR,
    BO,
    BR,
    CL,
    CO,
    CR,
    CU,
    DO,
    EA,
    EC,
    GQ,
    GT,
    HN,
    IC,
    MX,
    NI,
    PA,
    PE,
    PY,
    SV,
    UY,
    VE,
    EE,
    AF,
    IR,
    GN,
    FO,
    BF,
    BJ,
    BL,
    CD,
    CF,
    CG,
    CI,
    GA,
    GF,
    GP,
    HT,
    MC,
    MF,
    MQ,
    NC,
    PF,
    PM,
    RE,
    RW,
    WF,
    YT,
    HR,
    HU,
    AM,
    CN,
    ID,
    IS,
    SM,
    VA,
    JP,
    GE,
    CV,
    KZ,
    GL,
    KH,
    KP,
    KR,
    KG,
    LT,
    LV,
    MK,
    NO,
    NO_NY,
    PL,
    AO,
    GW,
    MZ,
    PT,
    ST,
    TL,
    RO,
    MD,
    UA,
    SK,
    AL,
    CS,
    ME,
    RS,
    TH,
    TH_TH,
    TR,
    VN,
    TW,
    /// Avoid usage of an `Option<T>`. The country code is not mandatory in a
    /// locale structure.
    None,
}

impl CountryCode {
    pub fn to_full_name(&self) -> &str {
        use CountryCode::*;

        match self {
            AE => "United Arab Emirates",
            BH => "Bahrain",
            DJ => "Djibouti",
            DZ => "Algeria",
            EG => "Egypt",
            EH => "Western Sahara",
            ER => "Eritrea",
            IL => "Israel",
            IQ => "Iraq",
            JO => "Jordan",
            KM => "Comoros",
            KW => "Kuwait",
            LB => "Lebanon",
            LY => "Libya",
            MA => "Morocco",
            MR => "Mauritania",
            OM => "Oman",
            PS => "Palestinian Territories",
            QA => "Qatar",
            SA => "Saudi Arabia",
            SD => "Sudan",
            SO => "Somalia",
            SS => "South Sudan",
            SY => "Syria",
            TD => "Chad",
            TN => "Tunisia",
            YE => "Yemen",
            IN => "India",
            TZ => "Tanzania",
            AZ => "Azerbaijan",
            CM => "Cameroon",
            BY => "Belarus",
            ZM => "Zambia",
            BG => "Bulgaria",
            ML => "Mali",
            BD => "Bangladesh",
            FR => "France",
            BA => "Bosnia & Herzegovina",
            AD => "Andorra",
            ES => "Spain",
            IT => "Italy",
            RU => "Russia",
            UG => "Uganda",
            US => "United States",
            CZ => "Czech Republic",
            GB => "United Kingdom",
            DK => "Denmark",
            KE => "Kenya",
            AT => "Austria",
            BE => "Belgium",
            CH => "Switzerland",
            DE => "Germany",
            LI => "Liechtenstein",
            LU => "Luxembourg",
            NE => "Niger",
            SN => "Senegal",
            BT => "Bhutan",
            GH => "Ghana",
            TG => "Togo",
            CY => "Cyprus",
            GR => "Greece",
            AG => "Antigua & Barbuda",
            AI => "Anguilla",
            AS => "American Samoa",
            AU => "Australia",
            BB => "Barbados",
            BI => "Burundi",
            BM => "Bermuda",
            BS => "Bahamas",
            BW => "Botswana",
            BZ => "Belize",
            CA => "Canada",
            CC => "Cocos [Keeling] Islands",
            CK => "Cook Islands",
            CX => "Christmas Island",
            DG => "Diego Garcia",
            DM => "Dominica",
            FI => "Finland",
            FJ => "Fiji",
            FK => "Falkland Islands",
            FM => "Micronesia",
            GD => "Grenada",
            GG => "Guernsey",
            GI => "Gibraltar",
            GM => "Gambia",
            GU => "Guam",
            GY => "Guyana",
            HK => "Hong Kong SAR China",
            IE => "Ireland",
            IM => "Isle of Man",
            IO => "British Indian Ocean Territory",
            JE => "Jersey",
            JM => "Jamaica",
            KI => "Kiribati",
            KN => "St. Kitts & Nevis",
            KY => "Cayman Islands",
            LC => "St. Lucia",
            LR => "Liberia",
            LS => "Lesotho",
            MG => "Madagascar",
            MH => "Marshall Islands",
            MO => "Macau SAR China",
            MP => "Northern Mariana Islands",
            MS => "Montserrat",
            MT => "Malta",
            MU => "Mauritius",
            MW => "Malawi",
            MY => "Malaysia",
            NA => "Namibia",
            NF => "Norfolk Island",
            NG => "Nigeria",
            NL => "Netherlands",
            NR => "Nauru",
            NU => "Niue",
            NZ => "New Zealand",
            PG => "Papua New Guinea",
            PH => "Philippines",
            PK => "Pakistan",
            PN => "Pitcairn Islands",
            PR => "Puerto Rico",
            PW => "Palau",
            SB => "Solomon Islands",
            SC => "Seychelles",
            SE => "Sweden",
            SG => "Singapore",
            SH => "St. Helena",
            SI => "Slovenia",
            SL => "Sierra Leone",
            SX => "Sint Maarten",
            SZ => "Swaziland",
            TC => "Turks & Caicos Islands",
            TK => "Tokelau",
            TO => "Tonga",
            TT => "Trinidad & Tobago",
            TV => "Tuvalu",
            UM => "U.S. Outlying Islands",
            VC => "St. Vincent & Grenadines",
            VG => "British Virgin Islands",
            VI => "U.S. Virgin Islands",
            VU => "Vanuatu",
            WS => "Samoa",
            ZA => "South Africa",
            ZW => "Zimbabwe",
            AR => "Argentina",
            BO => "Bolivia",
            BR => "Brazil",
            CL => "Chile",
            CO => "Colombia",
            CR => "Costa Rica",
            CU => "Cuba",
            DO => "Dominican Republic",
            EA => "Ceuta & Melilla",
            EC => "Ecuador",
            GQ => "Equatorial Guinea",
            GT => "Guatemala",
            HN => "Honduras",
            IC => "Canary Islands",
            MX => "Mexico",
            NI => "Nicaragua",
            PA => "Panama",
            PE => "Peru",
            PY => "Paraguay",
            SV => "El Salvador",
            UY => "Uruguay",
            VE => "Venezuela",
            EE => "Estonia",
            AF => "Afghanistan",
            IR => "Iran",
            GN => "Guinea",
            FO => "Faroe Islands",
            BF => "Burkina Faso",
            BJ => "Benin",
            BL => "St. Barthélemy",
            CD => "Congo - Kinshasa",
            CF => "Central African Republic",
            CG => "Congo - Brazzaville",
            CI => "Côte d'Ivoire",
            GA => "Gabon",
            GF => "French Guiana",
            GP => "Guadeloupe",
            HT => "Haiti",
            MC => "Monaco",
            MF => "St. Martin",
            MQ => "Martinique",
            NC => "New Caledonia",
            PF => "French Polynesia",
            PM => "St. Pierre & Miquelon",
            RE => "Réunion",
            RW => "Rwanda",
            WF => "Wallis & Futuna",
            YT => "Mayotte",
            HR => "Croatia",
            HU => "Hungary",
            AM => "Armenia",
            CN => "China",
            ID => "Indonesia",
            IS => "Iceland",
            SM => "San Marino",
            VA => "Vatican City",
            JP => "Japan",
            GE => "Georgia",
            CV => "Cape Verde",
            KZ => "Kazakhstan",
            GL => "Greenland",
            KH => "Cambodia",
            KP => "North Korea",
            KR => "South Korea",
            KG => "Kyrgyzstan",
            LT => "Lithuania",
            LV => "Latvia",
            MK => "Macedonia",
            NO => "Norway",
            NO_NY => "Norway Nynorsk",
            PL => "Poland",
            AO => "Angola",
            GW => "Guinea-Bissau",
            MZ => "Mozambique",
            PT => "Portugal",
            ST => "São Tomé & Príncipe",
            TL => "Timor-Leste",
            RO => "Romania",
            MD => "Moldova",
            UA => "Ukraine",
            SK => "Slovakia",
            AL => "Albania",
            CS => "Serbia and Montenegro",
            ME => "Montenegro",
            RS => "Serbia",
            TH => "Thailand",
            TH_TH => "Thailand",
            TR => "Turkey",
            VN => "Vietnam",
            TW => "Taiwan",
            CountryCode::None => panic!("cannot get the full name of a non-existant country code")
        }
    }

    pub fn to_code(&self) -> String {
        format!("{:?}", self)
    }
}
