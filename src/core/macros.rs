#[macro_export]
macro_rules! generate_struct {
    (
        $(#$der:tt)*
        $access:vis $name:ident {
            $( $ref_access:vis $ref_name:ident: $ref_type:ty
                $(=> $ref_default:expr)?,
            )*;
            $( $var_access:vis $var_name:ident: $var_type:ty
                $(=> $var_default:expr)?,
            )*;
            $( $v_access:vis $v_name:ident: $v_type: ty, )*
        }
    ) => {
        place! {
            $(#$der)*
            $access struct $name {
                $(
                    $(
                        __ignore__($ref_default)
                        #[serde(default = __string__(
                            $name "::default_" $ref_name
                        ))]
                    )?
                    $ref_access $ref_name: $ref_type,
                )*
                $(
                    $(
                        __ignore__($var_default)
                        #[serde(default = __string__(
                            $name "::default_" $var_name
                        ))]
                    )?
                    $var_access $var_name: $var_type,
                )*
                $( $v_access $v_name: $v_type, )*
                changed: bool,
            }

            impl $name {
                $(
                    #[allow(unused)]
                    pub fn __ident__("get_" $ref_name)(&self) -> &$ref_type {
                        &self.$ref_name
                    }

                    #[allow(unused)]
                    pub fn __ident__("set_" $ref_name)(&mut self, value: $ref_type) {
                        self.changed = true;
                        self.$ref_name = value;
                    }

                    $(
                        fn __ident__("default_" $ref_name)() -> $ref_type {
                            $ref_default
                        }
                    )?
                )*
                $(
                    #[allow(unused)]
                    pub fn __ident__("get_" $var_name)(&self) -> $var_type {
                        self.$var_name
                    }

                    #[allow(unused)]
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
