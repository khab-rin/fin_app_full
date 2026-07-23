use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    sqlx::Type,
    EnumString,
    Display,
    AsRefStr,
)]
#[sqlx(type_name = "TEXT")]
pub enum Account {
    // Раздел I. Внеоборотные активы
    #[strum(serialize = "01")]
    #[serde(rename = "01")]
    #[sqlx(rename = "01")]
    PPE,

    #[strum(serialize = "02")]
    #[serde(rename = "02")]
    #[sqlx(rename = "02")]
    Depreciation,

    #[strum(serialize = "03")]
    #[serde(rename = "03")]
    #[sqlx(rename = "03")]
    InvestmentProperty,

    #[strum(serialize = "04")]
    #[serde(rename = "04")]
    #[sqlx(rename = "04")]
    Intangibles,

    #[strum(serialize = "05")]
    #[serde(rename = "05")]
    #[sqlx(rename = "05")]
    IntangAmort,

    #[strum(serialize = "06")]
    #[serde(rename = "06")]
    #[sqlx(rename = "06")]
    RnD,

    #[strum(serialize = "07")]
    #[serde(rename = "07")]
    #[sqlx(rename = "07")]
    InstallEquipment,

    #[strum(serialize = "08")]
    #[serde(rename = "08")]
    #[sqlx(rename = "08")]
    CAPEX,

    #[strum(serialize = "09")]
    #[serde(rename = "09")]
    #[sqlx(rename = "09")]
    TaxAssets,

    // Раздел II. Производственные запасы
    #[strum(serialize = "10")]
    #[serde(rename = "10")]
    #[sqlx(rename = "10")]
    Materials,

    #[strum(serialize = "11")]
    #[serde(rename = "11")]
    #[sqlx(rename = "11")]
    Animals,

    #[strum(serialize = "14")]
    #[serde(rename = "14")]
    #[sqlx(rename = "14")]
    InventoryReserve,

    #[strum(serialize = "15")]
    #[serde(rename = "15")]
    #[sqlx(rename = "15")]
    MaterialPurchases,

    #[strum(serialize = "16")]
    #[serde(rename = "16")]
    #[sqlx(rename = "16")]
    MaterialVariances,

    #[strum(serialize = "19")]
    #[serde(rename = "19")]
    #[sqlx(rename = "19")]
    InputVAT,

    // Раздел III. Затраты на производство
    #[strum(serialize = "20")]
    #[serde(rename = "20")]
    #[sqlx(rename = "20")]
    MainProduction,

    #[strum(serialize = "21")]
    #[serde(rename = "21")]
    #[sqlx(rename = "21")]
    SemiFinished,

    #[strum(serialize = "23")]
    #[serde(rename = "23")]
    #[sqlx(rename = "23")]
    SupportServices,

    #[strum(serialize = "25")]
    #[serde(rename = "25")]
    #[sqlx(rename = "25")]
    FactoryOverhead,

    #[strum(serialize = "26")]
    #[serde(rename = "26")]
    #[sqlx(rename = "26")]
    GeneralOverhead,

    #[strum(serialize = "28")]
    #[serde(rename = "28")]
    #[sqlx(rename = "28")]
    ProductionDefects,

    #[strum(serialize = "29")]
    #[serde(rename = "29")]
    #[sqlx(rename = "29")]
    SocialFacilities,

    // Раздел IV. Готовая продукция и товары
    #[strum(serialize = "40")]
    #[serde(rename = "40")]
    #[sqlx(rename = "40")]
    Products,

    #[strum(serialize = "41")]
    #[serde(rename = "41")]
    #[sqlx(rename = "41")]
    Goods,

    #[strum(serialize = "42")]
    #[serde(rename = "42")]
    #[sqlx(rename = "42")]
    TradingMargin,

    #[strum(serialize = "43")]
    #[serde(rename = "43")]
    #[sqlx(rename = "43")]
    FinishedGoods,

    #[strum(serialize = "44")]
    #[serde(rename = "44")]
    #[sqlx(rename = "44")]
    SellingExpenses,

    #[strum(serialize = "45")]
    #[serde(rename = "45")]
    #[sqlx(rename = "45")]
    ShippedGoods,

    #[strum(serialize = "46")]
    #[serde(rename = "46")]
    #[sqlx(rename = "46")]
    CompletedContractStages,

    // Раздел V. Денежные средства
    #[strum(serialize = "50")]
    #[serde(rename = "50")]
    #[sqlx(rename = "50")]
    Cash,

    #[strum(serialize = "50.01")]
    #[serde(rename = "50.01")]
    #[sqlx(rename = "50.01")]
    CashInHand,

    #[strum(serialize = "50.02")]
    #[serde(rename = "50.02")]
    #[sqlx(rename = "50.02")]
    OperatingCash,

    #[strum(serialize = "50.03")]
    #[serde(rename = "50.03")]
    #[sqlx(rename = "50.03")]
    MonetaryDocuments,

    #[strum(serialize = "51")]
    #[serde(rename = "51")]
    #[sqlx(rename = "51")]
    BankAcc,

    #[strum(serialize = "52")]
    #[serde(rename = "52")]
    #[sqlx(rename = "52")]
    ForBankAcc,

    #[strum(serialize = "55")]
    #[serde(rename = "55")]
    #[sqlx(rename = "55")]
    SpecBankAcc,

    #[strum(serialize = "57")]
    #[serde(rename = "57")]
    #[sqlx(rename = "57")]
    TransfersInTransit,

    #[strum(serialize = "58")]
    #[serde(rename = "58")]
    #[sqlx(rename = "58")]
    FinancialInvestments,

    #[strum(serialize = "59")]
    #[serde(rename = "59")]
    #[sqlx(rename = "59")]
    InvestmentReserve,

    // Раздел VI. Расчеты
    #[strum(serialize = "60")]
    #[serde(rename = "60")]
    #[sqlx(rename = "60")]
    Vendors,

    #[strum(serialize = "62")]
    #[serde(rename = "62")]
    #[sqlx(rename = "62")]
    Customers,

    #[strum(serialize = "63")]
    #[serde(rename = "63")]
    #[sqlx(rename = "63")]
    BadDebtReserve,

    #[strum(serialize = "66")]
    #[serde(rename = "66")]
    #[sqlx(rename = "66")]
    ShortLoans,

    #[strum(serialize = "67")]
    #[serde(rename = "67")]
    #[sqlx(rename = "67")]
    LongLoans,

    #[strum(serialize = "68")]
    #[serde(rename = "68")]
    #[sqlx(rename = "68")]
    Taxes,

    #[strum(serialize = "69")]
    #[serde(rename = "69")]
    #[sqlx(rename = "69")]
    Social,

    #[strum(serialize = "70")]
    #[serde(rename = "70")]
    #[sqlx(rename = "70")]
    Payroll,

    #[strum(serialize = "71")]
    #[serde(rename = "71")]
    #[sqlx(rename = "71")]
    Advances,

    #[strum(serialize = "73")]
    #[serde(rename = "73")]
    #[sqlx(rename = "73")]
    OtherPayrollTransactions,

    #[strum(serialize = "75")]
    #[serde(rename = "75")]
    #[sqlx(rename = "75")]
    OwnerSettlements,

    #[strum(serialize = "76")]
    #[serde(rename = "76")]
    #[sqlx(rename = "76")]
    OtherPayables,

    #[strum(serialize = "77")]
    #[serde(rename = "77")]
    #[sqlx(rename = "77")]
    DeferredTaxLiabilities,

    #[strum(serialize = "79")]
    #[serde(rename = "79")]
    #[sqlx(rename = "79")]
    InternalSettlements,

    // Раздел VII. Капитал
    #[strum(serialize = "80")]
    #[serde(rename = "80")]
    #[sqlx(rename = "80")]
    EquityCapital,

    #[strum(serialize = "81")]
    #[serde(rename = "81")]
    #[sqlx(rename = "81")]
    TreasuryStock,

    #[strum(serialize = "82")]
    #[serde(rename = "82")]
    #[sqlx(rename = "82")]
    ReserveCapital,

    #[strum(serialize = "83")]
    #[serde(rename = "83")]
    #[sqlx(rename = "83")]
    AdditionalCapital,

    #[strum(serialize = "84")]
    #[serde(rename = "84")]
    #[sqlx(rename = "84")]
    RetainedProfit,

    #[strum(serialize = "86")]
    #[serde(rename = "86")]
    #[sqlx(rename = "86")]
    TargetedFinancing,

    // Раздел VIII. Финансовые результаты
    #[strum(serialize = "90")]
    #[serde(rename = "90")]
    #[sqlx(rename = "90")]
    Revenue,

    #[strum(serialize = "91")]
    #[serde(rename = "91")]
    #[sqlx(rename = "91")]
    OtherIncome,

    #[strum(serialize = "94")]
    #[serde(rename = "94")]
    #[sqlx(rename = "94")]
    ShortagesAndLosses,

    #[strum(serialize = "96")]
    #[serde(rename = "96")]
    #[sqlx(rename = "96")]
    Provisions,

    #[strum(serialize = "97")]
    #[serde(rename = "97")]
    #[sqlx(rename = "97")]
    DeferredExpenses,

    #[strum(serialize = "98")]
    #[serde(rename = "98")]
    #[sqlx(rename = "98")]
    DeferredIncome,

    #[strum(serialize = "99")]
    #[serde(rename = "99")]
    #[sqlx(rename = "99")]
    FinancialResults,
}

impl Account {
    pub const fn info(self) -> AccInfo {
        match self {
            // Раздел I. Внеоборотные активы
            Account::PPE => AccInfo { code: "01", name: "Основные средства" },
            Account::Depreciation => AccInfo { code: "02", name: "Aмортизация основных средств" },
            Account::InvestmentProperty => AccInfo { code: "03", name: "Доходные вложения в материальные ценности" },
            Account::Intangibles => AccInfo { code: "04", name: "Нематериальные активы" },
            Account::IntangAmort => AccInfo { code: "05", name: "Амортизация нематериальных активов" },
            Account::RnD => AccInfo { code: "06", name: "Расходы на научно-исследовательские, опытно-конструкторские и технологические работы" },
            Account::InstallEquipment => AccInfo { code: "07", name: "Оборудование к установке" },
            Account::CAPEX => AccInfo { code: "08", name: "Вложения во внеоборотные активы" },
            Account::TaxAssets => AccInfo { code: "09", name: "Отложенные налоговые активы" },

            // Раздел II. Производственные запасы
            Account::Materials => AccInfo { code: "10", name: "Материалы" },
            Account::Animals => AccInfo { code: "11", name: "Животные на выращивании и откорме" },
            Account::InventoryReserve => AccInfo { code: "14", name: "Резервы под снижение стоимости материальных ценностей" },
            Account::MaterialPurchases => AccInfo { code: "15", name: "Заготовление и приобретение материальных ценностей" },
            Account::MaterialVariances => AccInfo { code: "16", name: "Отклонение в стоимости материальных ценностей" },
            Account::InputVAT => AccInfo { code: "19", name: "Налог на добавленную стоимость по приобретенным ценностям" },

            // Раздел III. Затраты на производство
            Account::MainProduction => AccInfo { code: "20", name: "Основное производство" },
            Account::SemiFinished => AccInfo { code: "21", name: "Полуфабрикаты собственного производства" },
            Account::SupportServices => AccInfo { code: "23", name: "Вспомогательные производства" },
            Account::FactoryOverhead => AccInfo { code: "25", name: "Общепроизводственные расходы" },
            Account::GeneralOverhead => AccInfo { code: "26", name: "Общехозяйственные расходы" },
            Account::ProductionDefects => AccInfo { code: "28", name: "Брак в производстве" },
            Account::SocialFacilities => AccInfo { code: "29", name: "Обслуживающие производства и хозяйства" },

            // Раздел IV. Готовая продукция и товары
            Account::Products => AccInfo { code: "40", name: "Выпуск продукции (работ, услуг)" },
            Account::Goods => AccInfo { code: "41", name: "Товары" },
            Account::TradingMargin => AccInfo { code: "42", name: "Торговая наценка" },
            Account::FinishedGoods => AccInfo { code: "43", name: "Готовая продукция" },
            Account::SellingExpenses => AccInfo { code: "44", name: "Расходы на продажу" },
            Account::ShippedGoods => AccInfo { code: "45", name: "Товары отгруженные" },
            Account::CompletedContractStages => AccInfo { code: "46", name: "Выполненные этапы по незавершенным работам" },

            // Раздел V. Денежные средства
            Account::Cash => AccInfo { code: "50", name: "Касса" },
            Account::CashInHand => AccInfo { code: "50.01", name: "Касса организации" },
            Account::OperatingCash => AccInfo { code: "50.02", name: "Операционная касса" },
            Account::MonetaryDocuments => AccInfo { code: "50.03", name: "Денежные документы" },
            Account::BankAcc => AccInfo { code: "51", name: "Расчетные счета" },
            Account::ForBankAcc => AccInfo { code: "52", name: "Валютные счета" },
            Account::SpecBankAcc => AccInfo { code: "55", name: "Специальные счета в банках" },
            Account::TransfersInTransit => AccInfo { code: "57", name: "Переводы в пути" },
            Account::FinancialInvestments => AccInfo { code: "58", name: "Финансовые вложения" },
            Account::InvestmentReserve => AccInfo { code: "59", name: "Резервы под обесценение финансовых вложений" },

            // Раздел VI. Расчеты
            Account::Vendors => AccInfo { code: "60", name: "Расчеты с поставщиками и подрядчиками" },
            Account::Customers => AccInfo { code: "62", name: "Расчеты с покупателями и заказчиками" },
            Account::BadDebtReserve => AccInfo { code: "63", name: "Резервы по сомнительным долгам" },
            Account::ShortLoans => AccInfo { code: "66", name: "Расчеты по краткосрочным кредитам и займам" },
            Account::LongLoans => AccInfo { code: "67", name: "Расчеты по долгосрочным кредитам и займам" },
            Account::Taxes => AccInfo { code: "68", name: "Расчеты по налогам и сборам" },
            Account::Social => AccInfo { code: "69", name: "Расчеты по социальному страхованию и обеспечению" },
            Account::Payroll => AccInfo { code: "70", name: "Расчеты с персоналом по оплате труда" },
            Account::Advances => AccInfo { code: "71", name: "Расчеты с подотчетными лицами" },
            Account::OtherPayrollTransactions => AccInfo { code: "73", name: "Расчеты с персоналом по прочим операциям" },
            Account::OwnerSettlements => AccInfo { code: "75", name: "Расчеты с учредителями" },
            Account::OtherPayables => AccInfo { code: "76", name: "Расчеты с разными дебиторами и кредиторами" },
            Account::DeferredTaxLiabilities => AccInfo { code: "77", name: "Отложенные налоговые обязательства" },
            Account::InternalSettlements => AccInfo { code: "79", name: "Внутрихозяйственные расчеты" },

            // Раздел VII. Капитал
            Account::EquityCapital => AccInfo { code: "80", name: "Уставный капитал" },
            Account::TreasuryStock => AccInfo { code: "81", name: "Собственные доли (акции)" },
            Account::ReserveCapital => AccInfo { code: "82", name: "Резервный капитал" },
            Account::AdditionalCapital => AccInfo { code: "83", name: "Добавочный капитал" },
            Account::RetainedProfit => AccInfo { code: "84", name: "Нераспределенная прибыль (непокрытый убыток)" },
            Account::TargetedFinancing => AccInfo { code: "86", name: "Целевое финансирование" },

            // Раздел VIII. Финансовые результаты
            Account::Revenue => AccInfo { code: "90", name: "Продажи" },
            Account::OtherIncome => AccInfo { code: "91", name: "Прочие доходы и расходы" },
            Account::ShortagesAndLosses => AccInfo { code: "94", name: "Недостачи и потери от порчи ценностей" },
            Account::Provisions => AccInfo { code: "96", name: "Резервы предстоящих расходов" },
            Account::DeferredExpenses => AccInfo { code: "97", name: "Расходы будущих периодов" },
            Account::DeferredIncome => AccInfo { code: "98", name: "Доходы будущих периодов" },
            Account::FinancialResults => AccInfo { code: "99", name: "Прибыли и убытки" },
        }
    }

    pub const fn get_correspondents(self) -> &'static [Account] {
        match self {
            // Раздел I. Внеоборотные активы
            Account::PPE => &[
                Account::CAPEX,
                Account::EquityCapital,
                Account::FinancialResults,
                Account::Vendors,
                Account::OtherIncome,
            ],
            Account::Depreciation => &[
                Account::MainProduction,
                Account::SupportServices,
                Account::FactoryOverhead,
                Account::GeneralOverhead,
                Account::SellingExpenses,
                Account::SocialFacilities,
                Account::PPE,
            ],
            Account::InvestmentProperty => &[
                Account::CAPEX,
                Account::OtherIncome,
                Account::Vendors,
            ],
            Account::Intangibles => &[
                Account::CAPEX,
                Account::EquityCapital,
                Account::OtherIncome,
            ],
            Account::IntangAmort => &[
                Account::MainProduction,
                Account::FactoryOverhead,
                Account::GeneralOverhead,
                Account::SellingExpenses,
                Account::Intangibles,
            ],
            Account::RnD => &[
                Account::CAPEX,
                Account::MainProduction,
                Account::GeneralOverhead,
            ],
            Account::InstallEquipment => &[
                Account::Vendors,
                Account::Advances,
                Account::CAPEX,
            ],
            Account::CAPEX => &[
                Account::Vendors,
                Account::Materials,
                Account::Payroll,
                Account::Social,
                Account::InstallEquipment,
                Account::PPE,
                Account::Intangibles,
            ],
            Account::TaxAssets => &[
                Account::Taxes,
                Account::FinancialResults,
            ],

            // Раздел II. Производственные запасы
            Account::Materials => &[
                Account::Vendors,
                Account::Advances,
                Account::MainProduction,
                Account::SupportServices,
                Account::FactoryOverhead,
                Account::GeneralOverhead,
                Account::OtherIncome,
            ],
            Account::Animals => &[
                Account::Vendors,
                Account::MainProduction,
                Account::OtherIncome,
            ],
            Account::InventoryReserve => &[
                Account::OtherIncome,
                Account::Materials,
            ],
            Account::MaterialPurchases => &[
                Account::Vendors,
                Account::Materials,
            ],
            Account::MaterialVariances => &[
                Account::MaterialPurchases,
                Account::MainProduction,
                Account::FactoryOverhead,
            ],
            Account::InputVAT => &[
                Account::Vendors,
                Account::OtherPayables,
                Account::Taxes,
            ],

            // Раздел III. Затраты на производство
            Account::MainProduction => &[
                Account::Materials,
                Account::Payroll,
                Account::Social,
                Account::Depreciation,
                Account::FactoryOverhead,
                Account::GeneralOverhead,
                Account::FinishedGoods,
                Account::Products,
            ],
            Account::SemiFinished => &[
                Account::MainProduction,
            ],
            Account::SupportServices => &[
                Account::Materials,
                Account::Payroll,
                Account::Social,
                Account::MainProduction,
                Account::FactoryOverhead,
            ],
            Account::FactoryOverhead => &[
                Account::Materials,
                Account::Payroll,
                Account::Social,
                Account::Depreciation,
                Account::MainProduction,
            ],
            Account::GeneralOverhead => &[
                Account::Materials,
                Account::Payroll,
                Account::Social,
                Account::Depreciation,
                Account::MainProduction,
                Account::Revenue,
            ],
            Account::ProductionDefects => &[
                Account::MainProduction,
                Account::Materials,
                Account::Payroll,
                Account::OtherPayables,
            ],
            Account::SocialFacilities => &[
                Account::Materials,
                Account::Payroll,
                Account::Social,
                Account::Depreciation,
                Account::OtherIncome,
            ],

            // Раздел IV. Готовая продукция и товары
            Account::Products => &[
                Account::MainProduction,
                Account::FinishedGoods,
            ],
            Account::Goods => &[
                Account::Vendors,
                Account::Advances,
                Account::Revenue,
            ],
            Account::TradingMargin => &[
                Account::Goods,
                Account::Revenue,
            ],
            Account::FinishedGoods => &[
                Account::MainProduction,
                Account::Products,
                Account::Revenue,
                Account::ShippedGoods,
            ],
            Account::SellingExpenses => &[
                Account::Materials,
                Account::Payroll,
                Account::Social,
                Account::Vendors,
                Account::Revenue,
            ],
            Account::ShippedGoods => &[
                Account::FinishedGoods,
                Account::Goods,
                Account::Revenue,
            ],
            Account::CompletedContractStages => &[
                Account::MainProduction,
                Account::Customers,
            ],

            // Раздел V. Денежные средства
            Account::Cash | Account::CashInHand | Account::OperatingCash | Account::MonetaryDocuments => &[
                Account::BankAcc,
                Account::Customers,
                Account::Advances,
                Account::Payroll,
                Account::OtherPayrollTransactions,
            ],
            Account::BankAcc => &[
                Account::Customers,
                Account::Cash,
                Account::CashInHand,
                Account::Vendors,
                Account::Taxes,
                Account::Social,
                Account::Payroll,
                Account::ShortLoans,
                Account::LongLoans,
                Account::OtherPayables,
            ],
            Account::ForBankAcc => &[
                Account::Customers,
                Account::BankAcc,
                Account::Vendors,
                Account::TransfersInTransit,
            ],
            Account::SpecBankAcc => &[
                Account::BankAcc,
                Account::Vendors,
                Account::ShortLoans,
            ],
            Account::TransfersInTransit => &[
                Account::BankAcc,
                Account::ForBankAcc,
                Account::Cash,
                Account::CashInHand,
            ],
            Account::FinancialInvestments => &[
                Account::BankAcc,
                Account::OtherIncome,
            ],
            Account::InvestmentReserve => &[
                Account::OtherIncome,
                Account::FinancialInvestments,
            ],

            // Раздел VI. Расчеты
            Account::Vendors => &[
                Account::BankAcc,
                Account::Cash,
                Account::CashInHand,
                Account::Materials,
                Account::Goods,
                Account::CAPEX,
                Account::GeneralOverhead,
                Account::InputVAT,
            ],
            Account::Customers => &[
                Account::Revenue,
                Account::BankAcc,
                Account::Cash,
                Account::CashInHand,
                Account::BadDebtReserve,
            ],
            Account::BadDebtReserve => &[
                Account::OtherIncome,
                Account::Customers,
            ],
            Account::ShortLoans => &[
                Account::BankAcc,
                Account::OtherIncome,
            ],
            Account::LongLoans => &[
                Account::BankAcc,
                Account::OtherIncome,
            ],
            Account::Taxes => &[
                Account::MainProduction,
                Account::GeneralOverhead,
                Account::Payroll,
                Account::Revenue,
                Account::OtherIncome,
                Account::BankAcc,
                Account::InputVAT,
            ],
            Account::Social => &[
                Account::Payroll,
                Account::MainProduction,
                Account::FactoryOverhead,
                Account::GeneralOverhead,
                Account::BankAcc,
            ],
            Account::Payroll => &[
                Account::MainProduction,
                Account::FactoryOverhead,
                Account::GeneralOverhead,
                Account::SellingExpenses,
                Account::BankAcc,
                Account::Cash,
                Account::CashInHand,
                Account::Taxes,
            ],
            Account::Advances => &[
                Account::BankAcc,
                Account::Cash,
                Account::CashInHand,
                Account::Materials,
                Account::SellingExpenses,
                Account::GeneralOverhead,
            ],
            Account::OtherPayrollTransactions => &[
                Account::Cash,
                Account::CashInHand,
                Account::BankAcc,
                Account::OtherIncome,
            ],
            Account::OwnerSettlements => &[
                Account::EquityCapital,
                Account::BankAcc,
                Account::RetainedProfit,
            ],
            Account::OtherPayables => &[
                Account::BankAcc,
                Account::Cash,
                Account::CashInHand,
                Account::OtherIncome,
                Account::GeneralOverhead,
            ],
            Account::DeferredTaxLiabilities => &[
                Account::Taxes,
                Account::FinancialResults,
            ],
            Account::InternalSettlements => &[
                Account::BankAcc,
                Account::Materials,
                Account::PPE,
            ],

            // Раздел VII. Капитал
            Account::EquityCapital => &[
                Account::OwnerSettlements,
                Account::RetainedProfit,
            ],
            Account::TreasuryStock => &[
                Account::BankAcc,
                Account::EquityCapital,
            ],
            Account::ReserveCapital => &[
                Account::RetainedProfit,
                Account::EquityCapital,
            ],
            Account::AdditionalCapital => &[
                Account::PPE,
                Account::EquityCapital,
            ],
            Account::RetainedProfit => &[
                Account::FinancialResults,
                Account::ReserveCapital,
                Account::OwnerSettlements,
                Account::EquityCapital,
            ],
            Account::TargetedFinancing => &[
                Account::BankAcc,
                Account::DeferredIncome,
                Account::OtherIncome,
            ],

            // Раздел VIII. Финансовые результаты
            Account::Revenue => &[
                Account::FinishedGoods,
                Account::Goods,
                Account::SellingExpenses,
                Account::GeneralOverhead,
                Account::Taxes,
                Account::FinancialResults,
                Account::Customers,
            ],
            Account::OtherIncome => &[
                Account::PPE,
                Account::Materials,
                Account::Vendors,
                Account::BankAcc,
                Account::FinancialResults,
            ],
            Account::ShortagesAndLosses => &[
                Account::Materials,
                Account::Goods,
                Account::Advances,
                Account::OtherIncome,
            ],
            Account::Provisions => &[
                Account::MainProduction,
                Account::GeneralOverhead,
                Account::SellingExpenses,
                Account::Payroll,
            ],
            Account::DeferredExpenses => &[
                Account::Vendors,
                Account::BankAcc,
                Account::MainProduction,
                Account::GeneralOverhead,
            ],
            Account::DeferredIncome => &[
                Account::TargetedFinancing,
                Account::OtherIncome,
            ],
            Account::FinancialResults => &[
                Account::Revenue,
                Account::OtherIncome,
                Account::Taxes,
                Account::RetainedProfit,
            ],
        }
    }

    pub const fn acc_type(self) -> AccType {
        match self {
            // --- Активные счета ---
            // Раздел I
            Account::PPE
            | Account::InvestmentProperty
            | Account::Intangibles
            | Account::RnD
            | Account::InstallEquipment
            | Account::CAPEX
            | Account::TaxAssets
            // Раздел II
            | Account::Materials
            | Account::Animals
            | Account::MaterialPurchases
            | Account::InputVAT
            // Раздел III
            | Account::MainProduction
            | Account::SemiFinished
            | Account::SupportServices
            | Account::FactoryOverhead
            | Account::GeneralOverhead
            | Account::ProductionDefects
            | Account::SocialFacilities
            // Раздел IV
            | Account::Products
            | Account::Goods
            | Account::FinishedGoods
            | Account::SellingExpenses
            | Account::ShippedGoods
            | Account::CompletedContractStages
            // Раздел V
            | Account::Cash
            | Account::CashInHand
            | Account::OperatingCash
            | Account::MonetaryDocuments
            | Account::BankAcc
            | Account::ForBankAcc
            | Account::SpecBankAcc
            | Account::TransfersInTransit
            | Account::FinancialInvestments
            // Раздел VIII
            | Account::ShortagesAndLosses
            | Account::DeferredExpenses => AccType::Active,

            // --- Пассивные счета ---
            // Регулирующие/контрактивные и капитальные
            Account::Depreciation
            | Account::IntangAmort
            | Account::InventoryReserve
            | Account::TradingMargin
            | Account::InvestmentReserve
            | Account::EquityCapital
            | Account::ReserveCapital
            | Account::AdditionalCapital
            | Account::TargetedFinancing
            | Account::Provisions
            | Account::DeferredIncome => AccType::Passive,

            // --- Активно-пассивные (Dual) счета ---
            // Расчеты и финрезультаты
            Account::MaterialVariances
            | Account::Vendors
            | Account::Customers
            | Account::BadDebtReserve
            | Account::ShortLoans
            | Account::LongLoans
            | Account::Taxes
            | Account::Social
            | Account::Payroll
            | Account::Advances
            | Account::OtherPayrollTransactions
            | Account::OwnerSettlements
            | Account::OtherPayables
            | Account::DeferredTaxLiabilities
            | Account::InternalSettlements
            | Account::TreasuryStock
            | Account::RetainedProfit
            | Account::Revenue
            | Account::OtherIncome
            | Account::FinancialResults => AccType::Dual,
        }
    }
}


pub enum AccType {
    Active,
    Passive,
    Dual
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccInfo {
    pub code: &'static str,
    pub name: &'static str,
}
