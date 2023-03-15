function encodeUtf8mb3(str) {
  let result = "";
  const buf = Buffer.from(str);
  for (let i = 0; i < buf.length; i++) {
    const code = buf[i];
    if (code >= 0xf0) {
      const unicode =
        ((code & 0x07) << 18) |
        ((buf[i + 1] & 0x3f) << 12) |
        ((buf[i + 2] & 0x3f) << 6) |
        (buf[i + 3] & 0x3f);
      result += encodeUnicodemb3(unicode);
      i += 3;
    } else if (code >= 0xe0) {
      result += String.fromCodePoint(
        ((code & 0x0f) << 12) | ((buf[i + 1] & 0x3f) << 6) | (buf[i + 2] & 0x3f)
      );
      i += 2;
    } else if (code >= 0xc0) {
      result += String.fromCodePoint(
        ((code & 0x1f) << 6) | (buf[i + 1] & 0x3f)
      );
      i += 1;
    } else {
      result += String.fromCodePoint(buf[i]);
    }
  }

  return result;
}

function decodeUtf8mb3(str) {
  let result = "";
  for (let i = 0; i < str.length; i++) {
    const c = str[i];
    const nextChar = str[i + 1];
    if (isEncodeUtf8mb3(c, nextChar)) {
      const unicode = decodeUnicodemb3(c + nextChar);
      if(unicode > 0x10ffff){
        result += c;
      }else{
        result += String.fromCodePoint(unicode);
        i += 1;
      }
    } else {
      result += c;
    }
  }

  return result;
}

function encodeUnicodemb3(unicode) {
  const c11 = 0xee | (unicode & 0x01);
  const c12 = 0x80 | ((unicode >> 1) & 0x3f);
  const c13 = 0x80 | ((unicode >> 7) & 0x3f);
  const c21 = 0xee | ((unicode >> 13) & 0x01);
  const c22 = 0x80 | ((unicode >> 14) & 0x3f);
  const c23 = 0x80 | ((unicode >> 20) & 0x3f);
  const buf = Buffer.from([c11, c12, c13, c21, c22, c23]);
  return buf.toString("utf8");
}

function decodeUnicodemb3(char) {
  const buf = Buffer.from(char, "utf8");
  const c11 = buf[0];
  const c12 = buf[1];
  const c13 = buf[2];
  const c21 = buf[3];
  const c22 = buf[4];
  const c23 = buf[5];

  const unicode =
    (c11 & 0x01) |
    ((c12 & 0x3f) << 1) |
    ((c13 & 0x3f) << 7) |
    ((c21 & 0x01) << 13) |
    ((c22 & 0x3f) << 14) |
    ((c23 & 0x3f) << 20);

  return unicode;
}

function isEncodeUtf8mb3(char, nextChar) {
  const code = char ? char.codePointAt(0) : 0;
  const nextCode = nextChar ? nextChar.codePointAt(0) : 0;
  const charBuf = Buffer.from(char);
  return (
    (code & 0xe000) === 0xe000 &&
    (nextCode & 0xe000) === 0xe000 &&
    (charBuf[0] & 0xf0) !== 0xf0
  );
}

function includeEncodeUtf8mb3(str) {
  for (let i = 0; i < str.length; i++) {
    const c = str[i];
    const nextChar = str[i + 1];
    if (isEncodeUtf8mb3(c, nextChar)) {
      return true;
    }
  }
  return false;
}

function includeUtf8mb4(str) {
  const buf = Buffer.from(str);

  for (let i = 0; i < buf.length; i++) {
    const code = buf[i];
    if (code >= 0xf0) {
      return true;
    }
  }

  return false;
}

module.exports = {
  encodeUtf8mb3,
  decodeUtf8mb3,
  includeEncodeUtf8mb3,
  includeUtf8mb4,
};
