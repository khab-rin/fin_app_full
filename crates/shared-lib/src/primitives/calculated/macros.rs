macro_rules! calculated_primitives {
    ($avail:vis $name:ident, $data_type:ty, $frozen_name:ident, $label:literal) => {
        #[derive(Debug, Clone, Ord, PartialOrd, Serialize, Deserialize)]
        pub struct $name {
            $avail data : $data_type
        }

        impl $name {
            pub(crate) const LABEL: &'static str = $label;

            pub(crate) fn new() -> Self {
                Self { data: <$data_type>::default() }
            }

            pub(crate) fn from_raw(value: $data_type) -> Self {
                Self {data: value}
            }

            #[allow(dead_code)]
            pub(crate) fn label(&self) -> &'static str {
                Self::LABEL
            }
        }

        impl std::convert::TryFrom<$name> for $frozen_name {
            type Error = Status;
            fn try_from(value: $name) -> Result<$frozen_name, Self::Error> {
                let rounded = value.data.round_dp(2);
                $frozen_name::new(&rounded.to_string())
            }
        }

        impl std::convert::From<$frozen_name> for $name {
            fn from(value: $frozen_name) -> Self {
                Self::from_raw(*value)
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

        impl_partial_rules!($name);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", &self.data)
            }
        }
    };
}