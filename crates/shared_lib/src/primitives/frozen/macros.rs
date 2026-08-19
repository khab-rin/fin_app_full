macro_rules! frozen_primitives {
    ($avail:vis $name:ident, $validator:expr, $data_type:ty, $label:literal, $formatter:expr) => {
        #[derive(Debug, Clone, Ord, PartialOrd, Serialize, Deserialize, ts_rs::TS)]
        #[serde(try_from = "String", into = "String")]
        #[ts(as = "String")] 
        pub struct $name {
            $avail data: $data_type
        }

        impl $name {
            pub(crate) const LABEL: & 'static str = $label;

            pub fn new(value: &str) -> Result<Self, Status> {
                $validator(value).map(|v| Self {data: v})
            }

            #[allow(dead_code)]
            pub(crate) fn label(&self) -> & 'static str {
                Self::LABEL
            }

            pub fn unchecked<T>(val: T) -> Self
            where T: Into<$data_type>
            {
                Self { data: val.into() }
            }

            pub fn beat_string(&self) -> String {
                $formatter(&self.data)
            }
        }

        impl std::str::FromStr for $name {
            type Err = Status;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl std::convert::TryFrom<String> for $name {
            type Error = Status;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(&value)
            }
        }

        impl std::convert::TryFrom<&str> for $name {
            type Error = Status;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }


        impl std::ops::Deref for $name {
            type Target = $data_type;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        impl std::convert::AsRef<$data_type> for $name {
            fn as_ref(&self) -> &$data_type {
                &self.data
            }
        }


        impl std::convert::From<$name> for String {
            fn from(value: $name) -> String {
                value.data.to_string()
            }
        }

        impl std::convert::From<&$name> for $name {
            fn from(value: &Self) -> Self {
                value.clone()
            } 
        }

        impl_partial_rules!($name);     

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.data)
            }
        }

        

        impl sqlx::Type<sqlx::Sqlite> for $name {
            fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
                <String as sqlx::Type<sqlx::Sqlite>>::type_info()
            }
        }
        
        
        impl sqlx::Type<sqlx::Postgres> for $name 
        where 
            $data_type: sqlx::Type<sqlx::Postgres> 
        {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <$data_type as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }


        impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                let s = ($formatter)(&self.data);
                
                <String as sqlx::Encode<'q, sqlx::Sqlite>>::encode(s, buf)
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for $name 
        where 
            $data_type: sqlx::Encode<'q, sqlx::Postgres>
        {
            fn encode_by_ref(
                &self,
                buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync>> {
                <$data_type as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&self.data, buf)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for $name {
            fn decode(
                value: <sqlx::Sqlite as sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
  
                let raw_str = <String as sqlx::Decode<'r, sqlx::Sqlite>>::decode(value)?;
    
                let validated = Self::new(&raw_str)
                    .map_err(|e| format!("Failed to decode {}: {}", stringify!($name), e))?;
                
                Ok(validated)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for $name {
            fn decode(
                value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
               
                let type_val = <$data_type as sqlx::Decode<'r, sqlx::Postgres>>::decode(value)?;
                
                Ok(Self{data: type_val})
            }
        }

        impl ParseFromStrMapValue for $name {
            fn parse_from_str_map_value(map_value: Option<&&str>) -> Result<Self, Status> {
                map_value.ok_or(Status::MappingError)?
                    .parse::<Self>()
            }
        }

        impl ::sqlx::postgres::PgHasArrayType for $name {
            fn array_type_info() -> ::sqlx::postgres::PgTypeInfo {
                // Мы берем информацию о типе массива от базового типа (uuid, String и т.д.)
                < $data_type as ::sqlx::postgres::PgHasArrayType >::array_type_info()
            }
        }
    };
}


pub trait RussEnumName {
    fn russian_enum_name(&self) -> String;
}

#[macro_export]
macro_rules! impl_as_str {
    ($en_name:ident, { $($en_key:ident => $str_key:expr),* $(,)? }) => {
        impl $en_name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$en_key => $str_key,)*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! make_enum_frozen {
    ($name:ident, { $($l_name:ident, $r_name:expr, { $($alias:expr),* $(,)? }),* $(,)? }) => {
        #[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, Hash, sqlx::Type, ts_rs::TS)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        #[sqlx(type_name = "text", rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum $name {
            #[default]
            $(
                #[serde(alias=$r_name)]
                $(
                    #[serde(alias=$alias)]
                )*
                #[ts(rename = $r_name)]
                $l_name,
            )*
        }

        $crate::impl_as_str! {$name, { $($l_name => $r_name),* }}

        impl std::str::FromStr for $name {
            type Err = $crate::err_models::implements::Status;

            fn from_str(val: &str) -> Result<Self, Self::Err> {
                match val {
                    $(
                        $r_name => Ok(Self::$l_name),
                        $(
                            $alias => Ok(Self::$l_name),
                        )*
                    )*
                    _ => Err($crate::err_models::implements::Status::ValidEnum)
                }
            }
        }

        impl $crate::primitives::frozen::macros::RussEnumName for $name {
            fn russian_enum_name(&self) -> String {
                match self {
                    $(Self::$l_name => $r_name.to_string(),)*
                }
            }
        }
    };
}

macro_rules! gen_str_validator {
    ($name:ident, $min:expr, $max:expr, $err:expr) => {
        pub(crate) fn $name(val: &str) -> Result<String, $crate::err_models::implements::Status> {
            let s = val.trim();
            let n = s.chars().count();
            if ($min..=$max).contains(&n) { return Ok(s.into());}
            Err($err)
        }
    };
}

macro_rules! gen_digit_validator {
    ($name:ident, $min:expr, $max:expr, $err:expr) => {
        pub(crate) fn $name(val: &str) -> Result<String, $crate::err_models::implements::Status> {
            let s = val.trim();
            let mut clean_s = String::with_capacity(s.len());
            let mut digit_count = 0;

            for c in s.chars() {
                if c.is_ascii_digit() {
                    clean_s.push(c);
                    digit_count += 1;
                } else if matches!(c, '-' | '.' | ',' | '\\' | '|' | '/' | '\'' | '`' | ' ' | '\u{A0}') {
                    continue;
                } else {
                    return Err($err);
                }
            }
            if ($min..=$max).contains(&digit_count) {
                return Ok(clean_s.into());
            }
            Err($err)
        }
    };
}

