use ggez::graphics::{self, Canvas, DrawParam, Color, Rect, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use mint::{Point2};
// use rand::Rng;

const WINDOW_W: f32 = 600.0;
const WINDOW_H: f32 = 800.0;

pub struct Road {
    center: [Point2<f32>; 2],
    left: Vec<[Point2<f32>; 2]>,
    right: Vec<[Point2<f32>; 2]>,
    car_pos: Point2<f32>,
    speed: f32,
}

impl Road {
    // Constants
    const CAR_W: f32 = 30.0;

    pub fn new(center: [Point2<f32>; 2], left: Vec<[Point2<f32>; 2]>, right: Vec<[Point2<f32>; 2]>, car_pos: Point2<f32>, speed: f32) -> Road {
        Road{center, left, right, car_pos, speed}
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let mb = &mut MeshBuilder::new();
        self.left = self.line_builder(10.0, 5.0, -1, self.speed);
        self.right = self.line_builder(10.0, 5.0, 1, self.speed);
        if self.left.len() == self.right.len() {
            for i in 0..self.left.len() {
                mb.line(&self.left[i], 2.0, Color::YELLOW)?;
                mb.line(&self.right[i], 2.0, Color::YELLOW)?;
            }
        }
        else {
            println!("Not same amount of road-markings on left/right lane");
        }
        // self.pos += self.speed;
        // if self.pos > 15.0 {
        //     self.pos = 0.0;
        // }

        let line = Mesh::from_data(ctx, mb.build());
        let center_line_mesh = Mesh::new_line(ctx, &self.center, 2.0, Color::WHITE)?;
        let rect = Rect::new(WINDOW_W / 2.0 - Self::CAR_W * 1.5, 0.0, 2.0 * (Self::CAR_W * 1.5), WINDOW_H);
        let rect_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::BLACK)?;
        graphics::Canvas::draw(canvas, &rect_mesh, DrawParam::default());
        graphics::Canvas::draw(canvas, &line, DrawParam::default());
        graphics::Canvas::draw(canvas, &center_line_mesh, DrawParam::default());
        Ok(())
    }

    pub fn set_car_speed(&mut self, speed: f32) {
        if self.car_pos.y <= 320.0 || self.car_pos.y >= 620.0 {
            self.speed += speed;
        }
        if self.speed > 15.0 {
            self.speed = 0.0;
        }
    }

    pub fn set_car_pos(&mut self, car_pos: Point2<f32>) {
        println!("{:?}", self.car_pos);
        self.car_pos = car_pos;
    }

    fn line_builder(&self, seg_length: f32, spacing: f32, side: i8, speed: f32) -> Vec<[Point2<f32>; 2]> {
        let mut dashed_line = vec![];
        let x = WINDOW_W / 2.0 + Self::CAR_W * 1.5 * side as f32;
        let mut y = speed;

        while y < WINDOW_H {
            dashed_line.push([Point2{x: x, y: y}, Point2{x: x, y: y + seg_length}]);
            y += seg_length + spacing;
        }

        dashed_line.clone()
    }

    // pub fn spline(self, mut vect: Vec<f32>) {
    //     vect.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // }
}