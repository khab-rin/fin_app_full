use serde::{Serialize, Deserialize};

use axum::response::{IntoResponse, Response};

#[derive(Serialize, Deserialize)]
#[repr(u16)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    #[default]
    Success = 0,
    Unknown = 1,
    DecimalParse = 2,
    ChronoParse = 3,
    FromToJson = 4,

    MainPoolGetError = 51,

    

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

    FrontBackPostResponseParseError = 304,
    FrontCommitSuncCompanysQryErr = 305,
    FrontSqlGetIdInnKppPair = 306,
    BackGetUserIdsQuery = 307,
    BackSqlGetPersonTryFromDto = 308,
    BackSqlGetPersonWrongQueryLogic = 309,
    BackSqlAddHelperDtoToCompany = 310,
    BackGetCompanyQuery = 311,
    BackGetCheckPasswDataQuery = 312,
    BackAuthMissUser = 313,
    BackAuthWrongDbUserPassword = 314,
    BackAuthSqlGetUserQueryLogic = 315,
    BackAuthGetSeeionUserQuery = 316,
    BackAuthUserTryFromDto = 317,
    BackAuthSessionUserMiss = 318,
    BackAuthGetCheckPasswQueryLogic = 319,
    BackAuthWrongPassword = 320,
    BackAuthDeviceMiss = 321,
    BackAuthSmsRuWrongResponse = 324,
    BackAuthDelTokenMissUser = 325,
    BackAuthDelWarnTokenDevice = 326,
    BackAuthSendWarnMail = 327,
    BackAuthRestoreSessionLogicErr = 328,
    BackAuthNewCfQuery = 329,
    BackCallCfGetByDeviceExternQuery = 330,

    AuthShartPassword = 350,
    AuthDeviceIdErr = 351,
    AuthSendQuery = 352,
    AuthMapRegisterResponse = 353,
    AuthBackGetCompanyErr = 354,
    AuthWrongQueryLogic = 355,
    AuthBackGetPersonErr = 356,
    AuthClientCommandIsStateActiveDbErr = 357,
    AuthLoginGetToken = 358,
    AuthMissToken = 359,
    AuthGetUserIds = 361,
    AuthPersonMiss = 362,
    AuthGetCompany = 363,
    AuthGetPerson = 364,
    AuthCompanyMiss = 365,


    ClientAuthRestoreByToken = 500,
    ClientAuthRestoreResponseMap = 501,
    ClientAuthRestoreResponseStatus = 502,
    ClientInitSessionError = 503,
    ClientInitSessionGetPathParrent = 504,
    ClientInitSessionInitSqlOptions = 505,
    ClientInitSessionInitPool = 506,
    ClientInitSqlxMigrate = 507,
    ClientLoginWronTokenInSystem = 508,

    BackSmsRuBalance = 12003,
    BackSqlQueryCallCf = 12004,
    SmsruGetResResponseStructWrongMapping = 12005,
    BackFunSmsRuCfFailed = 12006,
    BackSqlQuerySessions = 12007,
    BackSqlQueryUsers = 12008,
    BackQueryGetErr = 12009,

    CryptoServerError = 3001,



    // Общие ошибки - запросы и так далее   
    QueryGetRequestErr = 1001,
    QueryBodyReadErr = 1002,
    QueryPostRequestErr = 1007,
    InvalideResponseFormat = 1008,

    FileReadError = 1003,
    InvalideFileFormat = 1005,
    InvalideFileData = 1006,

    MappingError = 1004,
    
    SqlLiterPoolErr = 1009,

    SqlQueryWrongLogic = 10010,
    

    // Общие ошибки - структуры
    UserWrongMapping = 2000,
    PersonWrongMapping = 2001,
    CompanyWrongMapping = 2002,
    CtrprtyMetadataWrongMapping = 2003,
    BoxUuidParsingErr = 2004,
    DadaRespWrapMappingErr = 2005,

    
    

    //BACK API 4000 - 4999
    SmsruGetResResponseMappigErr = 4001,
    SmsruCallResponseMappingErr = 4002,
    CryptoVerifyPersonResponseMappingErr = 4003,
    PersonMappingError = 4004,
    SqlPersonsQueryLogicErr = 4005,
    SqlCompanysQueryLogicErr = 4006,

}


impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({})", self, *self as u16)
    }
}

map_errors! {
    rust_decimal::Error     => Status::DecimalParse,
    chrono::ParseError      => Status::ChronoParse,
    std::num::ParseIntError => Status::Unknown,
    serde_json::Error       => Status::FromToJson
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