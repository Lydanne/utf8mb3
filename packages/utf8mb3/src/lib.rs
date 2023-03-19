/// Encode a four-byte string into a three-byte string, and if it is less than four-byte string, it will remain unchanged.
/// ```
///     let str = "👨‍👩‍👧‍👦";
///     let result = utf8mb3::encode(str);
///     assert_eq!(result, "‍ﴨ‍ﳨ‍");
/// ```
pub fn encode(input_str: &str) -> String {
    let mut result = String::new();
    let chars = input_str.chars();
    for c in chars {
        if c.len_utf8() >= 4 {
            let unicode = c as u32;
            result += String::from_utf8(encode_unicode(unicode)).unwrap().as_str();
        } else {
            result += c.to_string().as_str();
        }
    }

    result
}

fn encode_unicode(unicode: u32) -> Vec<u8> {
    let c11 = (0xee | (unicode & 0x01)) as u8;
    let c12 = (0x80 | ((unicode >> 1) & 0x3f)) as u8;
    let c13 = (0x80 | ((unicode >> 7) & 0x3f)) as u8;
    let c21 = (0xee | ((unicode >> 13) & 0x01)) as u8;
    let c22 = (0x80 | ((unicode >> 14) & 0x3f)) as u8;
    let c23 = (0x80 | ((unicode >> 20) & 0x3f)) as u8;
    vec![c11, c12, c13, c21, c22, c23]
}

/// Decode the compiled string.
/// ```
///     let str = "‍ﴨ‍ﳨ‍";
///     let result = utf8mb3::decode(str);
///     assert_eq!(result, "👨‍👩‍👧‍👦");
/// ```
pub fn decode(input_str: &str) -> String {
    let mut result = String::new();
    let chars = input_str.chars().collect::<Vec<char>>();
    let mut i = 0;

    while i < chars.len() {
        let c = chars.get(i).unwrap();
        let c_utf8_buf = to_utf8_buf(c);

        let next_c = chars.get(i + 1).unwrap_or(&' ');
        let next_c_utf8_buf = to_utf8_buf(next_c);

        if is_encode_utf8mb3(&c_utf8_buf, &next_c_utf8_buf) {
            let unicode = decode_unicode(&c_utf8_buf, &next_c_utf8_buf);
            let decode_c = &char::from_u32(unicode);
            if let Some(c) = decode_c {
                result += c.to_string().as_str();
                i += 1;
            } else {
                result += c.to_string().as_str();
            }
        } else {
            result += c.to_string().as_str();
        }
        i += 1;
    }

    result
}

fn decode_unicode(char_buf: &[u8], next_char_buf: &[u8]) -> u32 {
    let c11 = char_buf[0] & 0x01;
    let c12 = char_buf[1] & 0x3f;
    let c13 = char_buf[2] & 0x3f;
    let c21 = next_char_buf[0] & 0x01;
    let c22 = next_char_buf[1] & 0x3f;
    let c23 = next_char_buf[2] & 0x3f;
    let unicode = (c11 as u32)
        | ((c12 as u32) << 1)
        | ((c13 as u32) << 7)
        | ((c21 as u32) << 13)
        | ((c22 as u32) << 14)
        | ((c23 as u32) << 20);
    unicode
}

pub fn is_encode_utf8mb3(char_buf: &[u8], next_char_buf: &[u8]) -> bool {
    (char_buf[0] & 0xEE) == 0xEE
        && (next_char_buf[0] & 0xEE) == 0xEE
        && next_char_buf[2] & 0x03 == 0
}

/// Determine whether the string contains encode character.
pub fn include_encode_utf8mb3(input_str: &str) -> bool {
    let chars = input_str.chars().collect::<Vec<char>>();
    let mut i = 0;

    while i < chars.len() {
        let c = chars.get(i).unwrap();
        let c_utf8_buf = to_utf8_buf(c);

        let next_c = chars.get(i + 1).unwrap_or(&' ');
        let next_c_utf8_buf = to_utf8_buf(next_c);

        if is_encode_utf8mb3(&c_utf8_buf, &next_c_utf8_buf) {
            return true;
        }
        i += 1;
    }

    false
}

/// Determine whether the string contains a four-byte character.
pub fn include_utf8mb4(input_str: &str) -> bool {
    let chars = input_str.chars();

    for c in chars {
        if c.len_utf8() >= 4 {
            return true;
        }
    }

    false
}

fn to_utf8_buf(c: &char) -> [u8; 4] {
    let mut buf = [0; 4];
    c.encode_utf8(&mut buf);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_encode_utf8mb3_t1() {
        let str = "😊";
        let result = encode(str);
        assert_eq!(result, "");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t2() {
        let str = "👨‍👩‍👧‍👦";
        let result = encode(str);
        assert_eq!(result, "‍ﴨ‍ﳨ‍");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t3() {
        let str = "🀀";
        let result = encode(str);
        assert_eq!(result, "");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t4() {
        let str = "🀀🀀🀀";
        let result = encode(str);
        assert_eq!(result, "");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t5() {
        let str = "🤦🏿";
        let result = encode(str);
        assert_eq!(result, "￧");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t6() {
        let str = "🤦🏿🤦🏿";
        let result = encode(str);
        assert_eq!(result, "￧￧");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t7() {
        let str = "耀쀀😊";
        let result = encode(str);
        assert_eq!(result, "耀쀀");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t8() {
        let str = "老！😊";
        let result = encode(str);
        assert_eq!(result, "老！");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t9() {
        let str = "1234567890 1234567890,望家长多关注图图看看有没有什么问题的话都不用着急上火@@@哈哈哈看看健康快乐健康快乐健康快乐，童心不老！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！\\n1234567890 1234567890,望家长多关注图图看看有没有什么问题的话都不用着急a上火@@@哈哈哈看看健康快乐健康快乐健康快乐，童心不k😌😌😉😎😃😆😄😄✌️✌🏾";
        let result = encode(str);
        assert_eq!(result, "1234567890 1234567890,望家长多关注图图看看有没有什么问题的话都不用着急上火@@@哈哈哈看看健康快乐健康快乐健康快乐，童心不老！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！！\\n1234567890 1234567890,望家长多关注图图看看有没有什么问题的话都不用着急a上火@@@哈哈哈看看健康快乐健康快乐健康快乐，童心不k✌️✌");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t10() {
        let str = "活泼开朗的你这个月在游戏活动中都能积极主动参加活动，活动中与同伴友好相处，积极主动帮助小伙伴。但是课堂上你不够专心听讲哦！写作业的时候喜欢和小伙伴聊天。老师希望你下个月能够改掉这个坏习惯哟。加油！爱你的陆老师，欧老师，周老师";
        let result = encode(str);
        assert_eq!(result, "活泼开朗的你这个月在游戏活动中都能积极主动参加活动，活动中与同伴友好相处，积极主动帮助小伙伴。但是课堂上你不够专心听讲哦！写作业的时候喜欢和小伙伴聊天。老师希望你下个月能够改掉这个坏习惯哟。加油！爱你的陆老师，欧老师，周老师");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t11() {
        let str = "恶臭和蚕丝被罚款测分不开看不出魂儿完成ke并成为新区的话我想得到你就把我的比我低成本的吃不一班车比较成熟不吃肉的仅仅维持帮我写不完吃吧你会成为妞妞次我和妞仅存的你几点回血姐姐😇😄😌😁😇😃🙂😌😇😃😇😌😇😃😁🤦🏿😃😌😃🤣🤦🏿😙😃🤦🏿🤣😃😙😃🤦🏿😙😍😃🤦🏿🤣😙😇🤦🏿🤣😙😃🤦🏿😙😍😌😄🤣🤦🏿😙🤣😃🤦🏿😚😃🤦🏿🤣😙😋😚😚🙂😋🤦🏻‍♀️🤔🤣🤔🥰😋🤦🏻‍♀️😙😍";
        let result = encode(str);
        assert_eq!(result, "恶臭和蚕丝被罚款测分不开看不出魂儿完成ke并成为新区的话我想得到你就把我的比我低成本的吃不一班车比较成熟不吃肉的仅仅维持帮我写不完吃吧你会成为妞妞次我和妞仅存的你几点回血姐姐￧￧￧￧￧￧￧￧￧￧ｧ‍♀️ｧ‍♀️");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_encode_utf8mb3_t12() {
        let str = "！！";
        let result = encode(str);
        assert_eq!(result, "！！");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_repeat_encode() {
        let str = "🤦🏿🤦🏿";
        let result = encode(&encode(str));
        assert_eq!(result, "￧￧");
        assert_eq!(decode(&result), str);
    }

    #[test]
    fn it_repeat_decode() {
        let str = "🤦🏿🤦🏿";
        let result = encode(str);
        assert_eq!(result, "￧￧");
        assert_eq!(decode(&decode(&result)), str);
    }

    #[test]
    fn it_include_encode_utf8mb3() {
        assert!(include_encode_utf8mb3("￧￧") == true); // encode "🤦🏿🤦🏿"
        assert!(include_encode_utf8mb3("你好呀") == false);
    }

    #[test]
    fn it_include_utf8mb4() {
        assert!(include_utf8mb4("🤦🏿🤦🏿") == true);
        assert!(include_utf8mb4("你好呀") == false);
    }
}
