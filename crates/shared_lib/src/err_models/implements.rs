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

    BankParserReadFile = 101,
    BankParserWrongBytes = 102,
    BankParserGetHead = 103,
    BankParserGetBlock1 = 104,
    BankParserGetBlock2 = 105,
    BankParserWrongHead = 106,
    BankParserMissingFields = 107,
    BankParseHeadAccIsNotOwn = 108,

    ValidWrongDateValue = 121,
    ValidatorDecimalParsErr = 122,
    ValidWrongRubFValue = 123,
    ValidWrongInnValue = 124,
    ValidWrongKppValue = 125,
    ValidWrongBanAccValue = 126,
    ValidWrongOgrnValue = 127,
    ValidWrongBicValue = 128,
    ValidWrongNdsValue = 129,
    ValideWrongDateYear = 130,
    ValidWrongBranchValue = 131,
    ValidWrongOkpoValue = 132,
    ValidWrongOkpoCheckSum = 133,
    ValidWrongOktmoValue = 134,
    ValidWrongOkoguValue = 136,
    ValidWrongOkfsValue = 138,
    ValidWrongOkvedValue = 139,
    ValidWrongPhoneValue = 140,
    ValidWrongOpfValue = 141,
    ValidWrongOpfCheckSum = 142,
    ValidWrongRasBicAccPair = 143,
    ValidWrongCorBiccAccPair = 144,
    ValidWrongEnumValue = 145,
    ValidWrongRegionNumber = 146,
    ValidWrongUuid = 147,
    ValidNotarRegNum = 148,
    ValidWrongFio = 149,
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
    ValidWrongSnils = 166,
    ValidMchdMissManager = 167,
    ValidMchdStr1_50 = 168,
    ValidMchdStr1_28 = 169,
    ValidMchdDig4_4 = 170,
    ValidMchdMissSigature = 171,
    ValidMchdStr1_16000 = 172,
    ValidMchdStr1_2500 = 173,
    ValidMchdStr6_255 = 174,
    ValidMchdStr1_10000 = 175,
    ValidMchdMissPrincipal = 176,
    ValidMiddSubPrincipals = 177,
    ValidMiddDelegates = 178,
    ValidMchdStr1_5000 = 179,
    ValidDateTime = 180,
    ValidMchdDig10_10 = 181,
    ValidWrongEmailValue = 182,

    DadaSendQuery = 201,
    DadaMapPatternWr = 202,
    DadaFullMissData = 203,
    DadaCompStateNone = 204,

    AddCtrPryGetCompanyTypeQry = 211,
    AddCompanySerdeJsonMetadata = 212,
    AddCompanyMakeSerdeBankAcc = 213,
    AddCompanyAddBankAccQry = 214,
    AddCompanyInsertCompanyQry = 215,
    AddCompanyMissOpfMetadata = 216,
    AddCompanyMissOkvedMetadata = 217,
    AddCompanyWrongKPP = 218,
    AddCompanyMissOkved = 219,
    AddCompanyMissOpf = 220,
    AddCompanyMissCompState = 221,
    

    DataBaseWrongCompany = 250,

    FrontSqlQrySyncCompanysGetPoolBegin = 300,
    BackSqlQrySyncServerCompanysGetCopmQry = 301,
    BackSqlQrySyncServerCompanysInsertCompanyQry = 302,
    FrontBackPostError = 303,
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
    BackAuthSmsRuQuery = 322,
    BackAuthSmsruResponseMapping = 323,
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
    

    PersonWrongMetadataMap = 400,
    CompanyWrongMetadataMap = 401,
    

    ClientAuthRestoreByToken = 500,
    ClientAuthRestoreResponseMap = 501,
    ClientAuthRestoreResponseStatus = 502,
    ClientInitSessionError = 503,
    ClientInitSessionGetPathParrent = 504,
    ClientInitSessionInitSqlOptions = 505,
    ClientInitSessionInitPool = 506,
    ClientInitSqlxMigrate = 507,
    ClientLoginWronTokenInSystem = 508,

    UserWrongMapping = 1000,
    PersonWrongMapping = 1001,
    CompanyWrongMapping = 1002,
    
    BackSmsRuBalance = 1003,
    BackSqlQueryCallCf = 1004,
    SmsruGetResResponseStructWrongMapping = 1005,
    BackFunSmsRuCfFailed = 1006,
    BackSqlQuerySessions = 1007,
    BackSqlQueryUsers = 1008,

    CryptoServerError = 2001
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