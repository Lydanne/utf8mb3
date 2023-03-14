const {
  encodeUtf8mb3,
  decodeUtf8mb3,
  includeEncodeUtf8mb3,
  includeUtf8mb4,
} = require(".");

const str = "汉😊😊🛝🛝🛝汉";
// const str = "😊";

console.log(str, str.length);

const utf8mb3 = encodeUtf8mb3(str);

console.log("encodeUtf8mb4", utf8mb3, utf8mb3.length);
console.log("decodeUtf8mb4", decodeUtf8mb3(utf8mb3)); // 重复decode结果不会变
console.log("decodeUtf8mb4", decodeUtf8mb3(decodeUtf8mb3(utf8mb3))); // 重复decode结果不会变
console.log("includeEncodeUtf8mb3", includeEncodeUtf8mb3(utf8mb3));
console.log("includeEncodeUtf8mb3", includeEncodeUtf8mb3(str));
console.log("includeUtf8mb4", includeUtf8mb4(str));
console.log("includeUtf8mb4", includeUtf8mb4("测试"));
