# rg
废话生成器


```rust
#![feature(trace_macros)]
trace_macros!(true);
use rg::combine;
use rg::Mode;
use rg::Name;
use rg::NameKind;
use rg::Rg;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut rg = Rg::new();

    rg.left_dec("{").right_dec("}");

    // 1. 生成一个
    // pass separator
    let res = rg.generate::<&str, _>(Mode::ASLP(","));
    println!("{:?}", res);

    // 2. 使用迭代器
    for x in rg.iter::<&str>(&Mode::Adj).take(10) {
        println!("{:?}", x);
    }

    // 3. 使用组合
    for _ in 0..10 {
        let res = rg.combine(
            &[Mode::Diy(&["ABC", "abc", "123"]), Mode::Rand, Mode::Rand],
            &[",", "?", "!"],
        );
        println!("{:?}", res);
    }

    rg.reset();

    // 4. 生成用户名
    for _ in 0..20 {
        let res = rg.combine::<&str, _>(
            &[
                // 中文
                Mode::Noun,
                // 小写字母
                Mode::Name(Name::new(NameKind::Lowers, 0..=5)),
                // 数字
                Mode::Name(Name::new(NameKind::Digits, 0..=3)),
            ],
            &[],
        );
        println!("{:?}", res);
    }

    // 5. 使用宏
    println!("-----------USE MACRO-----------");
    let r = combine!(Mode::Noun, Mode::Adverb; ",", "");
    println!("{:?}", r);

    println!("{:?}", now.elapsed());
}
```

