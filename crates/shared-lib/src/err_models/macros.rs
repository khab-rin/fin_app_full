macro_rules! map_errors {
    ( $($in_type:ty => $status:expr),* $(,)? ) => {
        $(
            impl From<$in_type> for Status {
                fn from(_: $in_type) -> Self {
                    $status
                }
            }
        )*
    };
}