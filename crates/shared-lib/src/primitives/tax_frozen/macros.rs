macro_rules! tax_primitives {
    ($avail:vis $name:ident, $validator:expr, $default:expr, $label:literal) => {
        #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
        pub(crate) struct $name {
            $avail data : Decimal
        }

        impl $name {
            pub(crate) const LABEL: &'static str = $label;

            pub(crate) fn new(value: &str) -> Result<Self, Status> {
                $validator(value).map(|v| Self {data : v}) 
            }

            #[allow(dead_code)]
            pub(crate) fn label() -> &'static str {
                Self::LABEL
            }
        }

        impl std::default::Default for $name {
            fn default() -> Self {
                Self { data : *$default() }
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

        impl std::convert::AsRef<Decimal> for $name {
            fn as_ref(&self) -> &Decimal {
                &self.data
            }
        }

        impl std::ops::Deref for $name {
            type Target = Decimal;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        impl From<$name> for String {
            fn from(_: $name) -> Self {
                $name::LABEL.to_string()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", Self::LABEL)
            }
        }

    };
}