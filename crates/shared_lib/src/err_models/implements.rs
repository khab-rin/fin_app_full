use serde::{Serialize, Deserialize};

use axum::response::{IntoResponse, Response};

#[derive(Serialize, Deserialize)]
#[repr(u16)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    #[default]
    Success = 0,
    Unknown = 1,

    ValideInput = 120,
    ValidDate = 121,
    ValidDecimalParser = 122,
    ValidRubFParser = 123,
    ValidInn = 124,
    ValidKpp = 125,
    ValidBankAcc = 126,
    ValidOgrn = 127,
    ValidBic = 128,
    ValidNds = 129,
    ValidBranchType = 130,
    ValidOkpo = 132,
    ValidOktmo = 134,
    ValidOkogu = 136,
    ValidOkfs = 138,
    ValidOkved = 139,
    ValidPhone = 140,
    ValidOpf = 141,
    ValidRasBicAcc = 143,
    ValidCorBiccAcc = 144,
    ValidEnum = 145,
    ValidRegionNumber = 146,
    ValidUuid = 147,
    ValidNotarRegNum = 148,
    ValidFio = 149,
    ValidMchdStr3_8 = 150,
    ValidMchdStr11_11 = 151,
    ValidMchdStr3_13 = 152,
    ValidMchdStr1_80 = 153,
    ValidMchdStr1_120 = 154,
    ValidMchdStr3_129 = 155,
    ValidMchdStr1_255 = 156,
    ValidMchdStr1_1000 = 157,
    ValidMchdPartStatus = 158,
    ValidMchdDig2_2 = 159,
    ValidMchdDig3_3 = 160,
    ValidMchdStr1_25 = 161,
    ValidMchdStr7_7 = 162,
    ValidMchdStr1_4000 = 163,
    ValidMchdDig12_12 = 164,
    ValidMchdStr1_250 = 165,
    ValidSnils = 166,
    ValidMchdMissManager = 167,
    ValidMchdStr1_50 = 168,
    ValidMchdStr1_28 = 169,
    ValidMchdDig4_4 = 170,
    ValidMchdStr1_16000 = 172,
    ValidMchdStr1_2500 = 173,
    ValidMchdStr6_255 = 174,
    ValidMchdStr1_10000 = 175,
    ValidMchdStr1_5000 = 179,
    ValidDateTime = 180,
    ValidMchdDig10_10 = 181,
    ValidEmail = 182,
    ValidBoxUuid = 183,

    FileCreateError = 201,
    FileReadError = 202,
    FileWriteError = 203,
    FileInvalideFormat = 204,
    FileInvalideData = 205,

    MappingError = 301,
    
    SqlLiterPoolErr = 401,
    SqlQueryWrongLogic = 402,
    SqliteCommitErr = 403,

    SystemErr = 404,
    DataCorruptionErr = 405,

    BackSmsRuBalance = 501,
    BackApiError = 502,

    CryptoServerError = 601,

    QueryGetRequestErr = 701,
    QueryBodyReadErr = 702,
    QueryPostRequestErr = 703,
    QueryResponseFormatErr = 704,
}


impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({})", self, *self as u16)
    }
}

map_errors! {
    rust_decimal::Error     => Status::ValidDecimalParser,
    chrono::ParseError      => Status::ValidDate,
    std::num::ParseIntError => Status::Unknown,
    serde_json::Error       => Status::MappingError
}


impl IntoResponse for Status {
    fn into_response(self) -> Response {
        match serde_json::to_string(&self) {
            Ok(json_str) => json_str.into_response(),
            Err(_) => "Unknown".into_response(), // Запасной вариант
        }
    }
}

impl Status {
    pub fn from_u16(n: u16) -> Self {
        match n {
            0 => Status::Success,
            167 => Status::ValidMchdMissManager,
            _ => Status::Unknown,
        }
    }
}