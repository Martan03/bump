use serde_derive::{Deserialize, Serialize};
use std::concat;
use paste::paste;

macro_rules! generate_struct {
    (
        $access:vis $name:ident {
            $( $var_access:vis $var_name:ident: $var_type:ty
                $(=> $var_default:expr)?,
            )*
        }
    ) => {
        #[derive(Clone, Serialize, Deserialize)]
        $access struct $name {
            $($var_access $var_name: $var_type,)*
            changed: bool,
        }

        impl $name {
            $(
                paste! {
                    pub fn [<get_ $var_name>](&self) -> &$var_type {
                        &self.$var_name
                    }
                }

                paste! {
                    pub fn [<set_ $var_name>](&mut self, value: $var_type) {
                        self.changed = true;
                        self.$var_name = value;
                    }
                }

                $(paste! {
                    fn [<default_ $var_name>](&self) -> $var_type {
                        $var_default
                    }
                })?
            )*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $(
                        $var_name: Default::default(),
                    )*
                    changed: false,
                }
            }
        }
    }
}

fn test() -> String {
    let test = "test";
    concat!("default_", stringify!(test)).to_owned()
}

generate_struct! {
    pub Test {
        pub name: String => "This is a test".to_owned(),
        count: usize,
    }
}
