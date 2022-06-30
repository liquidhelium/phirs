#[derive(Debug)]
pub enum EaseType {
    Linear,
    Const,
}

pub fn interplot(time1: f32, time2: f32, y1: f32, y2: f32, time: f32, e_type: &EaseType) -> f32{
    match e_type {
        EaseType::Linear => linear(time1, time2, y1, y2, time),
        EaseType::Const => y1
    }
}

fn linear(time1: f32, time2: f32,y1: f32,y2: f32, time: f32) -> f32{
    if time2 != time1 {y1 + (y2-y1) / (time2-time1) * (time - time1)} else {y1}
}