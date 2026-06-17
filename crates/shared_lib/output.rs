#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "text", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompStatus {
    #[default]
    #[serde(alias = "ACTIVE")]
    #[serde(alias = "120")]
    Active,
    #[serde(alias = "LIQUIDATING")]
    #[serde(alias = "121")]
    Liquidating,
    #[serde(alias = "LIQUIDATED")]
    #[serde(alias = "122")]
    Liquidated,
    #[serde(alias = "REORGANIZING")]
    #[serde(alias = "123")]
    Reorganizing,
    #[serde(alias = "BANKRUPT")]
    #[serde(alias = "124")]
    Bankrupt,
}
