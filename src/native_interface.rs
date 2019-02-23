#[macro_export]
macro_rules! c_function {
    ($return_type:ty, $function:ident) => {
        extern {
            pub fn $function() -> $return_type;
        }
    };
    ($return_type:ty, $function:ident, $( $var:ident : $type:ty ),*) => {
        extern {
            pub fn $function($($var: $type),*) -> $return_type;
        }
    };
}
