macro_rules! gen_func_name {
    ($ident:ident) => {
        (stringify!($ident))
    };
}

macro_rules! generate_struct {
    (
        $access:vis $name:ident {
            $( $var_access:vis $var_name:ident: $var_type:ty, )*
        }
    ) => {
        $access struct $name {
            $( $var_access $var_name: $var_type, )*
        }

        impl $name {

        }
    }
}

pub fn test() {
    let name: String;
    let res = gen_func_name!(name);
}

generate_struct! {
    pub Test {
        pub name: String,
        count: usize,
    }
}
