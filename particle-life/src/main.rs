use macroquad::{prelude::*, rand::RandomRange};
use std::fs;
const NUM: usize = 1000;
const R: usize = 0;
const G: usize = 1;
const B: usize = 2;

const RADIUS: f32 = 200.;
const DAMPING: f32 = 0.95;
const EPS: f32 = 0.0001;
const DT: f32 = 1.;

const PARTICLE_SIZE: f32 = 2.0;

struct Particle {
    pos: Vec2,
    vel: Vec2,
    color: usize,
}

struct Life {
    particles: Vec<Particle>,
    att_matrix: Vec<Vec<f32>>,
}

impl Life {
    fn new() -> Self {
        Life {
            particles: Self::create_particles(),
            att_matrix: Self::create_attraction_matrix(),
        }
    }

    fn create_attraction_matrix() -> Vec<Vec<f32>> {
            let file = "./matrix/chaos.txt";

        return Self::read_file(file);
    }

    fn read_file(name: &str) -> Vec<Vec<f32>> {
        let mut matrix = vec![];

        let lines: Vec<String> = fs::read_to_string(name)
            .expect("error")
            .lines()
            .map(|s| s.to_string())
            .collect();

        for l in &lines {
            let numbers: Vec<f32> = l
                .split(",")
                .map(|i| i.parse::<f32>().unwrap())
                .collect();
            matrix.push(numbers);
        }
    
        matrix
    }

    fn create_particles() -> Vec<Particle> {
        let mut particles = vec![];
        let colors = vec![R, G, B];

        for _i in 0..NUM {
            let x = RandomRange::gen_range(0., screen_width());
            let y = RandomRange::gen_range(0., screen_height());

            let vx = RandomRange::gen_range(0., 0.);
            let vy = RandomRange::gen_range(0., 0.);

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

    fn draw(&self) {
        for p in &self.particles {
            let color = Self::get_color(p.color);
            // println!("{} {} {}", p.pos.x, p.pos.y, p.color);
            draw_circle(p.pos.x, p.pos.y, PARTICLE_SIZE, color);
        }
    }

    fn mov(&mut self) {
        let len = self.particles.len();

        dbg!(&self.att_matrix);
        for i in 0..len {
            let pos = self.particles[i].pos;
            let color = self.particles[i].color;

            let mut sum_force = vec2(0., 0.);

            for j in 0..len {
                if i == j {
                    continue;
                }

                let n = &self.particles[j];
                let a = self.att_matrix[color][n.color];
                let dis = n.pos - pos;
                let r = dis.length();

                if r > RADIUS {
                    continue;
                }
                if r < EPS {
                    continue;
                }

                let f = a / (r * r);
                sum_force += dis * f;
            }

            let p = &mut self.particles[i];
            p.vel += sum_force * DT;
            p.vel *= DAMPING;
            p.pos += p.vel;

            Self::check_bouncing(p);
        }
    }

    fn check_bouncing(p: &mut Particle) {
        if p.pos.x <= 0. || p.pos.x >= screen_width() {
            p.pos.x = p.pos.x.clamp(0., screen_width());
            p.vel.x *= -0.8;
        }

        if p.pos.y <= 0. || p.pos.y >= screen_height() {
            p.pos.y = p.pos.y.clamp(0., screen_height());
            p.vel.y *= -0.8;
        }
    }
}

#[macroquad::main("ParticleLife")]
async fn main() {
    let mut life = Life::new();

    loop {
        clear_background(BLACK);
        life.draw();
        life.mov();

        next_frame().await;
    }
}
