use macroquad::{prelude::*, rand::RandomRange};

const NUM: usize = 500;
const R: usize = 0;
const G: usize = 0;
const B: usize = 0;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: usize,
}

fn create_particles() -> Vec<Particle> {
    let mut particles = vec![];
    let colors = vec![R, G, B];

    for _i in 0..NUM {
        let x = RandomRange::gen_range(0., screen_width());
        let y = RandomRange::gen_range(0., screen_height());

        let vx = RandomRange::gen_range(0., 2.);
        let vy = RandomRange::gen_range(0., 2.);

        let icolor = RandomRange::gen_range(0, colors.len());

        let pos = vec2(x, y);
        let vel = vec2(vx, vy);
        let color = colors[icolor];

        particles.push(Particle { pos, vel, color })
    }

    return particles;
}

fn get_color(color: usize) -> Color {
    if color == R {
        return RED;
    }

    if color == G {
        return GREEN;
    }

    if color == B {
        return BLUE;
    }

    return BLACK;
}

fn draw(particles: &Vec<Particle>) {
    for p in particles {
        let color = get_color(p.color);
        draw_circle(p.pos.x, p.pos.y, 5., color)
    }
}

#[macroquad::main("ParticleLife")]
async fn main() {
    let particles = create_particles();

    loop {
        clear_background(BLACK);

        draw(&particles);
        next_frame().await;
    }
}
