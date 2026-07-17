use serde::{Serialize, Deserialize};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(Serialize, Deserialize)]
#[repr(u16)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, ts_rs::TS)]
pub enum Status {
    #[default]
    Success = 0,
    Unknown = 1,

    // ==========================================
    // 100-я группа: Ошибки валидации входных данных
    // (Превращаются в 400 Bad Request)
    // ==========================================
    ValideInput = 100,
    ValidDate = 101,
    ValidDecimalParser = 102,
    ValidRubFParser = 103,
    ValidInn = 104,
    ValidKpp = 105,
    ValidBankAcc = 106,
    ValidOgrn = 107,
    ValidBic = 108,
    ValidNds = 109,
    ValidBranchType = 110,
    ValidOkpo = 111,
    ValidOktmo = 112,
    ValidOkogu = 113,
    ValidOkfs = 114,
    ValidOkved = 115,
    ValidPhone = 116,
    ValidOpf = 117,
    ValidRasBicAcc = 118,
    ValidCorBiccAcc = 119,
    ValidEnum = 120,
    ValidRegionNumber = 121,
    ValidUuid = 122,
    ValidNotarRegNum = 123,
    ValidFio = 124,
    ValidMchdStr3_8 = 125,
    ValidMchdStr11_11 = 126,
    ValidMchdStr3_13 = 127,
    ValidMchdStr1_80 = 128,
    ValidMchdStr1_120 = 129,
    ValidMchdStr3_129 = 130,
    ValidMchdStr1_255 = 131,
    ValidMchdStr1_1000 = 132,
    ValidMchdPartStatus = 133,
    ValidMchdDig2_2 = 134,
    ValidMchdDig3_3 = 135,
    ValidMchdStr1_25 = 136,
    ValidMchdStr7_7 = 137,
    ValidMchdStr1_4000 = 138,
    ValidMchdDig12_12 = 139,
    ValidMchdStr1_250 = 140,
    ValidSnils = 141,
    ValidMchdMissManager = 142,
    ValidMchdStr1_50 = 143,
    ValidMchdStr1_28 = 144,
    ValidMchdDig4_4 = 145,
    ValidMchdStr1_16000 = 146,
    ValidMchdStr1_2500 = 147,
    ValidMchdStr6_255 = 148,
    ValidMchdStr1_10000 = 149,
    ValidMchdStr1_5000 = 150,
    ValidDateTime = 151,
    ValidMchdDig10_10 = 152,
    ValidEmail = 153,
    ValidBoxUuid = 154,
    ValidPassword = 155,
    CompInnValid = 156,
    PersInnValid = 157,
    ValidMchdDig7_7 = 158,

    // ==========================================
    // 200-я группа: Ошибки при работе с файлами и структуры данных
    // (Превращаются в 400 Bad Request / 500 Internal Server Error)
    // ==========================================
    FileInvalideFormat = 200,
    FileInvalideData = 201,
    MappingError = 202,
    SerializationError = 203,

    FileCreateError = 250,
    FileReadError = 251,
    FileWriteError = 252,
    DirCreateError = 253,

    // ==========================================
    // 300-я группа: Ошибки Баз Данных (Sqlite)
    // (Превращаются в 500 Internal Server Error)
    // ==========================================
    SqLitePoolErr = 300,
    SqlQueryWrongLogic = 301,
    SqliteCommitErr = 302,

    // ==========================================
    // 400-я группа: Системные ошибки бэкенда
    // (Превращаются в 500 Internal Server Error)
    // ==========================================
    SystemErr = 400,
    DataCorruptionErr = 401,
    SystemLogicErr = 402,
    ClientSessionMissError = 403,
    ResponseMappingError = 404,

    // ==========================================
    // 500-я группа: Ошибки интеграции с внешними API и сервисами
    // (Превращаются в 500 Internal Server Error)
    // ==========================================
    BackSmsRuBalance = 500,
    BackApiError = 501,
    CryptoServerError = 502,
    RequiredFieldsMiss = 503,
    DadataResponseError = 504,

    // ==========================================
    // 600-я группа: Ошибки выполнения HTTP/сетевых запросов
    // (Превращаются в 500 Internal Server Error)
    // ==========================================
    QueryGetRequestErr = 600,
    QueryBodyReadErr = 601,
    QueryPostRequestErr = 602,
    QueryResponseFormatErr = 603,
    QueryConnectErr = 604,


    
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
        let code = self as u16;

        let status_code = match code {

            100..250 => StatusCode::BAD_REQUEST,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, Json(self)).into_response()
    }
}

impl Status {
    pub fn from_u16(n: u16) -> Self {
        if n == 0 { return Status::Success; }
        if n == 1 { return Status::Unknown; }
        
        match n {
            100..=154 => unsafe { std::mem::transmute::<u16, Status>(n) },
            200..=202 => unsafe { std::mem::transmute::<u16, Status>(n) },
            250..=252 => unsafe { std::mem::transmute::<u16, Status>(n) },
            300..=302 => unsafe { std::mem::transmute::<u16, Status>(n) },
            400..=401 => unsafe { std::mem::transmute::<u16, Status>(n) },
            500..=502 => unsafe { std::mem::transmute::<u16, Status>(n) },
            600..=603 => unsafe { std::mem::transmute::<u16, Status>(n) },
            _ => Status::Unknown,
        }
    }
}