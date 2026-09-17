macro_rules! calculated_primitives {
    ($avail:vis $name:ident, $frozen_name:ident, $label:literal) => {
        #[derive(Debug, Clone, Ord, PartialOrd, Serialize, Deserialize)]
        #[serde(into = "String")]
        pub struct $name {
            $avail data: rust_decimal::Decimal
        }

        impl $name {
            pub(crate) const LABEL: &'static str = $label;

            pub(crate) fn new() -> Self {
                Self { data: rust_decimal::Decimal::ZERO }
            }

            pub(crate) fn from_raw(value: rust_decimal::Decimal) -> Self {
                Self { data: value }
            }

            #[allow(dead_code)]
            pub(crate) fn label(&self) -> &'static str {
                Self::LABEL
            }
        }

        impl std::convert::From<$name> for String {
            fn from(value: $name) -> String {
                format!("{:.2}", value.data)
            }
        }

        impl std::convert::TryFrom<$name> for $frozen_name {
            type Error = Status;
            fn try_from(value: $name) -> Result<$frozen_name, Self::Error> {
                let rounded = value.data.round_dp(2);
                $frozen_name::new(&format!("{:.2}", rounded))
            }
        }

        impl std::convert::From<$frozen_name> for $name {
            fn from(value: $frozen_name) -> Self {
                Self::from_raw(*value)
            }
        }

        impl std::ops::Deref for $name {
            type Target = rust_decimal::Decimal;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        impl std::convert::AsRef<rust_decimal::Decimal> for $name {
            fn as_ref(&self) -> &rust_decimal::Decimal {
                &self.data
            }
        }

        impl_partial_rules!($name);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:.2}", self.data)
            }
        }
    };
}