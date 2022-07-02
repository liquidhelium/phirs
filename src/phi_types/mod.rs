#![allow(unused)]
use crate::phi_event::{event_list_get_at, Event};
#[derive(Debug)]
pub enum NoteType {
    Hit,
    Drag,
    Flick,
    Hold(f32),
}
#[derive(Debug)]
pub struct JudgeLine<'a> {
    alpha: Vec<Event>,         // 0-255
    pos_x: Vec<Event>,         //  | mesurement: screen
    pos_y: Vec<Event>,         //  | also
    judge_line_at: Vec<Event>, //  | also
    angle: Vec<Event>,
    pub notes_above: Vec<Note<'a>>,
    pub notes_below: Vec<Note<'a>>,
}
impl<'a> JudgeLine<'a> {
    pub fn new(alpha: Vec<Event>, pos_x: Vec<Event>, pos_y: Vec<Event>, judge_line_at: Vec<Event>, angle: Vec<Event>, notes_above: Vec<Note<'a>>, notes_below: Vec<Note<'a>>) -> Self { Self { alpha, pos_x, pos_y, judge_line_at, angle, notes_above, notes_below } }

    pub fn get_alpha_at(&self, time: f32) -> f32 {
        event_list_get_at(&self.alpha, time)
    }
    pub fn get_pos_at(&self, time: f32) -> [f32;2] {
        [
            event_list_get_at(&self.pos_x, time),
            event_list_get_at(&self.pos_y, time),
        ]
    }
    pub fn get_angle_at(&self, time: f32) -> f32 {
        event_list_get_at(&self.angle, time)
    }
    pub fn get_judge_at(&self, time: f32) -> f32 {
        event_list_get_at(&self.judge_line_at, time)
    }
}
#[derive(Debug)]
pub struct Note<'a> {
    pub note_type: NoteType,
    pub time: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub parent: Option<&'a JudgeLine<'a>>,
}

impl<'a> Note<'a> {
    pub fn new(note_type: NoteType, time: f32, pos_x: f32, pos_y: f32, parent: Option<&'a JudgeLine<'a>>) -> Self { Self { note_type, time, pos_x, pos_y, parent } }

}

#[derive(Debug)]
pub struct Chart<'a> {
    pub lines: Vec<JudgeLine<'a>>,
    pub version: i8,
}

impl<'a> Chart<'a> {
    pub fn new(lines: Vec<JudgeLine<'a>>, version: i8) -> Self { Self { lines, version} }
}
