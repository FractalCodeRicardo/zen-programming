use macroquad::{color::{BLACK, PURPLE}, input::{KeyCode, is_key_pressed}, math::{Vec2, vec2}, prelude::collections, shapes::draw_line, window::{clear_background, next_frame}};

const SIZES: i8 = 8;
const PROPORTION: f32 = 0.5;
const RADIUS:f32 =  300.;
struct Line {
    p1: Vec2,
    p2: Vec2
}

fn get_lines(line: &Line) -> Vec<Line> {
    let mut lines = vec![];

    let l = line.p2 - line.p1;
    let mut p = vec2(l.y * -1. , l.x);
    p = p.normalize() *  l.length() * PROPORTION;

    let pi = line.p1;
    let pf = line.p2;
    let pc = pi + l * 0.5;

    let p1 = pi;
    let p2 = pi + l * 0.25;
    let p3 = pc + p;
    let p4 = pi + l * 0.75;
    let p5 = pf;

    lines.push(Line {p1:  p1, p2: p2});
    lines.push(Line {p1:  p2, p2: p3});
    lines.push(Line {p1:  p3, p2: p4});
    lines.push(Line {p1:  p4, p2: p5});

    lines
}

fn replace_lines(lines: Vec<Line>) -> Vec<Line> {
    let mut output_lines = vec![];

    for l in &lines {
        let mut new_lines = get_lines(l);
        output_lines.append( &mut new_lines);
    }
    output_lines
}

fn get_init_lines() -> Vec<Line> {
    let mut lines = vec![];

    let mut angle: f32 = 0.;
    let mut count = 0;
    let mut points = vec![];


    while count < SIZES {
        let x = RADIUS + 300. * angle.to_radians().cos();
        let y = RADIUS + 300. * angle.to_radians().sin();


        points.push(vec2(x, y));

        angle += 360. / SIZES as f32;
        count += 1;
    }

    for i in 0..points.len() -1 {
        lines.push(Line {
            p1: points[i],
            p2: points[i+1]
        });
    }

        lines.push(Line {
            p1: points[points.len() -1],
            p2: points[0]
        });

    lines
}

#[macroquad::main("Koch")]
async fn main() {


    let mut lines = get_init_lines();

    loop {
        clear_background(BLACK);


        for l in &lines {
            draw_line(l.p1.x, l.p1.y, l.p2.x,l.p2.y, 2., PURPLE);
        }

        if is_key_pressed(KeyCode::Space) {
            lines = replace_lines(lines)
        }

        next_frame().await;
    }
}
