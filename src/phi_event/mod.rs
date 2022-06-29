mod ease_type;

pub use ease_type::EaseType;
use ease_type::interplot;
use bisection::bisect_right_by as bisect;
use core::cmp::Ordering;
#[derive(Debug)]
pub struct Event {
    start_time: f64,
    end_time: f64,
    start_value:f64,
    end_value:f64,
    ease_type:EaseType,
    
}

impl Event {
    pub fn get_at(&self, time:f64) -> f64 {
        interplot(self.start_time, self.end_time,self.start_value, self.end_value, time, &self.ease_type)
    }
    pub fn new(start_time:f64,end_time:f64,start_value:f64,end_value:f64,ease_type:EaseType) -> Event{
        Event { start_time, end_time, start_value, end_value, ease_type }
    }
}

pub fn event_bisect(arr: &Vec<Event>, want_time:f64) -> usize{
    let compare =  |a:&Event| {
        if a.start_time > want_time{
            Ordering::Less
        } else if a.start_time == want_time {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    };
    dbg!(bisect(&arr, &compare) - 1)
}

pub fn event_list_get_at(arr: &Vec<Event>,time:f64) -> f64 {
    arr[event_bisect(arr, time)].get_at(time)
}