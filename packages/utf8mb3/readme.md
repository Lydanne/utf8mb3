# utf8mb3 core

utf8mb3 core implementation module.

## Start

```sh
cargo add utf8mb3
```

## Use

```rust

  let str = "😊";
  let result = utf8mb3::encode(str);
  println!("encode: {}", result); // encode: 
  println!("decode: {}", utf8mb3::decode(&result)); // decode: 😊
```

## License

MIT
