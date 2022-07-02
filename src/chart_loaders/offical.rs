use serde_json::Value;

use crate::{
    phi_event::{EaseType, Event},
    phi_types::{JudgeLine, Note, NoteType, Chart},
};

use super::Loader;
pub struct OfficalLoader {}

impl OfficalLoader {
    fn phi_to_real(phi: f32, bpm: f32) -> f32 {
        phi / 32.0 * (60.0 / bpm)
    }
    fn make_note<'a>(note: &serde_json::Value, parent_bpm: f32) -> Note<'a> {
        let phi_time = note["time"].as_i64().expect("time failed");
        let time = OfficalLoader::phi_to_real(phi_time as f32, parent_bpm);
        let posx = note["positionX"].as_f64().expect("posX failed") / 18.0;
        let posy = note["floorPosition"].as_f64().expect("posY failed") / 2.0;
        let hold_time_phi = note["holdTime"].as_f64().expect("holdtime failed");
        let hold_time = OfficalLoader::phi_to_real(hold_time_phi as f32, parent_bpm);
        let note_type = match note["type"].as_i64() {
            Some(int) => match int {
                1 => NoteType::Hit,
                2 => NoteType::Drag,
                3 => NoteType::Hold(hold_time),
                4 => NoteType::Flick,
                _ => panic!("invalid type"),
            },
            None => panic!("Wrong Type for note"),
        };
        Note::new(note_type, time, posx as f32, posy as f32, None)
    }
    
    fn make_judgepos_event(event: &serde_json::Value, parent_bpm: f32) -> Event {
        let start_time_phi = event["startTime"].as_f64().expect("start failed");
        let end_time_phi = event["endTime"].as_f64().expect("end failed");
        let start_time = OfficalLoader::phi_to_real(start_time_phi as f32, parent_bpm);
        let end_time = OfficalLoader::phi_to_real(end_time_phi as f32, parent_bpm);
        let time_delta = end_time - start_time;
        let floorpos = event["floorPosition"].as_f64().expect("floor failed");
        let speed = event["value"].as_f64().expect("speed failed");
        let end_position = floorpos as f32 + (speed as f32) * time_delta;
        Event::new(
            start_time,
            end_time,
            floorpos as f32 / 2.0,
            end_position / 2.0,
            EaseType::Linear,
        )
    }
    fn make_alpha_event(event: &serde_json::Value, parent_bpm: f32) -> Event {
        let start = event["start"].as_f64().expect("start failed") * 255.0;
        let end = event["end"].as_f64().expect("end failed") * 255.0;
        let start_time_phi = event["startTime"].as_f64().expect("starttime failed");
        let end_time_phi = event["endTime"].as_f64().expect("endtime failed");
        let start_time = OfficalLoader::phi_to_real(start_time_phi as f32, parent_bpm);
        let end_time = OfficalLoader::phi_to_real(end_time_phi as f32, parent_bpm);
        Event::new(start_time, end_time, start as f32, end as f32, EaseType::Linear)
    }
    fn make_pos_event(event: &serde_json::Value, parent_bpm: f32) -> (Event, Event) {
        let start = event["start"].as_f64().expect("start failed");
        let end = event["end"].as_f64().expect("end failed");
        let start2 = event["start2"].as_f64().expect("start2 failed");
        let end2 = event["end2"].as_f64().expect("end2 failed");
        let start_time_phi = event["startTime"].as_f64().expect("starttime failed");
        let end_time_phi = event["endTime"].as_f64().expect("endtime failed");
        let start_time = OfficalLoader::phi_to_real(start_time_phi as f32, parent_bpm);
        let end_time = OfficalLoader::phi_to_real(end_time_phi as f32, parent_bpm);
        let event1 = Event::new(
            start_time,
            end_time,
            start as f32,
            end as f32,
            EaseType::Linear
        );
        let event2 = Event::new(
            start_time,
            end_time,
            start2 as f32,
            end2 as f32,
            EaseType::Linear
        );
        (event1,event2)
    }
    fn make_angle_event(event: &serde_json::Value, parent_bpm: f32) -> Event {
        let start = event["start"].as_f64().expect("start failed");
        let end = event["end"].as_f64().expect("end failed");
        let start_time_phi = event["startTime"].as_f64().expect("starttime failed");
        let end_time_phi = event["endTime"].as_f64().expect("endtime failed");
        let start_time = OfficalLoader::phi_to_real(start_time_phi as f32, parent_bpm);
        let end_time = OfficalLoader::phi_to_real(end_time_phi as f32, parent_bpm);
        Event::new(start_time, end_time, start as f32, end as f32, EaseType::Linear)
    }
    fn make_vector<T, F>(jsonlist: &serde_json::Value, parent_bpm: f32, func: F) -> Vec<T>
        where F: Fn(&serde_json::Value, f32) -> T
    {
        let arr = jsonlist.as_array().expect("error in array");
        let mut ret:Vec<T> = Vec::new();
        for va in arr {
            ret.push(func(va, parent_bpm))
        }
        ret
    }
    fn split_xy(vec: Vec<(Event,Event)>, vec_x: &mut Vec<Event>, vec_y: &mut Vec<Event>) {
        for va in vec {
            vec_x.push(va.0);
            vec_y.push(va.1);
        }
    }
    fn make_line<'a>(line: &serde_json::Value) -> JudgeLine {
        let bpm = line["bpm"].as_f64().expect("wrong bpm") as f32;
        let notes_above_list = &line["notesAbove"];
        let notes_below_list = &line["notesBelow"];
        let alpha_event_list = &line["judgeLineDisappearEvents"];
        let move_event_list = &line["judgeLineMoveEvents"];
        let angle_event_list = &line["judgeLineRotateEvents"];
        let judgepos_event_list = &line["speedEvents"];
        let notes_above = OfficalLoader::make_vector(notes_above_list, bpm, OfficalLoader::make_note);
        let notes_below = OfficalLoader::make_vector(notes_below_list, bpm, OfficalLoader::make_note);
        let alpha_events = OfficalLoader::make_vector(alpha_event_list, bpm, OfficalLoader::make_alpha_event);
        let judgepos_events = OfficalLoader::make_vector(judgepos_event_list, bpm, OfficalLoader::make_judgepos_event);
        let move_events = OfficalLoader::make_vector(move_event_list, bpm, OfficalLoader::make_pos_event);
        let angle_events= OfficalLoader::make_vector(angle_event_list, bpm, OfficalLoader::make_angle_event);
        let mut pos_x_events:Vec<Event> = Vec::new();
        let mut pos_y_events:Vec<Event> = Vec::new();
        OfficalLoader::split_xy(move_events, &mut pos_x_events, &mut pos_y_events);
        JudgeLine::new(
            alpha_events,
            pos_x_events,
            pos_y_events,
            judgepos_events,
            angle_events,
            notes_above,
            notes_below
        )
    }
    fn make_line_list<'a>(chart: &serde_json::Value) -> Vec<JudgeLine<'a>> {
        unsafe{
            let line_list_json = chart["judgeLineList"].as_array().expect("line failed")as *const Vec<Value>;
            let mut line_list:Vec<JudgeLine> = Vec::new();
            for i in &*line_list_json {
                line_list.push(OfficalLoader::make_line(i));
            }
            line_list
        }
    }
}

impl Loader for OfficalLoader {
    fn load_chart<'a>(file: std::fs::File) -> crate::phi_types::Chart<'a> {
        let json_chart:Value = serde_json::from_reader(file)
            .expect("The file is unreadable!");
        let line_list = OfficalLoader::make_line_list(&json_chart);
        Chart::new(line_list, 3)
    }
    // fn save_chart(chart: crate::phi_types::Chart) -> std::io::Result<bool> {

    // }
}
