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

            pub doc_date: Option<Date>,

            pub issued_by: Option<String1_4000>,

            pub issued_code: Option<String7_7>,
            
            pub exp_date: Option<Date>
        }

        impl $doc_name {
            pub fn new(doc_code: $doc_code, doc_ser_num: $doc_num) -> Self {
                Self {
                    doc_code, 
                    doc_ser_num,
                    doc_date: None,
                    issued_by: None,
                    issued_code: None,
                    exp_date: None 
                }
            }

            pub fn with_doc_date(mut self, date: Date) -> Self {
                self.doc_date = Some(date);
                self
            }

            pub fn with_issued_by(mut self, by: String1_4000) -> Self {
                self.issued_by = Some(by);
                self
            }

            pub fn with_issued_code(mut self, by: String7_7) -> Self {
                self.issued_code = Some(by);
                self
            }

            pub fn with_exp_date(mut self, date: Date) -> Self {
                self.exp_date = Some(date);
                self
            }
        }
    };
}