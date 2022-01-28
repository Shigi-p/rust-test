- [単語帳](#単語帳)
  - [cargo](#cargo)
    - [Cargo.toml](#cargotoml)
    - [Cargo.lock](#cargolock)
  - [Types](#types)
    - [Scalar Types](#scalar-types)
    - [Compound Types](#compound-types)
# 単語帳

## cargo

### Cargo.toml

気分的にはpackage.jsonみたいなもので、ここに使用するクレートやバージョンを記述しておく感じ？

### Cargo.lock

まんまyarn.lockとかpackage.lock.jsonみたいなものかなぁと思っている。

## Types
### [Scalar Types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html#scalar-types)

- 符号付き整数: `i8`, `i16`, `i32`, `i64`, `i128`, `isize（ポインタのサイズ）`
- 符号無し整数: `u8`, `u16`, `u32`, `u64`, `u128`, `usize（ポインタのサイズ）`
- 浮動小数点数: `f32`, `f64`
- char: `'a'`, `'α'`, `'∞'`などのUnicodeのスカラー値
- bool: `true`または`false`
- ユニット型: ()が唯一の値

### [Compound Types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html#compound-types)

- 配列: e.g. `[1, 2, 3]`など
- タプル: e.g. `(1, true)`