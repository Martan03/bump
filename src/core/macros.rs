macro_rules! generate_struct {
    (
        $access:vis $name:ident {
            $( $var_access:vis $var_name:ident: $var_type:ty
                $(=> $var_default:expr)?,
            )*
        }
    ) => {
        place! {
            #[derive(Clone, Serialize, Deserialize)]
            $access struct $name {
                $(
                    $(
                        __ignore__($var_default)
                        #[serde(default = __string__(
                            $name "::default_" $var_name
                        ))]
                    )?
                    $var_access $var_name: $var_type,
                )*
                changed: bool,
            }

            impl $name {
                $(
                    pub fn __ident__("get_" $var_name)(&self) -> &$var_type {
                        &self.$var_name
                    }

                    pub fn __ident__("set_" $var_name)(&mut self, value: $var_type) {
                        self.changed = true;
                        self.$var_name = value;
                    }

                    $(
                        fn __ident__("default_" $var_name)() -> $var_type {
                            $var_default
                        }
                    )?
                )*
            }
        }
    }
}
