#[macro_export]
macro_rules! hashmap {
    // matches one or more key-value pairs that end with a comma
    // this rule is necessary in order to match single key-value
    // pairs that end with a trailing comma
    ($($key:expr => $value:expr,)+) => { $crate::hashmap!($($key => $value),+) };
    // matches zero or more key-value pairs that are each 
    // separated by commas
    ($($key:expr => $value:expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map 
        }
    }
}
