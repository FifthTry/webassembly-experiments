// // Import the WASI environment and additional WASI-enabled APIs
import { double } from "./env"

memory.grow(1);
const index = 0;
const value = 24;
store<u8>(index, value);

export function run(a: str, b: i32): i32 {
  // assert(a, jsonObj.toString());
  // let sum: i32 = double(a) + double(b); // type can be inferred, annotation can be omitted
  let valueAtIndexOne = load<u8>(1);
  return valueAtIndexOne;
  // return sum;
}


// import "wasi";
// import { Console } from "as-wasi/assembly";

// Console.log("content-type: text/plain");
// Console.log("");
// Console.log("Hello, World");