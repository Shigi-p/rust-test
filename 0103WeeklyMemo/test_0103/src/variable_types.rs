pub fn inference_test() {
    // アノテーションのおかげで、コンパイラは`elem`がu8型であることがわかる。
    let elem = 5u8;

    // 空のベクトル（可変長の配列）を生成
    let mut vec = Vec::new();
    // この時点でコンパイラは`vec`の型を知らず、
    // 単に何らかの値のベクトル(`Vec<_>`)であるということだけを把握している。\

    // NOTE: これvscodeに無茶苦茶怒られるので、やっぱり型付けの恩恵にあやかろう
    // let mut _vec = Vec::new();

    // `elem`をベクトルに挿入
    vec.push(elem);
    // よし！ これでコンパイラは`vec`が`u8`のベクトル(`Vec<u8>`)であることを把握する。
    // TODO ^ 上の `vec.push(elem)` をコメントアウトしてみましょう。

    println!("{:?}", vec);
    // println!("{:?}", _vec);
}

pub fn aliasing_test() {
    // `NanoSecond` is a new name for `u64`.
    type NanoSecond = u64;
    type Inch = u64;

    // Use an attribute to silence warning.
    #[allow(non_camel_case_types)]
    type u64_t = u64;
    // TODO ^ Try removing the attribute

    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    type User = (
        i32, // age
        i32, // height
    );

    let john_due: User = (18, 170);

    println!("John Due: age is {}, height is {}", john_due.0, john_due.1);
}
