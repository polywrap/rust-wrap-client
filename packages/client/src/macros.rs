#[macro_export]
macro_rules! invoke_args {
    ($($args:tt)+) => {
        polywrap_core::invoke::InvokeArgs::Msgpack(polywrap_msgpack::msgpack!($($args)+))
    };
}