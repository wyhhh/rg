#![feature(trace_macros)]
trace_macros!(true);
use rg::combinator::select;
use rg::combinator::Generator;
use rg::combinator::RgBindMode;
use rg::combine;
use rg::extend::Case;
use rg::fmt::json::Json;
use rg::Mode;
use rg::Others;
use rg::Rg;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut rg = Rg::new();

    rg.left_dec("{").right_dec("}");

    // 1. 生成一个
    // pass separator
    let res = rg.once::<&str, _>(Mode::ASLP(","));
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

    // 4. 生成昵称
    for _ in 0..20 {
        let res = rg.combine::<&str, _>(
            &[
                // 中文
                Mode::Noun,
                // 小写字母
                Mode::Others(Others::Lowers(0..=5)),
                // 数字
                Mode::Others(Others::Digits(0..=3)),
            ],
            &[],
        );
        println!("{:?}", res);
    }

    // 5. 使用宏
    let r = combine!(Mode::Noun, Mode::Adverb; ",", "");
    println!("{:?}", r);

    // 6. 生成数字
    let res = rg.numberic(1..=5, true);
    println!("{:?}", res);

    // 7. 生成单词
    let res = rg.word(3..=10, Case::Lower);
    println!("{:?}", res);

    // 8. 使用combinator
    let g: RgBindMode<&str> = RgBindMode::new(Mode::Noun);
    let g = g.and(RgBindMode::<&str>::new(Mode::Verb));
    let g = g.or(RgBindMode::<&str>::new(Mode::Others(Others::Digits(1..=3))));
    let g = g.map(|mut s| {
        s.push_str("~~");
        s
    });
    let g = g.tail("**");
    let mut g = g.repeat(3);
    let mut g2 = RgBindMode::<&str>::new(Mode::Noun);
    let arr: &mut [&mut dyn Generator] = &mut [&mut g, &mut g2];
    let g = select(arr);

    println!("{:?}", g.generate());

    // 9. 生成随机Json
    let mut json = Json::new();
    let res = json.generate();
    println!("{}", res);

    // 10. 小数
    let res = rg.float(1..=5, 1..=2, true);
    println!("{:?}", res);

    // 11. boolean
    let res = rg.boolean();
    println!("{:?}", res);

    println!("ok. cost: {:?}", now.elapsed());
}
