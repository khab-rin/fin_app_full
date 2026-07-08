use crate::service::mchd::powers_btg::{BTGPowerInfo, BTGPowerLimitType, BTGDocLimit};


pub const SIGN_DIG_DOC: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_SIGN_DIG_DOC", name: "Подписывать электронные документы" };
pub const CHANGE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_CHANGE", name: "Изменять" };
pub const SIGN: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_SIGN", name: "Подписывать" };
pub const TRANSMIT: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_TRANSMIT", name: "Передавать" };
pub const HAND_OVER: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_HAND_OVER", name: "Сдавать" };
pub const DESIGN: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_DESIGN", name: "Оформлять" };
pub const REPRESENT_INTERESTS: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_REPRESENT_INTERESTS", name: "Представлять интересы" };
pub const REGISTER: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_REGISTER", name: "Регистрировать" };
pub const APPROVE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_APPROVE", name: "Утверждать" };
pub const CONCLUDE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_CONCLUDE", name: "Заключать" };
pub const PRESENT: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_PRESENT", name: "Предъявлять" };
pub const PERFORM_OPERATIONS: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_PERFORM_OPERATIONS", name: "Проводить операции" };
pub const FILL: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_FILL", name: "Заполнять" };
pub const PARTICIPATE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_PARTICIPATE", name: "Участвовать" };
pub const PERFORM_ON_BEHALF_OF: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_PERFORM_ON_BEHALF_OF", name: "Совершать от имени" };
pub const MAKE_CALCULATIONS: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_MAKE CALCULATIONS", name: "Производить расчеты" };
pub const CONDUCT_CORRESPONDENCE_AND_NEGOTIATIONS: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_CONDUCT_CORRESPONDENCE_AND_NEGOTIATIONS", name: "Вести переписку, переговоры" };
pub const PAY: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_PAY", name: "Оплачивать" };
pub const RECEIVE_COPIES: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_RECEIVE_COPIES", name: "Получать копии" };
pub const TAKE_ON_THE_ROLE_OF: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_TAKE_ON_THE_ROLE_OF", name: "Выступать в качестве" };
pub const SERVE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_SERVE", name: "Подавать" };
pub const INTRODUCE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_INTRODUCE", name: "Представлять" };
pub const REFUSE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_REFUSE", name: "Отказываться" };
pub const RECEIVE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_RECEIVE", name: "Получать" };
pub const MAKE_STATEMENTS: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_MAKE_STATEMENTS", name: "Делать выписки" };
pub const EXECUTE_ON_BEHALF_OF: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_EXECUTE_ON_BEHALF_OF", name: "Осуществлять от имени" };
pub const ACCEPT: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_ACCEPT", name: "Принимать" };
pub const COORDINATE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_COORDINATE", name: "Согласовывать" };
pub const CERTIFY_COPIES: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_CERTIFY_COPIES", name: "Заверять копии" };
pub const DISSOLVE: BTGPowerInfo = BTGPowerInfo { code: "KLASS_2.0_DISSOLVE", name: "Расторгать" };




pub const LIM_DOC_TYPE: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_01", name: "Тип документа" };
pub const LIM_OBLIGATION_TYPE: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_02", name: "Вид обязательств" };
pub const LIM_AMOUNT: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_03", name: "Лимит по сумме" };
pub const LIM_AMOUNT_TYPE: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_04", name: "Тип суммы" };
pub const LIM_CURRENCY_TYPE: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_05", name: "Тип валюты" };
pub const LIM_COUNTERPARTY: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_08", name: "Контрагент" };
pub const LIM_COUNTERPARTY_BRANCH: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_09", name: "Филиал контрагента" };
pub const LIM_TERRITORY: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_10", name: "Территория" };
pub const LIM_SERVICES: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_11", name: "Услуги" };
pub const LIM_WORKS: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_12", name: "Работы" };
pub const LIM_GOODS: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_13", name: "Товары" };
pub const LIM_AUTHORITY: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_15", name: "Орган власти" };
pub const LIM_OBJECT_TYPE: BTGPowerLimitType = BTGPowerLimitType { code: "LIM_17", name: "Тип объекта" };


pub const ACCOUNTANT_PRESET_ARRAY: &[BTGDocLimit] = &[
    DOC_REQUEST,
    DOC_WAYBILL,
    DOC_UPD,
    DOC_UKD,
    DOC_BREAKDOWN,
    DOC_INVOICE,
    DOC_FACTURA,
    DOC_CORR_FACTURA,
    DOC_EXPENSE_REPORT,
    DOC_ACT,
    DOC_OFFSET_ACT,
    DOC_ACCEPTANCE_ACT,
    DOC_RETURN_ACTS,
    DOC_BANK_EXTRACT,
    DOC_GENERAL_AGR,
    DOC_CREDIT_AGR,
    DOC_ADDITIONAL_AGR,
    DOC_KM4_JOURNAL,
    DOC_REQ_REGISTRY_EXTRACTS,
    DOC_APPLICATION,
    DOC_COLLECTION_ORDER,
    DOC_INFO_LETTER,
    DOC_CASH_BOOK,
    DOC_ANNOUNCEMENT,
    DOC_REQ_RESPONSE_LETTER,
    DOC_REPORT,
    DOC_RECEIPT,
    DOC_LETTER,
    DOC_NOTICE,
    DOC_CERT_OR_EXTRACT,
    DOC_CERTIFICATE,
    DOC_AGREEMENT,
    DOC_CONSENT,
    DOC_DEAL,
    DOC_REGISTRY,
    DOC_ORDER,
    DOC_CLAIM_PRETENZIA,
    DOC_PRELIMINARY_AGR,
    DOC_ORDER_GENERIC
];



pub const DOC_REQUEST: BTGDocLimit = BTGDocLimit { code: "LIM_01_168", name: "Запрос" };
pub const DOC_CONTRACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_15", name: "Договор" };
pub const DOC_CARGO_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_09", name: "Заказ (заявка) на перевозку" };
pub const DOC_CHARTERING: BTGDocLimit = BTGDocLimit { code: "LIM_01_13", name: "Договор фрахтования" };
pub const DOC_WAYBILL: BTGDocLimit = BTGDocLimit { code: "LIM_01_11", name: "Путевой лист" };
pub const DOC_TORG12: BTGDocLimit = BTGDocLimit { code: "LIM_01_04", name: "Товарная накладная по форме ТОРГ-12" };
pub const DOC_TECH_ACTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_20", name: "Технические акты (акты осмотра, акты оценки)" };
pub const DOC_UPD: BTGDocLimit = BTGDocLimit { code: "LIM_01_01", name: "Универсальный передаточный документ" };
pub const DOC_BREAKDOWN: BTGDocLimit = BTGDocLimit { code: "LIM_01_19", name: "Расшифровка" };
pub const DOC_WORK_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_03", name: "Акт приемки-сдачи работ или услуг" };
pub const DOC_FUEL_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_14", name: "Расходный ордер на заправку воздушных судов" };
pub const DOC_KS2: BTGDocLimit = BTGDocLimit { code: "LIM_01_12", name: "Акт по форме КС-2" };
pub const DOC_DISCREPANCY_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_06", name: "Акт приемки и/или расхождений" };
pub const DOC_PDFA3: BTGDocLimit = BTGDocLimit { code: "LIM_01_08", name: "Договорный документ PDF/A-3" };
pub const DOC_UKD: BTGDocLimit = BTGDocLimit { code: "LIM_01_16", name: "Универсальный корректировочный документ" };
pub const DOC_TRANSPORT_WAYBILL: BTGDocLimit = BTGDocLimit { code: "LIM_01_10", name: "Транспортная накладная" };
pub const DOC_INVOICE: BTGDocLimit = BTGDocLimit { code: "LIM_01_17", name: "Счет на оплату" };
pub const DOC_RECONCILIATION_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_07", name: "Акт сверки взаимных расчетов" };
pub const DOC_FACTURA: BTGDocLimit = BTGDocLimit { code: "LIM_01_02", name: "Счет-фактура" };
pub const DOC_CORR_FACTURA: BTGDocLimit = BTGDocLimit { code: "LIM_01_05", name: "Корректировочный счет-фактура" };
pub const DOC_PRICE_LIST: BTGDocLimit = BTGDocLimit { code: "LIM_01_18", name: "Прейскурант" };
pub const DOC_NOTIFICATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_21", name: "Уведомление (о смене реквизитов, тарифа и прочее)" };
pub const DOC_EXPENSE_REPORT: BTGDocLimit = BTGDocLimit { code: "LIM_01_22", name: "Авансовый отчет" };
pub const DOC_AGENCY_CONTRACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_23", name: "Агентский договор, направленный на заключение и (или) сопровождение договора" };
pub const DOC_LETTER_OF_CREDIT_UNSECURED: BTGDocLimit = BTGDocLimit { code: "LIM_01_24", name: "Аккредитив без предоставления приказодателем денежного покрытия, включая условия финансирования, предоставляемого банками контрагентами" };
pub const DOC_LETTER_OF_CREDIT_SECURED: BTGDocLimit = BTGDocLimit { code: "LIM_01_25", name: "Аккредитив с предоставлением приказодателем денежного покрытия в сумме аккредитива" };
pub const DOC_LETTER_OF_CREDIT_MOD: BTGDocLimit = BTGDocLimit { code: "LIM_01_26", name: "Аккредитив, изменения к нему, включая увеличение сумм и продление (пролонгацию) срока действия" };
pub const DOC_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_27", name: "Акт" };
pub const DOC_OFFSET_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_28", name: "Акт взаимозачета" };
pub const DOC_INSURANCE_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_29", name: "Акт взаиморасчета со страховой организацией" };
pub const DOC_INVENTORY_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_30", name: "Акт инвентаризации" };
pub const DOC_DEFECT_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_31", name: "Акт о выявленных недостатках" };
pub const DOC_CLAIM_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_32", name: "Акт о страховом случае" };
pub const DOC_SURVEY_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_33", name: "Акт обследования и категорирования" };
pub const DOC_INSPECTION_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_34", name: "Акт осмотра" };
pub const DOC_VALUATION_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_35", name: "Акт оценки" };
pub const DOC_KM3_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_36", name: "Акт по форме № КМ-3" };

pub const DOC_MX1: BTGDocLimit = BTGDocLimit { code: "LIM_01_37", name: "Акт по форме № МХ-1" };
pub const DOC_MX3: BTGDocLimit = BTGDocLimit { code: "LIM_01_38", name: "Акт по форме № МХ-3" };
pub const DOC_OS1B: BTGDocLimit = BTGDocLimit { code: "LIM_01_39", name: "Акт по форме № ОС-1б" };
pub const DOC_OS3: BTGDocLimit = BTGDocLimit { code: "LIM_01_40", name: "Акт по форме № ОС-3" };
pub const DOC_OS4: BTGDocLimit = BTGDocLimit { code: "LIM_01_41", name: "Акт по форме № ОС-4" };
pub const DOC_OS4A: BTGDocLimit = BTGDocLimit { code: "LIM_01_42", name: "Акт по форме № ОС-4а" };
pub const DOC_OS4B: BTGDocLimit = BTGDocLimit { code: "LIM_01_43", name: "Акт по форме № ОС-4б" };
pub const DOC_TORG15: BTGDocLimit = BTGDocLimit { code: "LIM_01_44", name: "Акт по форме № ТОРГ-15" };
pub const DOC_TORG16: BTGDocLimit = BTGDocLimit { code: "LIM_01_45", name: "Акт по форме № ТОРГ-16" };
pub const DOC_TORG2: BTGDocLimit = BTGDocLimit { code: "LIM_01_46", name: "Акт по форме № ТОРГ-2" };
pub const DOC_TORG3: BTGDocLimit = BTGDocLimit { code: "LIM_01_47", name: "Акт по форме № ТОРГ-3" };
pub const DOC_H1: BTGDocLimit = BTGDocLimit { code: "LIM_01_48", name: "Акт по форме Н-1" };
pub const DOC_ACCEPTANCE_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_49", name: "Акт приема-передачи" };
pub const DOC_DISCREPANCY_ACT_ALT: BTGDocLimit = BTGDocLimit { code: "LIM_01_50", name: "Акт приемки и / или расхождений" };
pub const DOC_COMPLEX_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_51", name: "Акт приемки-сдачи товара / работ / услуг / оборудования" };
pub const DOC_RETURN_ACTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_52", name: "Акты возврата" };
pub const DOC_WITHDRAWAL_ACTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_53", name: "Акты вывода" };
pub const DOC_TRANSFER_ACTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_54", name: "Акты передачи" };
pub const DOC_RESPONSIBILITY_ACTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_55", name: "Акты разграничения балансовой принадлежности и эксплуатационной ответственности" };
pub const DOC_SHAREHOLDER_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_56", name: "Акционерное соглашение" };
pub const DOC_EXCHANGE_FORWARD: BTGDocLimit = BTGDocLimit { code: "LIM_01_57", name: "Биржевой форвардный договор" };
pub const DOC_BULLETIN: BTGDocLimit = BTGDocLimit { code: "LIM_01_58", name: "Бюллетень (опросный лист)" };
pub const DOC_PENSION_STATEMENT_PERSONAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_59", name: "Ведомость личных пенсионных взносов по договору" };
pub const DOC_PENSION_STATEMENT_SOLIDARY: BTGDocLimit = BTGDocLimit { code: "LIM_01_60", name: "Ведомость солидарных пенсионных взносов по договору" };
pub const DOC_PENSION_STATEMENT_TARGET: BTGDocLimit = BTGDocLimit { code: "LIM_01_61", name: "Ведомость целевых пенсионных взносов по договору" };
pub const DOC_OTC_FORWARD: BTGDocLimit = BTGDocLimit { code: "LIM_01_62", name: "Внебиржевой форвардный договор" };
pub const DOC_PENSION_EXTRACT_ACC: BTGDocLimit = BTGDocLimit { code: "LIM_01_63", name: "Выписка из именного пенсионного счета" };
pub const DOC_GUARANTEE_EXTRACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_64", name: "Выписка из реестра независимых гарантий" };
pub const DOC_PENSION_EXTRACT_CONTR: BTGDocLimit = BTGDocLimit { code: "LIM_01_65", name: "Выписка о поступивших и отраженных пенисонных взносах" };
pub const DOC_SAVINGS_EXTRACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_66", name: "Выписка о поступивших и отраженных сберегательных взносах по договору" };
pub const DOC_BANK_EXTRACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_67", name: "Выписка по банковскому счету" };
pub const DOC_CONTR_EXTRACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_68", name: "Выписка со счета по договору" };
pub const DOC_REGISTRY_INFO: BTGDocLimit = BTGDocLimit { code: "LIM_01_69", name: "Выписка / письмо / информация об операциях по счетам в реестре" };
pub const DOC_FIN_MARKET_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_70", name: "Генеральное соглашение о срочных сделках на финансовых рынках" };

pub const DOC_GENERAL_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_71", name: "Генеральное соглашение" };
pub const DOC_BILLING_DATA: BTGDocLimit = BTGDocLimit { code: "LIM_01_72", name: "Данные биллинга" };
pub const DOC_CONFORMITY_DECL: BTGDocLimit = BTGDocLimit { code: "LIM_01_73", name: "Декларация о соответствии" };
pub const DOC_DEPOSITORY_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_74", name: "Депозитарный договор (договор о счете депо)" };
pub const DOC_DERIVATIVES: BTGDocLimit = BTGDocLimit { code: "LIM_01_75", name: "Деривативы (ПФИ)" };
pub const DOC_INVOICE_DETAILS: BTGDocLimit = BTGDocLimit { code: "LIM_01_76", name: "Детализация счета" };
pub const DOC_DEFECT_STATEMENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_77", name: "Дефектная ведомость" };
pub const DOC_OUTSOURCING: BTGDocLimit = BTGDocLimit { code: "LIM_01_78", name: "Договор аутсорсинга" };
pub const DOC_OUTSTAFFING: BTGDocLimit = BTGDocLimit { code: "LIM_01_79", name: "Договор аутстаффинга (о предоставлении труда работников (персонала)" };
pub const DOC_SHARE_BUILDING: BTGDocLimit = BTGDocLimit { code: "LIM_01_80", name: "Договор долевого участия (ДДУ) в строительстве / инвестирование строительства" };
pub const DOC_CONTRACT_CREDIT: BTGDocLimit = BTGDocLimit { code: "LIM_01_81", name: "Договор контрактного кредитования" };
pub const DOC_OPERATIONAL_CREDIT: BTGDocLimit = BTGDocLimit { code: "LIM_01_82", name: "Договор кредитования текущей деятельности" };
pub const DOC_CREDIT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_83", name: "Договор кредитования" };
pub const DOC_INT_FACTORING: BTGDocLimit = BTGDocLimit { code: "LIM_01_84", name: "Договор международного факторинга" };
pub const DOC_PERSONAL_DATA_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_85", name: "Договор на обработку персональных данных" };
pub const DOC_PAWNSHOP_CREDIT: BTGDocLimit = BTGDocLimit { code: "LIM_01_86", name: "Договор на предоставление ломбардного кредита" };
pub const DOC_PAYMENT_SYSTEM_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_87", name: "Договор на участие в платежной системе" };
pub const DOC_TUITION_REFUND_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_88", name: "Договор о возмещении затрат работодателя на обучение" };
pub const DOC_PROMISSORY_ISSUE: BTGDocLimit = BTGDocLimit { code: "LIM_01_89", name: "Договор о выдаче векселя" };
pub const DOC_EARLY_PAYMENT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_90", name: "Договор о досрочной выплате (дисконтировании)" };
pub const DOC_INTERBANK_CREDIT: BTGDocLimit = BTGDocLimit { code: "LIM_01_91", name: "Договор о межбанковском кредите" };
pub const DOC_LIABILITY_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_92", name: "Договор о полной материальной ответственности" };
pub const DOC_MARGIN_PAYMENT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_93", name: "Договор о порядке уплаты плавающих маржевых сумм" };
pub const DOC_INCOME_REFUND_CREDIT: BTGDocLimit = BTGDocLimit { code: "LIM_01_94", name: "Договор о предоставлении возмещения недополученных доходов по кредитам физических лиц" };
pub const DOC_POST_FINANCING_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_95", name: "Договор о предоставлении постфинансирования" };
pub const DOC_STANDBY_CREDIT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_96", name: "Договор о резервном аккредитиве" };
pub const DOC_OBLIGATION_SECURITY: BTGDocLimit = BTGDocLimit { code: "LIM_01_97", name: "Договор об обеспечении исполнения обязательств" };
pub const DOC_CBR_EDO_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_98", name: "Договор об обмене электронными документами через расчетную сеть Центрального банка Российской Федерации" };
pub const DOC_CBR_PAYMENT_EDO: BTGDocLimit = BTGDocLimit { code: "LIM_01_99", name: "Договор об обмене электронными сообщениями при переводе денежных средств в рамках платежной системы Центрального банка Российской Федерации" };
pub const DOC_PARTICIPANT_RIGHTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_100", name: "Договор об осуществлении прав участников" };
pub const DOC_CBR_CREDIT_OPERATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_101", name: "Договор об участии в операциях по предоставлению и погашению кредитов Банка России, обеспеченных ценными бумагами или правами требования по кредитному договору" };
pub const DOC_SHARE_TRANSFER_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_102", name: "Договор приобретения и/или отчуждения акций хозяйственного общества" };
pub const DOC_PROJECT_FINANCING: BTGDocLimit = BTGDocLimit { code: "LIM_01_103", name: "Договор проектного финансирования" };

pub const DOC_MANAGEMENT_CO_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_104", name: "Договор с управляющей компанией, за исключением договоров доверительного управления" };
pub const DOC_SYNDICATED_CREDIT: BTGDocLimit = BTGDocLimit { code: "LIM_01_105", name: "Договор синдицированного кредита" };
pub const DOC_MORTGAGE_INSURANCE: BTGDocLimit = BTGDocLimit { code: "LIM_01_106", name: "Договор страхования в составе комбинированного (комплексного) договора ипотечного страхования" };
pub const DOC_COMPLEX_INSURANCE: BTGDocLimit = BTGDocLimit { code: "LIM_01_107", name: "Договор страхования в составе комбинированного (комплексного) договора страхования" };
pub const DOC_COMMERCIAL_CREDIT_INS: BTGDocLimit = BTGDocLimit { code: "LIM_01_108", name: "Договор страхования коммерческого (торгового) кредита" };
pub const DOC_CREDIT_ISSUE_ADDENDUM: BTGDocLimit = BTGDocLimit { code: "LIM_01_109", name: "Договор (контракт, соглашение), дополнение и изменение к нему, оформляемые в связи с выдачей" };
pub const DOC_GROUP_INSURANCE: BTGDocLimit = BTGDocLimit { code: "LIM_01_110", name: "Договор коллективного страхования" };
pub const DOC_PROMO_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_111", name: "Договор на проведение совместной акции/стимулирующего мероприятия" };
pub const DOC_CEO_POWERS_TRANSFER: BTGDocLimit = BTGDocLimit { code: "LIM_01_112", name: "Договор о передаче полномочий единоличного исполнительного органа ООО управляющей организации" };
pub const DOC_CA_ASSIGNMENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_113", name: "Договор поручения Удостоверяющего Центра" };
pub const DOC_REGISTRAR_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_114", name: "Договор с профессиональными участниками рынка ценных бумаг, имеющими лицензию на осуществление деятельности по ведению реестра (с регистраторами)" };
pub const DOC_LOYALTY_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_115", name: "Договор участия в программе лояльности" };
pub const DOC_CREDIT_LETTER_OPERATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_116", name: "Договор, заключаемый в связи с проведением аккредитивной и гарантийной операции" };
pub const DOC_PENSION_TRANSFER_NPF: BTGDocLimit = BTGDocLimit { code: "LIM_01_117", name: "Документы в другой негосударственный пенсионный фонд в связи с передачей средств пенсионных накоплений" };
pub const DOC_SECURITIES_MARKET_PROF: BTGDocLimit = BTGDocLimit { code: "LIM_01_118", name: "Документы в рамках осуществления профессиональной деятельности на рынке ценных бумаг" };
pub const DOC_AML_CFT: BTGDocLimit = BTGDocLimit { code: "LIM_01_119", name: "Документы в рамках ПОД / ФТ" };
pub const DOC_PENSION_PAYMENTS_SUCCESSORS: BTGDocLimit = BTGDocLimit { code: "LIM_01_120", name: "Документы выплат правопреемникам вкладчиков и (или) участников по договору" };
pub const DOC_TOKEN_ISSUE: BTGDocLimit = BTGDocLimit { code: "LIM_01_121", name: "Документы на выдачу ТОКЕНА" };
pub const DOC_NPF_TRANSFER_VALUE: BTGDocLimit = BTGDocLimit { code: "LIM_01_122", name: "Документы на перевод в другие негосударственные пенсионные фонды обязательств и выкупной суммы по договору" };
pub const DOC_NPF_TRANSFER_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_123", name: "Документы на перевод обязательств в другие негосударственные пенсионные фонды по договору" };
pub const DOC_REDEMPTION_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_124", name: "Документы по осуществлению выплат выкупных сумм по договору" };
pub const DOC_NPF_PENSION_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_125", name: "Документы по осуществлению выплат негосударственных пенсий по договору" };
pub const DOC_AGR_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_126", name: "Документы по осуществлению выплат по договору" };
pub const DOC_SUCCESSOR_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_127", name: "Документы по осуществлению выплат правопреемникам" };
pub const DOC_CONTR_SUCCESSOR_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_128", name: "Документы по осуществлению выплат правопреемникам вкладчиков и (или) участников по договору" };
pub const DOC_PENSION_INDEXATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_129", name: "Документы по осуществлению выплат / индексаций негосударственных пенсий по договору" };
pub const DOC_UNRECEIVED_PENSION_INDEX: BTGDocLimit = BTGDocLimit { code: "LIM_01_130", name: "Документы по осуществлению выплат / индексаций недополученных пенсий по договору" };
pub const DOC_PENSION_SAVINGS_CORR: BTGDocLimit = BTGDocLimit { code: "LIM_01_131", name: "Документы по осуществлению выплат / корректировке размера выплат за счет средств пенсионных накоплений застрахованным лицам по договору" };
pub const DOC_ONLY_PENSION_INDEXATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_132", name: "Документы по осуществлению индексации негосударственных пенсий по договору" };
pub const DOC_RESTRUCTURING_PHYSICAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_133", name: "Документы по реструктуризации задолженности физического лица" };
pub const DOC_FATCA_CRS: BTGDocLimit = BTGDocLimit { code: "LIM_01_134", name: "Документы по самосертификации FATCA / CRS" };
pub const DOC_SECURITIES_EARLY_REDEMPTION: BTGDocLimit = BTGDocLimit { code: "LIM_01_135", name: "Документы, касающиеся досрочного погашения ценных бумаг" };


pub const DOC_BUSINESS_OPS: BTGDocLimit = BTGDocLimit { code: "LIM_01_136", name: "Документы, необходимые для обеспечения хозяйственной деятельности" };
pub const DOC_BRANCH_OPS: BTGDocLimit = BTGDocLimit { code: "LIM_01_137", name: "Документы, обеспечивающие деятельность филиала (представительства)" };
pub const DOC_PENSION_PERS_ACCOUNT: BTGDocLimit = BTGDocLimit { code: "LIM_01_138", name: "Документы, по вопросам индивидуального персонифицированного учета в системе обязательного пенсионного страхования" };
pub const DOC_SRO_SECURITIES: BTGDocLimit = BTGDocLimit { code: "LIM_01_139", name: "Документы, предоставляемые в СРО в отношении деятельности профессионального участника рынка ценных бумаг" };
pub const DOC_AGR_STATUS_CHANGE: BTGDocLimit = BTGDocLimit { code: "LIM_01_140", name: "Документы, связанные c возникновением, изменением и / или прекращением правоотношений по заключенному договору" };
pub const DOC_MORTGAGE_OPERATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_141", name: "Документы, связанные с выдачей / получением закладной, ее изменением, заменой и аннулированием" };
pub const DOC_AGR_CONCLUSION: BTGDocLimit = BTGDocLimit { code: "LIM_01_142", name: "Документы, связанные с заключением договора" };
pub const DOC_LAND_REDISTRIBUTION: BTGDocLimit = BTGDocLimit { code: "LIM_01_143", name: "Документы, связанные с исполнением обязательств по соглашению о перераспределении земельного участка" };
pub const DOC_SHAREHOLDER_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_144", name: "Документы, связанные с исполнением / неисполнением обязательств по акционерному соглашению" };
pub const DOC_GENERAL_AGR_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_145", name: "Документы, связанные с исполнением / неисполнением обязательств по генеральному соглашению" };
pub const DOC_DDU_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_146", name: "Документы, связанные с исполнением / неисполнением обязательств по ДДУ в строительстве/инвестирование строительства" };
pub const DOC_STOCK_TRANSFER_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_147", name: "Документы, связанные с исполнением / неисполнением обязательств по договору приобретения и/или отчуждения акций хозяйственного общества" };
pub const DOC_AGR_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_148", name: "Документы, связанные с исполнением / неисполнением обязательств по договору (соглашению)" };
pub const DOC_CREDIT_LETTER_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_149", name: "Документы, связанные с исполнением / неисполнением обязательств по договору (соглашению) об открытии аккредитива" };
pub const DOC_PROMISSORY_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_150", name: "Документы, связанные с исполнением / неисполнением обязательств по договору о выдаче векселя" };
pub const DOC_SECURITY_AGR_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_151", name: "Документы, связанные с исполнением / неисполнением обязательств по обеспечительному договору" };
pub const DOC_PRELIM_AGR_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_152", name: "Документы, связанные с исполнением / неисполнением обязательств по предварительному договору (соглашению)" };
pub const DOC_OWN_POSITION_DEALS: BTGDocLimit = BTGDocLimit { code: "LIM_01_153", name: "Документы, связанные с исполнением / неисполнением обязательств по сделкам по собственной позиции" };
pub const DOC_CREDITOR_CLAIMS_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_154", name: "Документы, связанные с исполнением / неисполнением, прекращением обязательств по соглашениям о порядке удовлетворения требований кредиторов" };
pub const DOC_SECURITIES_CONVERSION: BTGDocLimit = BTGDocLimit { code: "LIM_01_155", name: "Документы, связанные с конвертацией ценных бумаг" };
pub const DOC_INSURANCE_REJECTION: BTGDocLimit = BTGDocLimit { code: "LIM_01_156", name: "Документы, связанные с отказом в страховой выплате" };
pub const DOC_TRANSFER_WITHDRAWAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_157", name: "Документы, связанные с передачей / выводом" };
pub const DOC_SHAREHOLDER_INFO_DISCLOSURE: BTGDocLimit = BTGDocLimit { code: "LIM_01_161", name: "Документы, связанные с раскрытием (предоставлением) информации об акционерах (участниках) общества, о лицах под контролем либо значительным влиянием которых находится кредитная организация, в том числе в виде форм отчетности" };
pub const DOC_DISPOSAL_AND_USE: BTGDocLimit = BTGDocLimit { code: "LIM_01_162", name: "Документы, связанные с распоряжением и / или использованием" };
pub const DOC_INSURANCE_SALVAGE_DISPOSAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_163", name: "Документы, связанные с реализацией годных остатков имущества, поступившего в собственность страховой организации после осуществления страховой выплаты" };
pub const DOC_CREDIT_LETTER_EVENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_158", name: "Документы, связанные с проведением аккредитивных мероприятий" };
pub const DOC_GUARANTEE_OPERATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_159", name: "Документы, связанные с проведением гарантийных операций" };
pub const DOC_INFO_DISCLOSURE: BTGDocLimit = BTGDocLimit { code: "LIM_01_160", name: "Документы, связанные с раскрытием (предоставлением) информации" };
pub const DOC_ADDITIONAL_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_164", name: "Дополнительное соглашение к договору" };
pub const DOC_KM4_JOURNAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_165", name: "Журнал кассира-операциониста (Форма № КМ-4)" };
pub const DOC_KM8_JOURNAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_166", name: "Журнал учета вызовов технических специалистов и регистрации выполненных работ (Форма № КМ-8)" };
pub const DOC_MORTGAGE_NOTE: BTGDocLimit = BTGDocLimit { code: "LIM_01_167", name: "Закладная (отметка на закладной)" };


pub const DOC_REQ_MEMBER_RIGHTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_169", name: "Запрос вкладчику - юридическому лицу на предоставление справки о наличии права Участника" };
pub const DOC_REQ_NPF_IDENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_170", name: "Запрос на предоставление документов, необходимых для идентификации Вкладчиков и Участников негосударственного пенсионного фонда, на обновление данных (в том числе персональных данных) Вкладчиков и Участников" };
pub const DOC_REQ_LOSS_SETTLE_DOCS: BTGDocLimit = BTGDocLimit { code: "LIM_01_171", name: "Запрос о предоставлении дополнительных документов и информации, в том числе связанных с изменением сроков урегулирования убытков" };
pub const DOC_REQ_SUBROGATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_172", name: "Запрос и иные документы о взыскании денежных средств в порядке суброгации и регресса" };
pub const DOC_REQ_REGISTRY_EXTRACTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_173", name: "Запрос на предоставление выписок по счетам в реестре" };
pub const DOC_REQ_CONTR_INFO: BTGDocLimit = BTGDocLimit { code: "LIM_01_174", name: "Запрос на предоставление документов и информации, адресованные вкладчикам и (или) участникам / застрахованным лицам и правопреемникам по договору" };
pub const DOC_CLAIM_INCOME_REFUND: BTGDocLimit = BTGDocLimit { code: "LIM_01_175", name: "Заявка на возмещение недополученных доходов" };
pub const DOC_CLAIM_FRAMEWORK_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_176", name: "Заявка на основании либо во исполнение рамочного договора" };
pub const DOC_FACILITY_ACCESS_CLAIM: BTGDocLimit = BTGDocLimit { code: "LIM_01_177", name: "Заявки на допуск в сооружения" };
pub const DOC_APPLICATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_178", name: "Заявление" };
pub const DOC_APP_BENEFICIARY_MOD: BTGDocLimit = BTGDocLimit { code: "LIM_01_179", name: "Заявление бенефициара об изменении условий аккредитива" };
pub const DOC_APP_CONFIRM_VALIDITY: BTGDocLimit = BTGDocLimit { code: "LIM_01_180", name: "Заявление на подтверждение действительности" };
pub const DOC_APP_STATUS_INFO: BTGDocLimit = BTGDocLimit { code: "LIM_01_181", name: "Заявление на получение информации о статусе" };
pub const DOC_APP_SUSPEND_RESUME: BTGDocLimit = BTGDocLimit { code: "LIM_01_182", name: "Заявление на приостановление (возобновление) действия" };
pub const DOC_APP_CREATE_ISSUE: BTGDocLimit = BTGDocLimit { code: "LIM_01_183", name: "Заявление на создание и выдачу" };
pub const DOC_APP_OFFSET_CLAIMS: BTGDocLimit = BTGDocLimit { code: "LIM_01_184", name: "Заявление о зачете взаимных требований" };
pub const DOC_APP_TERMINATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_185", name: "Заявление о прекращении действия (аннулирования)" };
pub const DOC_APP_CHANGE_PHONE_NUM: BTGDocLimit = BTGDocLimit { code: "LIM_01_186", name: "Заявление о смене абонентского номера" };
pub const DOC_APP_CHANGE_OWNER_NUM: BTGDocLimit = BTGDocLimit { code: "LIM_01_187", name: "Заявление о смене владельца номера" };
pub const DOC_APP_CHANGE_TARIFF: BTGDocLimit = BTGDocLimit { code: "LIM_01_188", name: "Заявление о смене тарифного плана" };
pub const DOC_APP_END_SERVICE: BTGDocLimit = BTGDocLimit { code: "LIM_01_189", name: "Заявление об окончании абонентского обслуживания" };
pub const DOC_APP_BENEFICIARY_WAIVER: BTGDocLimit = BTGDocLimit { code: "LIM_01_190", name: "Заявление об отказе бенефициара от своих прав по аккредитиву" };
pub const DOC_APP_UPDATE_REQUISITES: BTGDocLimit = BTGDocLimit { code: "LIM_01_191", name: "Заявление об уточнении платежных реквизитов" };
pub const DOC_APP_CANCEL_CREDIT_LETTER: BTGDocLimit = BTGDocLimit { code: "LIM_01_192", name: "Заявление плательщика / приказодателя об отмене или отзыва аккредитива" };
pub const DOC_APP_SIM_BLOCK_UNBLOCK: BTGDocLimit = BTGDocLimit { code: "LIM_01_193", name: "Заявление на блокировку / разблокировку номеров, замену Sim-карт" };
pub const DOC_APP_CLOSE_SERVICE_APPS: BTGDocLimit = BTGDocLimit { code: "LIM_01_194", name: "Заявление на закрытие приложений обслуживания" };
pub const DOC_APP_CHANGE_BILL_DELIVERY: BTGDocLimit = BTGDocLimit { code: "LIM_01_195", name: "Заявление на изменение адреса и / или метода доставки счетов" };
pub const DOC_APP_CHANGE_CONTACTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_196", name: "Заявление на изменение контактных данных" };


pub const DOC_APP_ACQUIRING_CANCEL: BTGDocLimit = BTGDocLimit { code: "LIM_01_197", name: "Заявление на отмену операций по договору эквайринга" };
pub const DOC_APP_ACCOUNT_TRANSFER: BTGDocLimit = BTGDocLimit { code: "LIM_01_198", name: "Заявление на перенос средств между лицевыми счетами" };
pub const DOC_APP_SERVICE_TOGGLE: BTGDocLimit = BTGDocLimit { code: "LIM_01_199", name: "Заявление на подключение / отключение услуг, в том числе, услуг роуминга" };
pub const DOC_APP_REGISTRY_OPEN: BTGDocLimit = BTGDocLimit { code: "LIM_01_200", name: "Заявление / анкета для открытия счетов в реестрах" };
pub const DOC_APP_SECURITIES_REDEMPTION: BTGDocLimit = BTGDocLimit { code: "LIM_01_201", name: "Заявление / уведомление о погашении / выкупе / досрочном погашении ценных бумаг" };
pub const DOC_RECADV_NOTICE: BTGDocLimit = BTGDocLimit { code: "LIM_01_202", name: "Извещение о приемке товара на склад (RECADV)" };
pub const DOC_COLLECTION_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_203", name: "Инкассовое поручение" };
pub const DOC_INFO_LETTER: BTGDocLimit = BTGDocLimit { code: "LIM_01_204", name: "Информационное письмо" };
pub const DOC_NAV_INFO: BTGDocLimit = BTGDocLimit { code: "LIM_01_205", name: "Информация о стоимости чистых активов" };
pub const DOC_PRIVATE_GUARANTEE_REGISTRY: BTGDocLimit = BTGDocLimit { code: "LIM_01_206", name: "Информация, подлежащая включению в закрытый реестр независимых гарантий" };
pub const DOC_GUARANTEE_REGISTRY_INFO: BTGDocLimit = BTGDocLimit { code: "LIM_01_207", name: "Информация, подлежащая включению в реестр независимых гарантий" };
pub const DOC_CASH_BOOK: BTGDocLimit = BTGDocLimit { code: "LIM_01_208", name: "Кассовая книга" };
pub const DOC_INSURANCE_KEY_INFO: BTGDocLimit = BTGDocLimit { code: "LIM_01_209", name: "Ключевой информационный документ об условиях добровольного страхования" };
pub const DOC_FX_CONVERSION_DEAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_210", name: "Конверсионная сделка" };
pub const DOC_PAPER_CREDIT_LETTERS: BTGDocLimit = BTGDocLimit { code: "LIM_01_211", name: "Копии аккредитивов, оформленных на бумажном носителе" };
pub const DOC_CORPORATE_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_212", name: "Корпоративный договор" };
pub const DOC_CORRESPONDENCE: BTGDocLimit = BTGDocLimit { code: "LIM_01_213", name: "Корреспонденция" };
pub const DOC_CREDIT_AGR_LETTER_COVER: BTGDocLimit = BTGDocLimit { code: "LIM_01_214", name: "Кредитный договор (соглашение) для покрытия по аккредитивам" };
pub const DOC_LICENSE_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_215", name: "Лицензионный договор" };
pub const DOC_EXCLUSIVE_LICENSE: BTGDocLimit = BTGDocLimit { code: "LIM_01_216", name: "Лицензионный договор на условиях исключительной лицензии" };
pub const DOC_SIMPLE_LICENSE: BTGDocLimit = BTGDocLimit { code: "LIM_01_217", name: "Лицензионный договор на условиях простой (неисключительной) лицензии" };
pub const DOC_MEMORANDUM: BTGDocLimit = BTGDocLimit { code: "LIM_01_220", name: "Меморандум" };
pub const DOC_SETTLEMENT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_221", name: "Мировое соглашение" };
pub const DOC_CMR_WAYBILL: BTGDocLimit = BTGDocLimit { code: "LIM_01_222", name: "Накладная международного образца CMR (International Waybill)" };


pub const DOC_WAYBILL_M15: BTGDocLimit = BTGDocLimit { code: "LIM_01_223", name: "Накладная по форме № М-15" };
pub const DOC_WAYBILL_TORG13: BTGDocLimit = BTGDocLimit { code: "LIM_01_224", name: "Накладная по форме № ТОРГ-13" };
pub const DOC_GUARANTEE_LETTER_CREDIT: BTGDocLimit = BTGDocLimit { code: "LIM_01_225", name: "Независимые гарантии / банковские гарантии / резервные аккредитивы, изменения к ним, включая увеличение сумм и продление (пролонгацию) сроков действия" };
pub const DOC_ANNOUNCEMENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_226", name: "Объявление" };
pub const DOC_OPTION_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_227", name: "Опционный договор" };
pub const DOC_REQ_RESPONSE_LETTER: BTGDocLimit = BTGDocLimit { code: "LIM_01_228", name: "Ответ на запрос / письмо" };
pub const DOC_INSIDER_OPS_NOTICE: BTGDocLimit = BTGDocLimit { code: "LIM_01_229", name: "Ответ / уведомление / письмо о совершенных операциях в рамках предоставления сведений юридическим лицам, включивших лицо в списки инсайдеров" };
pub const DOC_BENEFICIARY_REJECTION: BTGDocLimit = BTGDocLimit { code: "LIM_01_230", name: "Отказ в удовлетворении требований бенефициаров" };
pub const DOC_BENEFICIARY_WAIVER_RIGHTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_231", name: "Отказ бенефициара от своих прав" };
pub const DOC_REPORT: BTGDocLimit = BTGDocLimit { code: "LIM_01_232", name: "Отчет" };
pub const DOC_AGENT_REPORT: BTGDocLimit = BTGDocLimit { code: "LIM_01_233", name: "Отчет агента" };
pub const DOC_COMMISSIONER_REPORT: BTGDocLimit = BTGDocLimit { code: "LIM_01_234", name: "Отчет комиссионера" };
pub const DOC_ASSIGNMENT_REPORT: BTGDocLimit = BTGDocLimit { code: "LIM_01_236", name: "Отчет об исполнении поручения" };
pub const DOC_RECEIPT: BTGDocLimit = BTGDocLimit { code: "LIM_01_371", name: "Чек" };
pub const DOC_FUTURES_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_370", name: "Фьючерсный договор" };
pub const DOC_ASSOCIATION_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_369", name: "Учредительный договор" };
pub const DOC_NPF_LEGAL_REPORT: BTGDocLimit = BTGDocLimit { code: "LIM_01_235", name: "Отчет о назначенных негосударственных пенсиях закрытых договорах негосударственного пенсионного обеспечения Вкладчику – юридическому лицу" };
pub const DOC_OFFER_ISSUE_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_237", name: "Оферта / поручение на выдачу" };
pub const DOC_PROPERTY_PASSPORT: BTGDocLimit = BTGDocLimit { code: "LIM_01_238", name: "Паспорт безопасности объекта недвижимости" };
pub const DOC_TRANSFER_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_239", name: "Передаточное распоряжение" };
pub const DOC_TRANSFER_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_240", name: "Передаточный акт" };
pub const DOC_LETTER: BTGDocLimit = BTGDocLimit { code: "LIM_01_241", name: "Письмо" };
pub const DOC_LETTER_CREDIT_ACCEPT: BTGDocLimit = BTGDocLimit { code: "LIM_01_242", name: "Письмо (акцепт), подтверждающее исполнение банком заявлений на открытие / увеличение / пролонгацию аккредитивов с применением ставки комиссионного вознаграждения" };
pub const DOC_ADVISING_LETTER: BTGDocLimit = BTGDocLimit { code: "LIM_01_243", name: "Письмо об авизовании" };
pub const DOC_LETTER_CREDIT_ADVISING: BTGDocLimit = BTGDocLimit { code: "LIM_01_244", name: "Письмо об авизовании аккредитива и изменений к нему" };
pub const DOC_LETTER_INQUIRY: BTGDocLimit = BTGDocLimit { code: "LIM_01_245", name: "Письмо, запрос" };


pub const DOC_NOTICE_CONTR_FEEDBACK: BTGDocLimit = BTGDocLimit { code: "LIM_01_368", name: "Уведомление, справка, ответ на обращение и претензию, адресованные вкладчиками и (или) участниками / застрахованными лицами и правопреемникам по договору" };
pub const DOC_NOTICE_CREDIT_LETTER_OPEN: BTGDocLimit = BTGDocLimit { code: "LIM_01_367", name: "Уведомление об открытии (выставлении) аккредитива" };
pub const DOC_NOTICE_ISSUE_FACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_366", name: "Уведомление о факте выдачи" };
pub const DOC_NOTICE_CREDIT_LETTER_DISCREPANCY: BTGDocLimit = BTGDocLimit { code: "LIM_01_365", name: "Уведомление о согласии / отказе принять документы с расхождениями, представленные по аккредитиву" };
pub const DOC_NOTICE_DEBT_CALC: BTGDocLimit = BTGDocLimit { code: "LIM_01_364", name: "Уведомление о расчете задолженности (неустойки, штрафа, пени)" };
pub const DOC_NOTICE_BENEFICIARY_MOD_FEEDBACK: BTGDocLimit = BTGDocLimit { code: "LIM_01_363", name: "Уведомление бенефициара о согласии / несогласии с изменением условий" };
pub const DOC_NOTICE_RECIPIENT_LETTER_MOD: BTGDocLimit = BTGDocLimit { code: "LIM_01_362", name: "Уведомление получателя средств / бенефициара о согласии / несогласии с изменением условий аккредитива" };
pub const DOC_NOTICE_TARIFF_CHANGE: BTGDocLimit = BTGDocLimit { code: "LIM_01_361", name: "Уведомление о смене тарифа" };
pub const DOC_NOTICE_BANK_REQUISITES_CHANGE: BTGDocLimit = BTGDocLimit { code: "LIM_01_360", name: "Уведомление о смене банковских реквизитов" };
pub const DOC_NOTICE_PENSION_TERMINATION_LEGAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_359", name: "Уведомление о расторжении индивидуального пенсионного договора с дополнительными пенсионными основаниями Вкладчиком - юридическим лицом" };
pub const DOC_NOTICE_TRUST_MGMT_INTENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_358", name: "Уведомление о намерении передать имущество в доверительное управление / вывести имущество из доверительного управления" };
pub const DOC_NOTICE: BTGDocLimit = BTGDocLimit { code: "LIM_01_357", name: "Уведомление" };
pub const DOC_CLAIM_PAYMENT_WITH_DOCS: BTGDocLimit = BTGDocLimit { code: "LIM_01_356", name: "Требование об уплате денежной суммы (требования платежа), и документы, прилагаемые к требованию об уплате денежной суммы (требованию платежа)" };
pub const DOC_REQ_SHAREHOLDER_MEETING_LIST: BTGDocLimit = BTGDocLimit { code: "LIM_01_355", name: "Требование о предоставлении выписки из списка лиц, имеющих право на участие в общем собрании акционеров" };
pub const DOC_CLAIM_AGR_BREACH_REFUND: BTGDocLimit = BTGDocLimit { code: "LIM_01_354", name: "Требование о возмещении сумм / расходов / убытков / штрафов (пеней, неустоек) за нарушение условий договоров" };
pub const DOC_CLAIM_PRINCIPAL_EXPENSE_REFUND: BTGDocLimit = BTGDocLimit { code: "LIM_01_353", name: "Требование в адрес принципалов (приказодателей) об обязанности возмещения расходов / убытков / штрафов (пеней, неустоек)" };
pub const DOC_CLAIM_PAYMENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_352", name: "Требование об уплате денежной суммы" };
pub const DOC_SUBLICENSE_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_351", name: "Сублицензионный договор" };
pub const DOC_CERT_OR_EXTRACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_350", name: "Справка / выписка" };
pub const DOC_CERT_REDEMPTION_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_349", name: "Справка о произведенных выплатах выкупных сумм по договору" };
pub const DOC_CERT_AGR_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_348", name: "Справка о назначенных и произведенных выплатах по договору" };
pub const DOC_CERT_NPF_PENSION_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_347", name: "Справка о назначенных и выпаченных негосударственных пенсиях" };
pub const DOC_CERT_REDEMPTION_VALUES: BTGDocLimit = BTGDocLimit { code: "LIM_01_346", name: "Справка о выкупных суммах" };
pub const DOC_KS3_CERT: BTGDocLimit = BTGDocLimit { code: "LIM_01_345", name: "Справка по форме № КС-3" };
pub const DOC_CERT_TURNOVER: BTGDocLimit = BTGDocLimit { code: "LIM_01_344", name: "Справка о товарообороте" };
pub const DOC_CERT_MEMBER_CONTRIBUTIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_343", name: "Справка о перечисленных взносах для Участников" };
pub const DOC_CERT_2NDFL_MEMBERS: BTGDocLimit = BTGDocLimit { code: "LIM_01_342", name: "Справка о доходах 2-НДФЛ для участников" };
pub const DOC_CERTIFICATE: BTGDocLimit = BTGDocLimit { code: "LIM_01_341", name: "Справка" };
pub const DOC_LIST_CONTR_SIGNATORIES: BTGDocLimit = BTGDocLimit { code: "LIM_01_340", name: "Список подписантов Вкладчика" };
pub const DOC_LIST_SIGNATORIES: BTGDocLimit = BTGDocLimit { code: "LIM_01_339", name: "Список подписантов" };
pub const DOC_LIST_PENSION_TERMINATION_IND: BTGDocLimit = BTGDocLimit { code: "LIM_01_338", name: "Список по расторжению индивидуальных пенсионных договоров с дополнительными пенсионными основаниями по договор" };
pub const DOC_LIST_RELATED_LEGAL_ENTITIES: BTGDocLimit = BTGDocLimit { code: "LIM_01_337", name: "Список связанных юридических лиц" };
pub const DOC_LIST_REGISTRY_BANK_CARDS: BTGDocLimit = BTGDocLimit { code: "LIM_01_336", name: "Список и реестр на выпуск банковских карт" };
pub const DOC_LIST_AFFILIATED_PERSONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_335", name: "Список аффилированных лиц эмитента, включая списки аффилированных лиц, принадлежащих к той группе, к которой принадлежит эмитент, в том числе в виде форм отчетности" };


pub const DOC_LIST_NPF_AFFILIATED: BTGDocLimit = BTGDocLimit { code: "LIM_01_334", name: "Список аффилированных лиц негосударственного пенсионного фонда" };
pub const DOC_EDO_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_333", name: "Соглашение об электронном документообороте" };
pub const DOC_CLAIM_SETTLEMENT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_332", name: "Соглашение об урегулировании претензий, убытков, неустоек" };
pub const DOC_FIN_ASSISTANCE_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_331", name: "Соглашение о финансовой помощи" };
pub const DOC_PARTNERSHIP_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_330", name: "Соглашение о партнерстве (сотрудничестве)" };
pub const DOC_CYBERSECURITY_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_329", name: "Соглашение о кибербезопасности" };
pub const DOC_WARRANTIES_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_328", name: "Соглашение о гарантиях и заверениях" };
pub const DOC_MOU_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_327", name: "Соглашение о взаимопонимании" };
pub const DOC_BENEFICIARY_BUILD_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_326", name: "Соглашение с бенефициарами о порядке взаиодействия при расчетах бенефициара с участниками долевого строительства" };
pub const DOC_DIGITAL_SIGN_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_324", name: "Соглашение об электронной подписи" };
pub const DOC_DIGITAL_INTERACT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_325", name: "Соглашение об электронном взаимодействии" };
pub const DOC_INSURANCE_SETTLEMENT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_323", name: "Соглашение об урегулировании страховых случаев" };
pub const DOC_FX_CASH_DEAL_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_322", name: "Соглашение об общих условиях проведения кассовых конверсионных сделок" };
pub const DOC_DEBT_RESTRUCTURE_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_321", name: "Соглашение о реструктуризации и / или урегулировании проблемной задолженности" };
pub const DOC_OPTION_GRANT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_320", name: "Соглашение о предоставлении опциона на заключение договора" };
pub const DOC_CREDITOR_SATISFACTION_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_319", name: "Соглашение о порядке удовлетворения требований кредиторов" };
pub const DOC_INTEREST_ON_BALANCE_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_318", name: "Соглашение о порядке начисления процентов на остаток денежных средств на счете" };
pub const DOC_NEGOTIATION_PROC_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_317", name: "Соглашение о порядке ведения переговоров" };
pub const DOC_PROCEDURE_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_316", name: "Соглашение о порядке" };
pub const DOC_REDISTRIBUTION_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_315", name: "Соглашение о перераспределении" };
pub const DOC_INTENT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_314", name: "Соглашение о намерении" };
pub const DOC_NDA_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_313", name: "Соглашение о конфиденциальности" };
pub const DOC_COMPETITION_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_312", name: "Соглашение о конкуренции" };
pub const DOC_AGREEMENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_310", name: "Соглашение" };
pub const DOC_LOSS_COMPENSATION_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_311", name: "Соглашение о возмещении потерь" };
pub const DOC_CONSENT_DEFERRED_PAYMENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_309", name: "Согласие на предоставление приказодателю отсрочки возмещения платежа, произведенного банком по аккредитиву за счет собственных средств" };
pub const DOC_BENEFICIARY_MOD_CONSENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_308", name: "Согласие / отказы бенефициара на изменения" };
pub const DOC_CONSENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_307", name: "Согласие" };
pub const DOC_INV19_STATEMENT: BTGDocLimit = BTGDocLimit { code: "LIM_01_306", name: "Сличительная ведомость по форме ИНВ-19" };
pub const DOC_SCANNED_GUARANTEES: BTGDocLimit = BTGDocLimit { code: "LIM_01_305", name: "Сканированные копии независимых гарантий / банковских гарантий / резервные аккредитивы, изменения к ним на бумажном носителе" };
pub const DOC_CONFORMITY_CERT: BTGDocLimit = BTGDocLimit { code: "LIM_01_304", name: "Сертификат соответствия" };
pub const DOC_OWN_POSITION_TRADE: BTGDocLimit = BTGDocLimit { code: "LIM_01_303", name: "Сделка по собственной позиции (в трейдинге)" };
pub const DOC_BANKING_OPS_DEAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_302", name: "Сделка по вопросу осуществления банковской деятельности" };
pub const DOC_DEAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_301", name: "Сделка" };
pub const DOC_SWAP_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_300", name: "Своп-договор" };



pub const DOC_RECLAMATION_ACT: BTGDocLimit = BTGDocLimit { code: "LIM_01_299", name: "Рекламационный акт" };
pub const DOC_FLIGHT_REGISTRY: BTGDocLimit = BTGDocLimit { code: "LIM_01_298", name: "Реестр рейсов" };
pub const DOC_INSURED_PERSONS_REGISTRY: BTGDocLimit = BTGDocLimit { code: "LIM_01_297", name: "Реестр застрахованных лиц" };
pub const DOC_REGISTRY: BTGDocLimit = BTGDocLimit { code: "LIM_01_296", name: "Реестр" };
pub const DOC_ASSET_VALUE_CALC: BTGDocLimit = BTGDocLimit { code: "LIM_01_295", name: "Расчет стоимости активов" };
pub const DOC_SETTLEMENT_DOCS: BTGDocLimit = BTGDocLimit { code: "LIM_01_294", name: "Расчетные (платежные) документы" };
pub const DOC_INSURANCE_LOSS_CALC: BTGDocLimit = BTGDocLimit { code: "LIM_01_293", name: "Расчет суммы ущерба, причиненного страховым случаем" };
pub const DOC_CASH_OUT_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_292", name: "Расходный кассовый ордер" };
pub const DOC_SECURITIES_PLEDGE_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_291", name: "Распоряжение о внесении изменений в сведениях о заложенных ценных бумагах и условиях залога" };
pub const DOC_TRANSFER_ORDER_BANK: BTGDocLimit = BTGDocLimit { code: "LIM_01_290", name: "Распоряжение на перевод" };
pub const DOC_PENSION_TERMINATION_ORDER_LEGAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_289", name: "Распоряжение о расторжении индивидуального пенсионного договоров с дополнительными пенсионными основаниями" };
pub const DOC_TERMINATION_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_288", name: "Распоряжение о прекращении" };
pub const DOC_INSURANCE_PAYMENT_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_287", name: "Распоряжение на страховую выплату" };
pub const DOC_SHAREHOLDER_REGISTRY_INFO_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_286", name: "Распоряжение на предоставление информации из реестра акционеров" };
pub const DOC_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_285", name: "Распоряжение" };
pub const DOC_RECEIPTS_AND_NOTICES_EXEC: BTGDocLimit = BTGDocLimit { code: "LIM_01_284", name: "Расписки, уведомления и иные документы в рамках исполнения" };
pub const DOC_RECEIPTS_NOTICES_MEMBER_SUCCESSORS: BTGDocLimit = BTGDocLimit { code: "LIM_01_283", name: "Расписки и уведомления о регистрации (получении) заявлений правопреемников Участников" };
pub const DOC_RECEIPTS_NOTICES_PAYMENTS: BTGDocLimit = BTGDocLimit { code: "LIM_01_282", name: "Расписки и уведомления о регистрации (получении) заявлений о периодических и единовременных выплатах" };
pub const DOC_RECEIPTS_NOTICES_LUMP_SUM_CONTRIB: BTGDocLimit = BTGDocLimit { code: "LIM_01_281", name: "Расписки и уведомления о регистрации (получении) заявлений о единовременном взносе" };
pub const DOC_RECEIPTS_NOTICES_REDEMPTION_VALUES: BTGDocLimit = BTGDocLimit { code: "LIM_01_280", name: "Расписки и уведомления о регистрации (получении) заявлений о выплатах выкупных сумм" };
pub const DOC_RECEIPTS_NOTICES_REGISTRATION_MEMBER: BTGDocLimit = BTGDocLimit { code: "LIM_01_279", name: "Расписки и уведомления о регистрации заявлений (и прилагаемых к ним документов) Участников и Вкладчиков в рамках" };
pub const DOC_FRAMEWORK_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_278", name: "Рамочный договор" };
pub const DOC_REIMBURSEMENT_OBLIGATIONS: BTGDocLimit = BTGDocLimit { code: "LIM_01_277", name: "Рамбурсные обязательства" };
pub const DOC_MEETING_PROTOCOL_LEGAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_276", name: "Протокол (решение), оформляющее решение, принятое на общих собраниях участников (акционеров) коммерческих организаций" };
pub const DOC_PROTOCOL_NPF_EARLY_TERMINATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_275", name: "Протокол о досрочном расторжении договора негосударственного пенисонного обеспечения с полнительными пенсионными основаниями" };
pub const DOC_CASH_IN_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_274", name: "Приходный кассовый ордер" };
pub const DOC_AGR_MOD_ANNEX_SPEC: BTGDocLimit = BTGDocLimit { code: "LIM_01_273", name: "Приложение, спецификация, дополнительное соглашение об изменении, расторжении и прекращении договора" };
pub const DOC_CLAIM_PRETENZIA: BTGDocLimit = BTGDocLimit { code: "LIM_01_272", name: "Претензия" };
pub const DOC_PRELIMINARY_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_271", name: "Предварительный договор" };
pub const DOC_OPS_ORDER_DOCS: BTGDocLimit = BTGDocLimit { code: "LIM_01_270", name: "Поручение / распоряжение / иные документы с целью совершения операций" };
pub const DOC_PLEDGE_PRIORITY_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_269", name: "Поручение / распоряжение об установлении / изменении старшинства залогов" };
pub const DOC_REGISTRATION_SUSPEND_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_268", name: "Поручение / распоряжение о приостановлении (прекращении) государственной регистрации" };



pub const DOC_ORDER_TERMINATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_267", name: "Поручение / распоряжение о прекращении" };
pub const DOC_ORDER_DERIVATIVE_DEAL: BTGDocLimit = BTGDocLimit { code: "LIM_01_266", name: "Поручение на срочную сделку" };
pub const DOC_ORDER_SECURITIES_WITHDRAW: BTGDocLimit = BTGDocLimit { code: "LIM_01_265", name: "Поручение на снятие ценных бумаг с хранения и/или учета" };
pub const DOC_ORDER_DEAL_GENERIC: BTGDocLimit = BTGDocLimit { code: "LIM_01_264", name: "Поручение на сделку" };
pub const DOC_ORDER_RESERVATION: BTGDocLimit = BTGDocLimit { code: "LIM_01_263", name: "Поручение на резервирование" };
pub const DOC_ORDER_PLEDGE_ASSIGNMENT_REG: BTGDocLimit = BTGDocLimit { code: "LIM_01_262", name: "Поручение на регистрацию уступки права залога" };
pub const DOC_ORDER_SUBSEQUENT_PLEDGE_REG: BTGDocLimit = BTGDocLimit { code: "LIM_01_261", name: "Поручение на регистрацию последующего залога" };
pub const DOC_ORDER_SECURITIES_PLEDGE_REG: BTGDocLimit = BTGDocLimit { code: "LIM_01_260", name: "Поручение на регистрацию залога ценных бумаг" };
pub const DOC_ORDER_SECURITIES_DEPOSIT: BTGDocLimit = BTGDocLimit { code: "LIM_01_259", name: "Поручение на прием ценных бумаг на хранение и/или учет" };
pub const DOC_ORDER_GET_REPORTS_INFO: BTGDocLimit = BTGDocLimit { code: "LIM_01_258", name: "Поручение на получение справок, отчетов, выписок" };
pub const DOC_ORDER_PLEDGED_SECURITIES_TRANSFER: BTGDocLimit = BTGDocLimit { code: "LIM_01_257", name: "Поручение на перевод заложенных ценных бумаг" };
pub const DOC_ORDER_TRANSFER: BTGDocLimit = BTGDocLimit { code: "LIM_01_256", name: "Поручение на перевод" };
pub const DOC_ORDER_DEPO_OPEN_WITH_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_255", name: "Поручение на открытие счета депо (с правом заключать депозитарный договор)" };
pub const DOC_ORDER_DEPO_OPEN_WITHOUT_AGR: BTGDocLimit = BTGDocLimit { code: "LIM_01_254", name: "Поручение на открытие счета депо (без права заключать депозитарный договор)" };
pub const DOC_ORDER_DEPO_SECTION_OPEN: BTGDocLimit = BTGDocLimit { code: "LIM_01_253", name: "Поручение на открытие раздела счета депо" };
pub const DOC_ORDER_PROFILE_DATA_CHANGE: BTGDocLimit = BTGDocLimit { code: "LIM_01_252", name: "Поручение на изменение анкетных данных" };
pub const DOC_ORDER_DEPO_CLOSE: BTGDocLimit = BTGDocLimit { code: "LIM_01_251", name: "Поручение на закрытие счета депо" };
pub const DOC_ORDER_RETURN: BTGDocLimit = BTGDocLimit { code: "LIM_01_250", name: "Поручение на возврат" };
pub const DOC_ORDER_GENERIC: BTGDocLimit = BTGDocLimit { code: "LIM_01_249", name: "Поручение" };
pub const DOC_CREDIT_LETTER_CONFIRM_UNCOVERED: BTGDocLimit = BTGDocLimit { code: "LIM_01_248", name: "Подтверждение аккредитива, по которому банком-эмитентом не предоставлено исполняющему банку денежное покрытие в сумме подтверждения аккредитива" };
pub const DOC_PAYMENT_ORDER: BTGDocLimit = BTGDocLimit { code: "LIM_01_247", name: "Платежное поручение" };
pub const DOC_PAYROLL_SHEET: BTGDocLimit = BTGDocLimit { code: "LIM_01_246", name: "Платежная ведомость" };