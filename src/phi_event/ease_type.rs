#[derive(Debug)]
pub enum EaseType {
    Linear,
    Const,
}

pub fn interplot(time1: f64, time2: f64, y1: f64, y2: f64, time: f64, e_type: &EaseType) -> f64{
    match e_type {
        EaseType::Linear => linear(time1, time2, y1, y2, time),
        EaseType::Const => y1
    }
}

fn linear(time1: f64, time2: f64,y1: f64,y2: f64, time: f64) -> f64{
    if time2 != time1 {y1 + (y2-y1) / (time2-time1) * (time - time1)} else {y1}
}