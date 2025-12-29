use macroquad::{prelude::*, rand::RandomRange};

const NUM: usize = 500;
const R: usize = 0;
const G: usize = 1;
const B: usize = 2;

const RADIUS: f32 = 100.;
const FRICTION: f32 = 0.5;

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
        let matrix = vec![
            // R   G   B
            vec![1., -0.5, -0.5], //R
            vec![-0.5, 1., -0.5], //G
            vec![-0.5, -0.5, 1.], //B
        ];

        return matrix;
    }

    fn create_particles() -> Vec<Particle> {
        let mut particles = vec![];
        let colors = vec![R, G, B];

        // particles.push(Particle {
        //     pos: vec2(100., 100.),
        //     vel: vec2(0.0, 0.0),
        //     color: R
        // });
        //
        // particles.push(Particle {
        //     pos: vec2(150., 150.),
        //     vel: vec2(0.0, 0.0),
        //     color: R
        // });
        //
        //
        // particles.push(Particle {
        //     pos: vec2(125., 125.),
        //     vel: vec2(0.0, 0.0),
        //     color: B
        // });

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
            draw_circle(p.pos.x, p.pos.y, 5., color)
        }
    }

    fn mov(&mut self) {
        let len = self.particles.len();

        for i in 0..len {
            let p = &self.particles[i];
            let pos = p.pos;
            let color = p.color;
            let mut new_vel = vec2(0., 0.);

            for j in 0..len {
                if i == j {
                    continue;
                }

                let n = &self.particles[j];
                let a = self.att_matrix[color][n.color];
                let f = Self::get_force(a, pos, n.pos.clone());

                new_vel += (p.vel + f) * FRICTION;
            }

            let p_mut = &mut self.particles[i];
            p_mut.vel = new_vel;
            p_mut.pos += new_vel;

            if p_mut.pos.x < 0. || p_mut.pos.x > screen_width() {
                p_mut.vel.x *= -1.;
            }

            if p_mut.pos.y < 0. || p_mut.pos.y > screen_height() {
                p_mut.vel.y *= -1.;
            }
        }
    }

    fn get_force(att: f32, particle: Vec2, neighbor: Vec2) -> Vec2 {
        let dis = neighbor - particle;
        let dis_mag = dis.length();

        if dis_mag > RADIUS || dis_mag <= 0. {
            return vec2(0., 0.);
        }

        let f = att / dis_mag;
        let fv = f * dis;

        return fv;
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
