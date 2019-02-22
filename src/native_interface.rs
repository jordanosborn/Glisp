#[macro_export]
macro_rules! c_function {
    ($return_type:ty, $function:ident) => {
        extern {
            pub fn $function() -> $return_type;
        }
    };
    ( $return_type:ty, $function:ident, $( $var:ident : $type:ty ),*) => {
        extern {
            pub fn $function($($var: $type),*) -> $return_type;
        }
    };
    ( $return_type:ty, $function:ident, $safe_function:ident, $( $var:ident : $type:ty ),*) => {
        extern {
            pub fn $function($($var: $type),*) -> $return_type;
        }
        pub fn $safe_function($($var),*) -> $return_type {
            unsafe {
                $function($($var),*)
            }
        }
    };
}
