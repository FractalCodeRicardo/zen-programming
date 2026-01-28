use macroquad::{
    color::{Color, BLACK, GREEN},
    math::{vec2, Vec2},
    shapes::draw_poly_lines,
    window::{clear_background, next_frame, screen_height, screen_width},
};

const MAX_LEVEL: i8 = 6;
const RADIUS:f32 = 200.;

struct Pentagon {
    pos: Vec2,
    rotation: f32,
    radius: f32,
    level: i8,
}

fn get_poly_points(pentagon: &Pentagon) -> Vec<Vec2> {
    let mut points = vec![];
    for i in 0..5 {
        let angle: f32 = pentagon.rotation + (i as f32 * 72 as f32);
        let mut x = angle.to_radians().cos() * pentagon.radius;
        let mut y = angle.to_radians().sin() * pentagon.radius;

        x += pentagon.pos.x;
        y += pentagon.pos.y;

        points.push(vec2(x, y));
    }

    points
}

fn get_pentagons(parent: &Pentagon, level: i8) -> Vec<Pentagon> {
    let mut pentagons = vec![];

    if level == MAX_LEVEL {
        return pentagons;
    }

    let points = get_poly_points(parent);

    for p in &points {
        let pentagon = Pentagon {
            pos: p.clone(),
            rotation: parent.rotation + 72.,
            radius: parent.radius / 2.,
            level: level,
        };

        pentagons.extend(get_pentagons(&pentagon, level + 1));
        pentagons.push(pentagon);
    }

    return pentagons;
}

fn get_line_size(level: i8) -> f32 {
    let proportion = level as f32 / MAX_LEVEL as f32;

    return 4. - 4. * proportion; 
}

fn get_color(level: i8) -> Color {
    let color = GREEN;
    let proportion = level as f32 / MAX_LEVEL as f32;

    return Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: 1.- proportion 
    }
}

#[macroquad::main("Main")]
async fn main() {
    let pos = Vec2 {
        x: screen_width() / 2. + 30.,
        y: screen_height() / 2. + 100.,
    };

    let mut pentagons = vec![];

    let parent = Pentagon {
        pos: pos,
        radius: RADIUS,
        rotation: 0.,
        level: 0,
    };

    pentagons.extend(get_pentagons(&parent, 0));
    pentagons.push(parent);

    let mut count = 0;
    let partition = pentagons.len() / 50;

    loop {
        clear_background(BLACK);

        for i in 0..count{
            let p = &pentagons[i];
            draw_poly_lines(
                p.pos.x,
                p.pos.y,
                5,
                p.radius,
                p.rotation,
                get_line_size(p.level),
                get_color(p.level),
            );
        }

        if count + partition < pentagons.len() {
            count += partition;
        }

        next_frame().await
    }
}
