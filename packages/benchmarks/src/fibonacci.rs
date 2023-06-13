use polywrap_wasm_rs::BigInt;

pub fn fibonacci_loop(n: i32) -> Result<BigInt, String> {
    let mut a = BigInt::from(0);
    let mut b = BigInt::from(1);
    let result = match n {
        0 => BigInt::from(0),
        1 => BigInt::from(1),
        _ => {
            for _ in 0..n - 1 {
                let temp = &a + &b;
                a = b;
                b = temp;
            }
            b
        },
    };
    Ok(result)
}

pub fn fibonacci_recursive(n: i32) -> Result<BigInt, String> {
    Ok(_fibonacci(&BigInt::from(n)))
}

fn _fibonacci(n: &BigInt) -> BigInt {
    let zero = BigInt::from(0);
    let one = BigInt::from(1);
    if n == &zero {
        zero
    } else if n == &one {
        one
    } else {
        let n_1 = n - one;
        let n_2 = n - 2;
        _fibonacci(&n_1) + _fibonacci(&n_2)
    }
}