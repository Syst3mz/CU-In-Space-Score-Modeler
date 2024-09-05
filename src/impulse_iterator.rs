use itertools::Itertools;
use crate::CONFIG;

pub fn new(number_of_stages: usize, total_impulse: u16) -> impl Iterator<Item=Vec<u16>>{
    let per_stage_max_impulse = total_impulse - (number_of_stages as u16 - 1);
    (CONFIG.min_stage_impulse..=per_stage_max_impulse)
        .into_iter()
        .combinations(number_of_stages)
        .filter(move |x| x.iter().sum::<u16>() <= total_impulse)
}
