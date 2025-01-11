macro_rules! define_node_enum {
    ($enum_name:ident, $($variant:ident),* $(,)?) => {
        #[derive(Debug, PartialEq, Eq,Clone)]
        pub enum $enum_name {
            $(
                $variant($variant),
            )*
        }

        impl Node for $enum_name {
            fn token_literal(&self) -> String {
                match self {
                    $(
                        $enum_name::$variant(s) => s.token_literal(),
                    )*
                }
            }

            fn to_string(&self) -> String {
                match self {
                    $(
                        $enum_name::$variant(s) => s.to_string(),
                    )*
                }
            }
        }

        $(
            impl From<$variant> for $enum_name {
                fn from(variant: $variant) -> $enum_name {
                    $enum_name::$variant(variant)
                }
            }
            impl TryFrom<$enum_name> for $variant {
                type Error = ();

                fn try_from(node: $enum_name) -> Result<$variant, Self::Error> {
                    match node {
                        $enum_name::$variant(s) => Ok(s),
                        _ => Err(()),
                    }
                }
            }
            impl TryFrom<&$enum_name> for $variant {
                type Error = ();

                fn try_from(node: &$enum_name) -> Result<$variant, Self::Error> {
                    match node {
                        $enum_name::$variant(s) => Ok(s.clone()),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
}
pub(crate) use define_node_enum;
