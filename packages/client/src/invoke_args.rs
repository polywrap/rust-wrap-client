#[macro_export(local_inner_macros)]
macro_rules! invoke_args {
    // Hide distracting implementation details from the generated rustdoc.
    ($($json_syntax:tt)+) => {
      {
        &Some(msgpack!($($json_syntax)+));
      }
    };
}