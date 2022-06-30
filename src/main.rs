use std::{time::Duration, fs::File};

use ggez::{GameResult, event, graphics::{self, Transform, DrawParam, Rect, Color}, mint::{Point2, Vector2}, conf::WindowMode,};
use phirs::{phi_types::Chart, chart_loaders::{OfficalLoader, Loader}};
use std::f32::consts::PI;

struct GameState<'a> {
    chart: Chart<'a>,
    now_time: Duration,
    win_width: f32,
    win_height:f32
}

impl<'a> GameState<'a> {
    fn new(chart: Chart<'a>) -> GameResult<Self> { Ok(Self { chart, now_time:Duration::new(0,0),win_height:800.0,win_width:600.0}) }
}
fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;

    let file = File::open("/home/helium/coding/Rust/phirs/assets/Chart_IN_Error").expect("open err");
    let chart = OfficalLoader::load_chart(file);
    let state = GameState::new(chart)?;
    event::run(ctx, event_loop, state)

}
fn angle_to_radians(angle:f32) -> f32 {
    angle * PI / 180.0
}

impl event::EventHandler<ggez::GameError> for GameState<'_> {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.now_time = self.now_time.checked_add(ggez::timer::delta(ctx)).expect("overflow");
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        graphics::clear(ctx, [0.0,0.0,0.0,0.0].into());
        graphics::set_screen_coordinates(ctx, Rect { x: 0.0, y: 1.0, w: 1.0, h: -1.0 })?;
        let mut mode =WindowMode::default()
        .resizable(true)
        .resize_on_scale_factor_change(true);
        mode.height = self.win_height;
        mode.width = self.win_width;
        graphics::set_mode(ctx, mode)?;
        let time = self.now_time.as_secs_f32();
        for line in &self.chart.lines {
            let grfline = graphics::Mesh::new_line(
                ctx, 
                &[[-2.0, 0.0],[2.0, 0.0]], 
                0.01,
                Color { r: 255.0, g: 255.0, b: 255.0, a: line.get_alpha_at(time) }
            )?;
            let pos = dbg!(line.get_pos_at(time));
            let trans = Transform::Values {
                dest: Point2 { x: pos[0], y: pos[1] },
                rotation: angle_to_radians(line.get_angle_at(time)),
                scale: Vector2 { x: 1.0, y: 1.0 },
                offset: Point2 {x:0.0,y:0.0}
            };
            let param = DrawParam {
                src: Rect::new(0.0, 0.0, 1.0, 1.0),
                color: Color::WHITE,
                trans
            };
            graphics::draw(ctx, &grfline, param)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
    fn resize_event(&mut self, _ctx: &mut ggez::Context, width: f32, height: f32) {
        self.win_height = height;
        self.win_width = width;
    }
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
        let chart = OfficalLoader::load_chart(file);
        assert_eq!(chart.lines[0].get_pos_at(0.0),[0.5,0.5]);
        assert_eq!(chart.lines[0].get_judge_at(0.0), 0.0);
    }
}
