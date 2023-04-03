const { encode, decode } = require("utf8mb3");

let str = "😊".repeat(1000000);
console.time("encode");
decode(encode(str));
console.timeEnd("encode");
