#[macro_export(local_inner_macros)]
macro_rules! msgpack_to_value {
    // Hide distracting implementation details from the generated rustdoc.
    ($($msgpack:tt)+) => {
        msgpack_internal!($($msgpack)+)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! msgpack {
    // Hide distracting implementation details from the generated rustdoc.
    ($($msgpack:tt)+) => {
      {
        let mut buf = Vec::new();
        let value = msgpack_internal!($($msgpack)+);
        $crate::write_value(&mut buf, &value).unwrap();
        buf
      }
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! msgpack_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: msgpack_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        msgpack_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        msgpack_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        msgpack_internal!(@array [$($elems,)* msgpack_internal!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        msgpack_internal!(@array [$($elems,)* msgpack_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        msgpack_internal!(@array [$($elems,)* msgpack_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        msgpack_internal!(@array [$($elems,)* msgpack_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        msgpack_internal!(@array [$($elems,)* msgpack_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        msgpack_internal!(@array [$($elems,)* msgpack_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        msgpack_internal!(@array [$($elems,)* msgpack_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        msgpack_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        msgpack_unexpected!($unexpected)
    };

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: msgpack_internal!(@object $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@object $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        msgpack_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        msgpack_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is `null`.
    (@object $object:ident ($($key:tt)+) (: null $($rest:tt)*) $copy:tt) => {
        msgpack_internal!(@object $object [$($key)+] (msgpack_internal!(null)) $($rest)*);
    };

    // Next value is `true`.
    (@object $object:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        msgpack_internal!(@object $object [$($key)+] (msgpack_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@object $object:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        msgpack_internal!(@object $object [$($key)+] (msgpack_internal!(false)) $($rest)*);
    };

    // Next value is an array.
    (@object $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        msgpack_internal!(@object $object [$($key)+] (msgpack_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        msgpack_internal!(@object $object [$($key)+] (msgpack_internal!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        msgpack_internal!(@object $object [$($key)+] (msgpack_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        msgpack_internal!(@object $object [$($key)+] (msgpack_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        msgpack_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@object $object:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        msgpack_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@object $object:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        msgpack_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@object $object:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        msgpack_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        msgpack_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        msgpack_expect_expr_comma!($($unexpected)+);
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        msgpack_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: msgpack_internal!($($msgpack)+)
    //////////////////////////////////////////////////////////////////////////

    (null) => {
        $crate::Value::Nil
    };

    (true) => {
        $crate::Value::Boolean(true)
    };

    (false) => {
        $crate::Value::Bool(false)
    };

    ([]) => {
        $crate::Value::Array(msgpack_internal_vec![])
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Value::Array(msgpack_internal!(@array [] $($tt)+))
    };

    ({}) => {
      $crate::Value::Map($crate::RMPVObject::new().values)
    };

    ({ $($tt:tt)+ }) => {
        $crate::Value::Map({
          let mut object = $crate::RMPVObject::new();
          msgpack_internal!(@object object () ($($tt)+) ($($tt)+));
          object.values
        })
    };

    // Any Serialize type: numbers, strings, struct literals, variables etc.
    // Must be below every other rule.
    ($other:expr) => {
        $crate::Value::from($other)
    };
}

// The msgpack_internal macro above cannot invoke vec directly because it uses
// local_inner_macros. A vec invocation there would resolve to $crate::vec.
// Instead invoke vec here outside of local_inner_macros.
#[macro_export]
#[doc(hidden)]
macro_rules! msgpack_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! msgpack_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! msgpack_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}

#[cfg(test)]
mod tests {

    use rmpv::Value;

    #[test]
    fn msgpack() {
        let value = msgpack_to_value!({
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "serde",
                    "msgpack"
                ]
            }
        });

        let expected_map_tuples: Vec<(Value, Value)> = vec![
            (Value::from("code"), Value::from(200)),
            (Value::from("success"), Value::from(true)),
            (
                Value::from("payload"),
                Value::Map(vec![(
                    Value::from("features"),
                    Value::Array(vec![Value::from("serde"), Value::from("msgpack")]),
                )]),
            ),
        ];

        let expected = Value::Map(expected_map_tuples);

        assert_eq!(value, expected)
    }

    #[test]
    fn msgpack_to_vec() {
        let value = msgpack!({
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "serde",
                    "msgpack"
                ]
            }
        });

        assert_eq!(
            value,
            [
                131, 164, 99, 111, 100, 101, 204, 200, 167, 115, 117, 99, 99, 101, 115, 115, 195,
                167, 112, 97, 121, 108, 111, 97, 100, 129, 168, 102, 101, 97, 116, 117, 114, 101,
                115, 146, 165, 115, 101, 114, 100, 101, 167, 109, 115, 103, 112, 97, 99, 107
            ]
        )
    }
}
