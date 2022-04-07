use std::cmp::min;

use rand::Rng;

use probability::Probability;

#[test]
fn main() {
    // 模拟次数
    let n = 10_000_00;
    // 原概率
    let prob = 0.05;
    // 保底限制
    let tb = 60;
    // 欧皇限制
    let db = 0;
    let mut y = vec![0; 100];
    println!("原概率：{}，模拟次数：{}", prob, n);
    let cut = min(tb, 100) - 1;

    let mut rng = rand::thread_rng();
    // 模拟次数
    let mut ct = 0;
    // 真随机
    for _ in 0..n {
        ct += 1;
        if ct >= db && (rng.gen::<f32>() < prob || ct >= tb) {
            if ct < 100 {
                y[ct] += 1;
            }
            ct = 0;
        }
    }
    y.remove(0);
    // 中奖次数
    let sun = y.iter().sum::<i32>();
    println!(
        "真随机： 期望概率：{}，保底占比：{}",
        sun as f32 / n as f32,
        y[cut] as f32 / sun as f32
    );

    // 伪随机
    let mut y = vec![0; 100];
    let mut ct: usize = 0;
    for _ in 0..n {
        ct += 1;
        if Probability::draw(ct) {
            y[ct] += 1;
            ct = 0;
        }
    }
    y.remove(0);
    // 中奖次数
    let sun = y.iter().sum::<i32>();
    println!(
        "伪随机： 期望概率：{}，保底占比：{}",
        sun as f32 / n as f32,
        y[cut] as f32 / sun as f32
    );
}
