pub mod wrap;
use wrap::module::{Module, ModuleTrait};
pub use wrap::*;
use polywrap_wasm_rs::BigInt;

impl ModuleTrait for Module {
    fn fibonacci_loop(args: ArgsFibonacciLoop) -> Result<BigInt, String> {
        let n = args.n;
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

    fn fibonacci_recursive(args: ArgsFibonacciRecursive) -> Result<BigInt, String> {
        let n = BigInt::from(args.n);
        Ok(_fibonacci(&n))
    }
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

