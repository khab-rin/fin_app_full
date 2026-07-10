use crate::service::mchd::home_mchd_power::HomePowerInfo;


pub const H110: HomePowerInfo = HomePowerInfo { code: "H110", name: "Просматривать сведения о персонале" };
pub const H111: HomePowerInfo = HomePowerInfo { code: "H111", name: "Редактировать сведения о персонале (полный доступ)" };

pub const H210: HomePowerInfo = HomePowerInfo { code: "H210", name: "Просматривать выписки и банковские счета" };
pub const H211: HomePowerInfo = HomePowerInfo { code: "H211", name: "Распоряжаться банковскими счетами и подписывать платежи (полный доступ)" };

pub const H310: HomePowerInfo = HomePowerInfo { code: "H310", name: "Просматривать первичные документы и налоговую отчетность" };
pub const H311: HomePowerInfo = HomePowerInfo { code: "H311", name: "Подписывать и отправлять отчетность, акты, УПД и счета-фактуры (полный доступ)" };

pub const H410: HomePowerInfo = HomePowerInfo { code: "H410", name: "Просматривать складские остатки и накладные" };
pub const H411: HomePowerInfo = HomePowerInfo { code: "H411", name: "Управлять складом, подписывать товарно-транспортные накладные и акты приемки (полный доступ)" };

pub const H510: HomePowerInfo = HomePowerInfo { code: "H510", name: "Просматривать договоры и юридические документы" };
pub const H511: HomePowerInfo = HomePowerInfo { code: "H511", name: "Заключать, изменять, расторгать договоры и подписывать соглашения (полный доступ)" };

pub const H610: HomePowerInfo = HomePowerInfo { code: "H610", name: "Просматривать журнал событий безопасности и результаты проверок контрагентов" };
pub const H611: HomePowerInfo = HomePowerInfo { code: "H611", name: "Управлять уровнями риска, блокировать учетные записи и согласовывать доступы (полный доступ)" };

pub const H710: HomePowerInfo = HomePowerInfo { code: "H710", name: "Просматривать системные настройки и конфигурации" };
pub const H711: HomePowerInfo = HomePowerInfo { code: "H711", name: "Администрировать информационные системы, управлять интеграциями и базами данных (полный доступ)" };

pub const H810: HomePowerInfo = HomePowerInfo { code: "H810", name: "Просматривать базу контрагентов, лиды и воронку продаж" };
pub const H811: HomePowerInfo = HomePowerInfo { code: "H811", name: "Управлять сделками, выставлять счета на оплату и согласовывать скидки (полный доступ)" };

pub const H910: HomePowerInfo = HomePowerInfo { code: "H910", name: "Просматривать заявки на закупку и прайс-листы поставщиков" };
pub const H911: HomePowerInfo = HomePowerInfo { code: "H911", name: "Формировать заказы поставщикам, участвовать в тендерах и закупках (полный доступ)" };

pub const H1010: HomePowerInfo = HomePowerInfo { code: "H1010", name: "Просматривать маркетинговые отчеты и аналитику кампаний" };
pub const H1011: HomePowerInfo = HomePowerInfo { code: "H1011", name: "Управлять рекламными бюджетами, подрядчиками и маркетинговыми акциями (полный доступ)" };

pub const UNKNOWN: HomePowerInfo = HomePowerInfo { code: "Unknown", name: "Неизвестное полномиче" };