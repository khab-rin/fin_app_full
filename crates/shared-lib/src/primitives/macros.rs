macro_rules! impl_partial_rules {
    ($name:ident) => {
        impl std::cmp::PartialEq for $name {
            fn eq(&self, r: &$name) -> bool {
                self.data == r.data
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.as_ref().to_string().hash(state);
            }
        }

        impl std::cmp::PartialEq<$name> for &$name {
            fn eq(&self, r: &$name) -> bool {
                self.data == r.data
            }
        }

        impl std::cmp::PartialEq<&$name> for $name {
            fn eq(&self, r: &&$name) -> bool {
                self.data == r.data
            }
        }

        impl std::cmp::Eq for $name {}
    };
}


macro_rules! math_rules {
    ($trait:ident, $method:ident, $op:tt, $ltype:ident, $rtype:ident => $res_type:ident) => {
        impl<'a, 'b> std::ops::$trait<&'b $rtype> for &'a $ltype {
            type Output = $res_type;
            fn $method(self, r: &'b $rtype) -> Self::Output {
                $res_type::from_raw(**self $op **r)
            }
        }

        impl<'b> std::ops::$trait<&'b $rtype> for $ltype {
            type Output = $res_type;
            fn $method(self, r: &'b $rtype) -> Self::Output {
                &self $op r
            }
        }

        impl<'a> std::ops::$trait<$rtype> for &'a $ltype {
            type Output = $res_type;
            fn $method(self, r: $rtype) -> Self::Output {
                self $op &r
            }
        }

        impl std::ops::$trait<$rtype> for $ltype {
            type Output = $res_type;
            fn $method(self, r: $rtype) -> Self::Output {
                &self $op &r
            }
        }
    };
}

macro_rules! impl_is_zero {
    ($name:ident) => {
        impl $name {
            pub fn is_zero(&self) -> bool {
                self.data.as_ref() == "0"
            }
        }
    };
}

