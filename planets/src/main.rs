use macroquad::{prelude::*, rand::RandomRange};

#[macroquad::main("Planets")]
async fn main() {

    let mut planets = vec![];

    for i in 0..200{
        planets.push(Planet::new(i as f32 * 5.));
    }

    loop {
        clear_background(BLACK);

        for p in &mut planets {
            p.draw();
            p.draw_history();
        }

        next_frame().await;
    }
}

struct Planet {
    size: f32,
    radius: f32,
    angle: f32,
    color: Color,
    history: Vec<Vec2>,
    a: f32,
    b: f32,
    speed: f32
}

impl Planet {
    fn new(radius: f32) -> Self {
        Planet {
            size: RandomRange::gen_range(10., 15.),
            radius: radius,
            color: Color {
                r: RandomRange::gen_range(0., 1.),
                g: RandomRange::gen_range(0., 1.),
                b: RandomRange::gen_range(0., 1.),
                a: 1.
            },
            angle: RandomRange::gen_range(0., 360.),
            history: vec![],
            a: RandomRange::gen_range(0.4,0.8),
            b: RandomRange::gen_range(0.4,0.8),
            speed: RandomRange::gen_range(2., 6.)
        }
    }

    fn draw(&mut self) {
        let cx = 500.;
        let cy = 500.; 
        
        let px = self.radius *
            self.a *
            self.angle.to_radians().cos();

        let py = self.radius * 
            self.b *
            self.angle.to_radians().sin();


        draw_circle(px +cx, py +cy, self.size / 2.,self.color);

        self.angle += self.speed;

        self.history.push(vec2(px + cx, py + cy));
    }

    fn draw_history(&self) {
        if self.history.len() <=5 {
            return;
        }

        for i in 4..self.history.len() - 1 {
            if i >= 300 {
                return;
            }

            let p1 = self.history[i];
            let p2 = self.history[i+1];

            draw_line(
                p1.x,
                p1.y,
                p2.x, p2.y, 1., self.color);

        }
    }

}
