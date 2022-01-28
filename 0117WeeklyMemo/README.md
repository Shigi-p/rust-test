## rust by examples

### 5. types

#### Casting

```rust
let decimal = 65.4321_f32;
let integer = decimal as u8;
let character = integer as char;

println!("Casting: {} -> {} -> {}", decimal, integer, character);
```

`Casting: 65.4321 -> 65 -> A`

実行結果はこう(なんでintの65をcharにキャストするとAなの？って思ったけどこれアスキーコードか…)

> when casting any value to an unsigned type, T, T::MAX + 1 is added or subtracted until the value fits into the new type
>
> 何らかの値を符号なしの型（仮にTとする）へキャスティングすると値がTに収まるまで、T::MAX + 1 が加算あるいは減算される。

ほへ〜そんなつくりになってるんや、無理やりasでキャストするのは基本的に危険と考えたほうがよさそう

(というか、偏見なんだけどこういった静的型付けの言語でキャストはしないにこしたことが良い気がしている…)

#### Literals

なんか特に言うことはなく…

#### Inference

こちらも特に言うことはなさそう。型推論はできるけどもせっかくなので使うものは明示しておくとハッピーになれる気がしている

というかコレ以前にどこかで触れたな？なんとなく覚えているぞ

#### alias

いつの間にかこれもどこかで触れていた、やってたやってた

ちょっとまってここまで書いて気がついたけど**俺0103の週にここやってんじゃん！！！**ちくしょう……もったいない二度手間を踏んでしまった…

### 6. Conversion

#### From and Into

> もし型Aから型Bへの変換ができるのであれば、型Bから型Aへの変換もできると思うのが自然です。

言われてみりゃ確かに

名前通りで、別の型からある型を作り出すのがFrom、ある型から別の型を作り出す(Fromの逆を行う)のがInto。
(なのでIntoはFromさえ定義されていれば、それの逆を行えばいいのでFromさえあれば大丈夫っぽい)

自作の型に対しても、FromとIntoを定義してあげればうまいことうごくらしい。

