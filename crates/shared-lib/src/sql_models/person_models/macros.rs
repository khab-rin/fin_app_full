macro_rules! make_doc_type {
    ($enum_name: ident, $type_name: ident, $number: expr) => {
        #[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, sqlx::Type)]
        pub enum $enum_name {
            #[serde(rename = $number)]
            #[default]
            $type_name
        }
    };
}


macro_rules! make_document  {
    ($doc_name: ident, $doc_code: ident, $doc_num: ident) => {
        #[derive(Serialize, Deserialize, Clone, Debug, sqlx::Type)]
        pub struct $doc_name {
            pub doc_code: $doc_code,

            pub doc_ser_num: $doc_num,

            pub doc_date: Date,

            pub issued_by: Option<String1_4000>,

            pub issued_code: Option<String7_7>,
            
            pub exp_date: Option<Date>
        }
    };
}