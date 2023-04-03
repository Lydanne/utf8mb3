const { encodeUtf8mb3, decodeUtf8mb3 } = require("./index");

let str = "😊".repeat(1000000);
console.time("encode");
decodeUtf8mb3(encodeUtf8mb3(str));
console.timeEnd("encode");
