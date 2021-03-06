use ggez::{GameError, Context};
use ggez::graphics::{Rect, Image, draw, DrawParam, Transform};
use glam::Vec2;
pub fn draw_hold(ctx: &mut Context,hold_head: &Image, hold_body:&Image, hold_tail: &Image, hold_len: f32, cut_pos: f32,draw_param: DrawParam)  -> Result<(),GameError>{
    // let note_width = hold_head.width();
    let hold_headtail_height = hold_head.height();
    let body_len = hold_body.height();
    let body_trans: mint::ColumnMatrix4<f32>;
    let head_trans: mint::ColumnMatrix4<f32>;
    let tail_trans: mint::ColumnMatrix4<f32>;
    let head_cut: f32;
    let body_cut: f32;
    let tail_cut: f32;
    match draw_param.trans {
        Transform::Values { dest: origin_dest, rotation: rt, scale: scl, offset: of } => {
            let rotation = glam::Mat2::from_angle(rt);
            let head_pt = Vec2::new(0.0, hold_headtail_height as f32 *scl.y);
            let body_pt = Vec2::new(0.0,hold_headtail_height as f32 *scl.y + hold_len);
            let tail_pt = Vec2::new(0.0, hold_headtail_height as f32 *scl.y *2.0 + hold_len);
            let body_rt = rotation *body_pt;
            let head_rt = rotation *head_pt;
            let tail_rt = rotation *tail_pt;
            head_cut = float_max(float_min(cut_pos, head_pt.y)/head_pt.y, 0.0);
            body_cut = float_max(float_min(cut_pos-head_pt.y,body_pt.y)/body_pt.y, 0.0);
            tail_cut = float_max(float_min(cut_pos-body_pt.y, tail_pt.y)/tail_pt.y, 0.0);
            body_trans = Transform::Values { 
                dest: mint::Point2 {y:origin_dest.y+body_rt.y,x: origin_dest.x+body_rt.x},
                rotation: rt,
                scale: mint::Vector2 {y: -hold_len/(body_len as f32), ..scl},
                offset: of,
            }.to_bare_matrix();
            head_trans = Transform::Values { 
                dest: mint::Point2 {y:origin_dest.y+head_rt.y,x: origin_dest.x+head_rt.x},
                rotation: rt,
                scale: mint::Vector2 { x: scl.x, y: -scl.y },
                offset: of,
            }.to_bare_matrix();
            tail_trans = Transform::Values { 
                dest: mint::Point2 {y:origin_dest.y+tail_rt.y,x: origin_dest.x+tail_rt.x},
                rotation: rt,
                scale: mint::Vector2 { x: scl.x, y: -scl.y },
                offset: of,
            }.to_bare_matrix();
        }
        _ => todo!()
    }
    let mut body_param =draw_param.to_owned();
    let mut head_param = draw_param.to_owned();
    let mut tail_param = draw_param.to_owned();
    body_param = body_param.transform(body_trans).src(Rect {x: 0.0, y: dbg!(body_cut), w: 1.0, h: 1.0-body_cut});
    head_param = head_param.transform(head_trans).src(Rect {x: 0.0, y: head_cut, w: 1.0, h: 1.0-head_cut});
    tail_param =tail_param.transform(tail_trans).src(Rect {x: 0.0, y: tail_cut, w: 1.0, h: 1.0-tail_cut});

    draw(ctx,hold_head,head_param)?;
    draw(ctx, hold_body, body_param)?;
    draw(ctx,hold_tail,tail_param)?;
    // overlay(&mut target, hold_head, 0, (hold_len/NOTE_SCALE ) as i64 + hold_headtail_height as i64);
    Ok(())
}

pub fn float_min(a1:f32, a2: f32) -> f32{
    if a1.le(&a2) {a1} else {a2}
}

pub fn float_max(a1:f32, a2: f32) -> f32{
    if a1.ge(&a2) {a1} else {a2}
}