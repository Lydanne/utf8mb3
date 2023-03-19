const {
  encodeUtf8mb3,
  decodeUtf8mb3,
  includeEncodeUtf8mb3,
  includeUtf8mb4,
} = require(".");

// const str = "耀쀀😊";
// const str = "老！😊";
// const str = "😊";
// const str =
// "1234567890 1234567890,望家长多关注图图看看有没有什么问题的话都不用着急上火@@@哈哈哈看看健康快乐健康快乐健康快乐，童心不老！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！\\n1234567890 1234567890,望家长多关注图图看看有没有什么问题的话都不用着急a上火@@@哈哈哈看看健康快乐健康快乐健康快乐，童心不k😌😌😉😎😃😆😄😄✌️✌🏾";
// const str = "活泼开朗的你这个月在游戏活动中都能积极主动参加活动，活动中与同伴友好相处，积极主动帮助小伙伴。但是课堂上你不够专心听讲哦！写作业的时候喜欢和小伙伴聊天。老师希望你下个月能够改掉这个坏习惯哟。加油！爱你的陆老师，欧老师，周老师";
const str =
  "恶臭和蚕丝被罚款测分不开看不出魂儿完成ke并成为新区的话我想得到你就把我的比我低成本的吃不一班车比较成熟不吃肉的仅仅维持帮我写不完吃吧你会成为妞妞次我和妞仅存的你几点回血姐姐😇😄😌😁😇😃🙂😌😇😃😇😌😇😃😁🤦🏿😃😌😃🤣🤦🏿😙😃🤦🏿🤣😃😙😃🤦🏿😙😍😃🤦🏿🤣😙😇🤦🏿🤣😙😃🤦🏿😙😍😌😄🤣🤦🏿😙🤣😃🤦🏿😚😃🤦🏿🤣😙😋😚😚🙂😋🤦🏻‍♀️🤔🤣🤔🥰😋🤦🏻‍♀️😙😍";
// const str = "🤦🏿";
// const str = "😊";
// const str = "🀀";
// const str = "！！";

console.log(str, Buffer.from(str), str.length);

const utf8mb3 = encodeUtf8mb3(str);

console.log("encodeUtf8mb4", utf8mb3, Buffer.from(utf8mb3), utf8mb3.length);
console.log("decodeUtf8mb4", decodeUtf8mb3(utf8mb3)); // 重复decode结果不会变
console.log("repeat decodeUtf8mb4", decodeUtf8mb3(decodeUtf8mb3(utf8mb3))); // 重复decode结果不会变
console.log("includeEncodeUtf8mb3", includeEncodeUtf8mb3(utf8mb3));
console.log("includeEncodeUtf8mb3", includeEncodeUtf8mb3(str));
console.log("includeUtf8mb4", includeUtf8mb4(str));
console.log("includeUtf8mb4", includeUtf8mb4("测试"));
console.log("includeUtf8mb4", includeEncodeUtf8mb3("！！"));
