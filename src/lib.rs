#![deny(clippy::pedantic)]

use rand::Rng;

pub struct Probability;

impl Probability {
    /// 概率
    const PROBABILITY: f32 = 0.0038;
    /// 保底概率
    const MIN_PROBABILITY: usize = 60;
    /// 欧皇限制
    const RESTRICTIONS: usize = 1;

    /// 抽奖
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn draw(num: usize) -> bool {
        let mut rng = rand::thread_rng();
        let prob = rng.gen::<f32>();
        if num >= Self::RESTRICTIONS
            && (prob < Self::PROBABILITY * num as f32 || num >= Self::MIN_PROBABILITY)
        {
            return true;
        }
        false
    }
}
