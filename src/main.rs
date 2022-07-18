use std::{fs::File, time::Duration};

use ggez::{
    conf::WindowMode,
    event,
    graphics::{self, Color, DrawParam, Image, Rect, Transform},
    mint::{Point2, Vector2},
    Context, GameResult,
};
use glam::vec2;
use phirs::{
    chart_loaders::{Loader, OfficalLoader},
    phi_types::{Chart, NoteType},
};
use std::f32::consts::PI;


const FLICK_PATH: &str = "/flick.png";
const HIT_PATH: &str = "/tap.png";
const HOLD_PATH:&str = "/hold.png";
const HOLD_HEAD_PATH:&str = "/HoldHead.png";
const HOLD_BODY_PATH:&str = "/HoldBody.png";
const HOLD_TAIL_PATH:&str = "/HoldEnd.png";

const DRAG_PATH:&str = "/drag.png";
const SCALE_FACTOR: [f32; 2] = [0.1, 0.1];
// const ASSETS_PREFIX: &str = "/home/helium/coding/Rust/phirs/assets/";
// const FLICK_PATH: &str = &(ASSETS_PREFIX.to_owned() + "flick.png");
// const HIT_PATH: &str = &(ASSETS_PREFIX.to_owned() + "tap.png");
// const HOLD_PATH:&str = &(ASSETS_PREFIX.to_owned() + "hold.png");
// const DRAG_PATH:&str = ASSETS_PREFIX "drag.png";

struct GameState<'a> {
    chart: Chart<'a>,
    now_time: Duration,
    do_update: bool,
    win_width: f32,
    win_height: f32,
    assets: Assets,
}
struct Assets {
    flick: Image,
    hit: Image,
    hold: Image,
    hold_head: Image,
    hold_body: Image,
    hold_tail: Image,
    drag: Image,
}
impl Assets {
    fn gen(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            flick: Image::new(ctx, FLICK_PATH)?,
            hit: Image::new(ctx, HIT_PATH)?,
            hold: Image::new(ctx, HOLD_PATH)?,
            hold_head: Image::new(ctx, HOLD_HEAD_PATH)?,
            hold_body: Image::new(ctx, HOLD_BODY_PATH)?,
            hold_tail: Image::new(ctx, HOLD_TAIL_PATH)?,
            drag: Image::new(ctx, DRAG_PATH)?,
        })
    }
}

impl<'a> GameState<'a> {
    fn new(chart: Chart<'a>, assets: Assets) -> GameResult<Self> {
        Ok(Self {
            chart,
            now_time: Duration::new(14, 0),
            win_height: 800.0,
            win_width: 600.0,
            assets,
            do_update:true
        })
    }
    fn warph(&self, value: f32) -> f32 {
        value * self.win_height
    }
    fn warpw(&self, value: f32) -> f32 {
        value * self.win_width
    }
}
fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("super_simple", "ggez");
    cb = cb.add_resource_path("/home/helium/coding/Rust/phirs/assets/");
    let (mut ctx, event_loop) = cb.build()?;

    let file =
        File::open("/home/helium/coding/Rust/phirs/assets/Introduction_chart.json").expect("open err");
    let chart = OfficalLoader::load_chart(file);
    let assets = Assets::gen(&mut ctx)?;
    let state = GameState::new(chart, assets)?;
    event::run(ctx, event_loop, state)
}
fn angle_to_radians(angle: f32) -> f32 {
    angle * PI / 180.0
}

// fn make_hold(hold_head: &Image) -> Image{
//     todo!()
// }

impl event::EventHandler<ggez::GameError> for GameState<'_> {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        if self.do_update {
            self.now_time = self
                .now_time
                .checked_add(ggez::timer::delta(ctx))
                .expect("overflow");
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 0.0].into());
        graphics::set_screen_coordinates(
            ctx,
            Rect {
                x: 0.0,
                y: self.warph(1.0),
                w: self.warpw(1.0),
                h: self.warph(-1.0),
            },
        )?;
        let mut mode = WindowMode::default()
            .resizable(true)
            .resize_on_scale_factor_change(true);
        mode.height = self.win_height;
        mode.width = self.win_width;
        graphics::set_mode(ctx, mode)?;
        let time = self.now_time.as_secs_f32();
        for line in &self.chart.lines {
            let grfline = graphics::Mesh::new_line(
                ctx,
                &[[self.warpw(-1.0), 0.0], [self.warpw(1.0), 0.0]],
                self.warph(0.01),
                Color {
                    r: 255.0,
                    g: 255.0,
                    b: 255.0,
                    a: line.get_alpha_at(time),
                },
            )?;
            let pos = line.get_pos_at(time);
            let pos = Point2 {
                x: self.warpw(pos[0]),
                y: self.warph(pos[1]),
            };
            let angle = angle_to_radians(line.get_angle_at(time));
            let trans = Transform::Values {
                dest: pos,
                rotation: angle,
                scale: Vector2 { x: 1.0, y: 1.0 },
                offset: Point2 { x: 0.0, y: 0.0 },
            };
            let param = DrawParam {
                src: Rect::new(0.0, 0.0, 1.0, 1.0),
                color: Color::WHITE,
                trans,
            };
            let judge_pos = line.get_judge_at(time);
            graphics::draw(ctx, &grfline, param)?;
            for note in &line.notes_above {
                if note.time + match &note.note_type {
                    NoteType::Hold(f) => {*f}
                    _ => {Default::default()}
                }>= time {
                    let img = match &note.note_type {
                        NoteType::Hit => &self.assets.hit,
                        NoteType::Drag => &self.assets.drag,
                        NoteType::Hold(_f) => &self.assets.hold,
                        NoteType::Flick => &self.assets.flick,
                    };
                    let anchors: [f32; 2] = match &note.note_type {
                        NoteType::Hold(_f) => [
                            img.width() as f32 * SCALE_FACTOR[0] / 2.0 ,
                            self.assets.hit.height() as f32 * SCALE_FACTOR[1]/ 2.0 ,
                        ],
                        _otherwise => [img.width() as f32 * SCALE_FACTOR[0] / 2.0, img.height() as f32 * SCALE_FACTOR[1] / 2.0],
                    };
                    let rotation = glam::Mat2::from_angle(angle);
                    let note_x = note.pos_x;
                    let note_y = note.pos_y - judge_pos;

                    let mut param = DrawParam {
                        trans,
                        src: Rect::new(0.0, 0.0, 1.0, 1.0),
                        color: Color::WHITE,
                    };
                    let dest = rotation
                        * vec2(
                            self.warpw(note_x) - anchors[0],
                            self.warph(note_y) - anchors[1],
                        )
                        + vec2(pos.x, pos.y);
                    param = param.dest([dest.x, dest.y]).scale(SCALE_FACTOR);
                    if let NoteType::Hold(f) = note.note_type {
                        phirs::draw_hold(ctx, &self.assets.hold_head, &self.assets.hold_body, &self.assets.hold_tail, self.warph(line.get_judge_at(f+note.time) - note.pos_y), self.warph(-note_y),param)?;
                    }else {
                        graphics::draw(ctx, img, param)?;
                    }
                }
            }
            for note in &line.notes_below { // 以后再拆函数， 懒了
                if note.time + match &note.note_type {
                    NoteType::Hold(f) => {*f}
                    _ => {Default::default()}
                }>= time{
                    let img = match &note.note_type {
                        NoteType::Hit => &self.assets.hit,
                        NoteType::Drag => &self.assets.drag,
                        NoteType::Hold(_f) => &self.assets.hold,
                        NoteType::Flick => &self.assets.flick,
                    };
                    let anchors: [f32; 2] = match &note.note_type {
                        NoteType::Hold(_f) => [
                            img.width() as f32 * SCALE_FACTOR[0] / 2.0 ,
                            self.assets.hit.height() as f32 * SCALE_FACTOR[1]/ 2.0 ,
                        ],
                        _otherwise => [img.width() as f32 * SCALE_FACTOR[0] / 2.0, img.height() as f32 * SCALE_FACTOR[1] / 2.0],
                    };
                    let rotation = glam::Mat2::from_angle(angle);
                    let note_x = note.pos_x;
                    let note_y = note.pos_y - judge_pos;

                    let mut param = DrawParam {
                        trans,
                        src: Rect::new(0.0, 0.0, 1.0, 1.0),
                        color: Color::WHITE,
                    };
                    let dest = rotation
                        * vec2(
                            self.warpw(note_x) + anchors[0],
                            -self.warph(note_y) + anchors[1],
                        )
                        + vec2(pos.x, pos.y);
                    param = param.dest([dest.x, dest.y]).scale(SCALE_FACTOR).rotation(angle +PI);
                    if let NoteType::Hold(f) = note.note_type {
                        // TODO fix hold position
                        phirs::draw_hold(ctx, &self.assets.hold_head, &self.assets.hold_body, &self.assets.hold_tail, self.warph(line.get_judge_at(f+note.time) - note.pos_y), self.warph(-note_y),param)?;
                    }else {
                        graphics::draw(ctx, img, param)?;
                    }
                }
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
    fn resize_event(&mut self, _ctx: &mut ggez::Context, width: f32, height: f32) {
        self.win_height = height;
        self.win_width = width;
    }
    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) {
        if let event::MouseButton::Right = button {
            self.do_update = if self.do_update {false} else {true};
        }
    }
}
#[cfg(test)]
pub mod tests {
    use std::fs::File;

    use phirs::{
        chart_loaders::{Loader, OfficalLoader},
        phi_event::{event_bisect, EaseType, Event},
    };

    #[test]
    fn test_event_linear() {
        let event = Event::new(0.0, 10.0, 0.0, 10.0, EaseType::Linear);
        dbg!(&event);
        let result = event.get_at(5.0);
        assert_eq!(result, 5.0)
    }
    #[test]
    fn test_event_const() {
        let event2 = Event::new(0.0, 10.0, 3.0, 10.0, EaseType::Const);
        dbg!(&event2);
        assert_eq!(3.0, event2.get_at(5.0));
    }

    #[test]
    fn test_bisect() {
        let mut events: Vec<Event> = Vec::new();
        events.push(Event::new(0.0, 10.0, 0.2, 2.0, EaseType::Linear));
        events.push(Event::new(10.0, 12.0, 2.0, 5.0, EaseType::Linear));
        events.push(Event::new(12.0, 30.0, 5.0, 5.0, EaseType::Const));
        let wanted_time = 0.0;
        assert_eq!(
            events[event_bisect(&events, wanted_time)].get_at(wanted_time),
            0.2
        );
        let wanted_time = 10.0;
        assert_eq!(
            events[event_bisect(&events, wanted_time)].get_at(wanted_time),
            2.0
        );
        let wanted_time = 30.0;
        assert_eq!(
            events[event_bisect(&events, wanted_time)].get_at(wanted_time),
            5.0
        )
    }
    #[test]
    fn test_chart() {
        let file = File::open("/home/helium/coding/Rust/phirs/assets/Introduction_chart.json")
            .expect("open err");
        let chart = OfficalLoader::load_chart(file);
        assert_eq!(chart.lines[0].get_pos_at(0.0), [0.5, 0.5]);
        assert_eq!(chart.lines[0].get_judge_at(0.0), 0.0);
    }
}
