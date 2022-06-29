fn main() {
}

#[cfg(test)]
pub mod tests{
    use std::fs::File;

    use phirs::{
        phi_event::{EaseType, Event, event_bisect}, 
        chart_loaders::{Loader, OfficalLoader}
    };


    #[test]
    fn test_event_linear() {
        let event = Event::new(
            0.0,
            10.0,
            0.0,
            10.0,
            EaseType::Linear
        );
        dbg!(&event);
        let result = event.get_at(5.0);
        assert_eq!(result, 5.0)
    }
    #[test]
    fn test_event_const() {
    
        let event2 = Event::new(
            0.0, 
            10.0, 
            3.0, 
            10.0, 
            EaseType::Const
        );
        dbg!(&event2);
        assert_eq!(3.0,event2.get_at(5.0));
    }

    #[test]
    fn test_bisect() {
        let mut events:Vec<Event> = Vec::new();
        events.push(Event::new(0.0, 10.0, 0.2, 2.0, EaseType::Linear));
        events.push(Event::new(10.0, 12.0, 2.0, 5.0, EaseType::Linear));
        events.push(Event::new(12.0, 30.0, 5.0, 5.0, EaseType::Const));
        let wanted_time = 0.0;
        assert_eq!(events[event_bisect(&events, wanted_time)].get_at(wanted_time),0.2);
        let wanted_time = 10.0;
        assert_eq!(events[event_bisect(&events, wanted_time)].get_at(wanted_time),2.0);
        let wanted_time = 30.0;
        assert_eq!(events[event_bisect(&events, wanted_time)].get_at(wanted_time),5.0)
    }
    #[test]
    fn test_chart() {
        let file = File::open("/home/helium/coding/Rust/phirs/assets/Introduction_chart.json").expect("open err");
        let chart = OfficalLoader::load_chart(&file);
        dbg!(&chart.lines[0]);
    }
}