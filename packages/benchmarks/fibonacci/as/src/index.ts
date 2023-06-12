import {
  Args_fibonacci_loop,
  Args_fibonacci_recursive,
  ModuleBase
} from "./wrap";
import { BigInt } from "@polywrap/wasm-as"

export class Module extends ModuleBase {
  fibonacci_loop(args: Args_fibonacci_loop): BigInt {
    const n = args.n;
    let a = BigInt.ZERO;
    let b = BigInt.ONE;

    if (n == 0) return BigInt.ZERO;
    if (n == 1) return BigInt.ONE;
    for (let i = 0; i < n - 1; i++) {
      let temp = a + b;
      a = b;
      b = temp;
    }
    return b;
  }

  fibonacci_recursive(args: Args_fibonacci_recursive): BigInt {
    return this._fibonacci(BigInt.from(args.n));
  }

  private _fibonacci(n: BigInt): BigInt {
    if (n == BigInt.ZERO) return BigInt.ZERO;
    if (n == BigInt.ONE) return BigInt.ONE;

    let n_1 = n - BigInt.ONE;
    let n_2 = n - BigInt.from(2);
    return this._fibonacci(n_1) + this._fibonacci(n_2);
  }
}
