macro_rules! define_group_builder {
    (
        enum $feature_ty:ident {
            $(
              $enum_variant:ident
            ),* $(,)?
        }

        struct $name:ident {
            $(
                $field_name:ident : $field_ty:ty, $builder_ty:ty, $error_ty:ty
            ),* $(,)?
        }
    ) => {
        paste::paste! {
            #[derive(Debug, derive_more::Display, PartialEq, Eq, Hash, Clone, Copy)]
            pub(in crate::track) enum $feature_ty {
                $($enum_variant),*
            }

            #[derive(getset::Getters)]
            #[getset(get = "pub")]
            pub struct $name {
                features: HashSet<$feature_ty>,
                $($field_name: $field_ty),*
            }

            #[derive(Default, Clone)]
            pub struct [<$name Builder>] {
                features: HashSet<$feature_ty>,
                $($field_name: $builder_ty),*
            }

            #[derive(Debug, thiserror::Error)]
            pub enum [<$name SubBuilderError>] {
                $(
                    #[error("{0}")]
                    [<$field_name:camel>](#[from] $error_ty),
                )*
            }

            pub type [<$name BuilderError>] = GroupBuilderError<[<$name SubBuilderError>]>;

            impl GroupBuilderBase for [<$name Builder>] {
                type Feature = $feature_ty;
                type Output = $name;
                type SubError = [<$name SubBuilderError>];
            }
        }
    };
}

pub(crate) use define_group_builder;
