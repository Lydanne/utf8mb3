import { encode, decode } from "utf8mb3-wasm";

let str = "😊".repeat(1000000);
console.time("encode");
decode(encode(str));
console.timeEnd("encode");
