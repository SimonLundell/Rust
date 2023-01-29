use ggez::event;
use ggez::input::keyboard::{KeyCode};
use ggez::graphics::{self, Canvas, DrawParam, Color, Quad, Rect, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use ggez::conf::*;
use mint::{Point2};
use rand::Rng;

const WINDOW_W: f32 = 600.0;
const WINDOW_H: f32 = 800.0;
const CAR_W: f32 = 30.0;
const CAR_H: f32 = 60.0;
const START_POS: Point2<f32> = Point2{x: WINDOW_W / 2.0, y: WINDOW_H - (4.0 * CAR_H)};
const ROAD_CENTER: [Point2<f32>; 2] = [Point2{x: WINDOW_W / 2.0, y: 0.0}, Point2{x: WINDOW_W / 2.0, y: WINDOW_H}];

struct Road {
    center: [Point2<f32>; 2],
    left: Vec<[Point2<f32>; 2]>,
    right: Vec<[Point2<f32>; 2]>,
    pos: f32,
    speed: f32,
}

impl Road {
    fn new(center: [Point2<f32>; 2], left: Vec<[Point2<f32>; 2]>, right: Vec<[Point2<f32>; 2]>, pos: f32, speed: f32) -> Road {
        Road{center: center, left: left, right: right, pos: pos, speed: speed}
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let mb = &mut MeshBuilder::new();
        self.left = line_builder(10.0, 5.0, -1, self.pos);
        self.right = line_builder(10.0, 5.0, 1, self.pos);
        if self.left.len() == self.right.len() {
            for i in 0..self.left.len() {
                mb.line(&self.left[i], 2.0, Color::YELLOW)?;
                mb.line(&self.right[i], 2.0, Color::YELLOW)?;
            }
        }
        else {
            println!("Not same amount of road-markings on left/right lane");
        }
        self.pos += self.speed;
        if self.pos > 15.0 {
            self.pos = 0.0;
        }

        let line = Mesh::from_data(ctx, mb.build());
        let center_line_mesh = Mesh::new_line(ctx, &self.center, 2.0, Color::WHITE)?;
        let rect = Rect::new(WINDOW_W / 2.0 - CAR_W * 1.5, 0.0, 2.0 * (CAR_W * 1.5), WINDOW_H);
        let rect_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::BLACK)?;
        graphics::Canvas::draw(canvas, &rect_mesh, DrawParam::default());
        graphics::Canvas::draw(canvas, &line, DrawParam::default());
        graphics::Canvas::draw(canvas, &center_line_mesh, DrawParam::default());
        Ok(())
    }

    fn get_car_speed(&mut self, speed: f32) {
        self.speed = speed;
    }
}


struct Car {
    w: f32,
    h: f32,
    pos: Point2<f32>,
    speed: f32,
}

impl Car {
    fn new(w: f32, h: f32, pos: Point2<f32>, speed: f32) -> Car {
        Car{w: w, h: h, pos: pos, speed: speed}
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let rect = Rect::new(self.pos.x, self.pos.y, self.w, self.h);
        // rect.rotate(f32::to_radians(-1.0)); Rotates entire world for some reason
        let rect_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(4.0), rect, Color::RED)?;
        graphics::Canvas::draw(canvas, &rect_mesh, DrawParam::default());
        Ok(())
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}

struct MainState {
    car: Car,
    road: Road,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let pos: Point2<f32> = START_POS;
        let ego = Car::new(CAR_W, CAR_H, pos, 0.0);
        let road = Road::new(ROAD_CENTER, vec![], vec![], 0.0, 0.0);
        let state = MainState{car: ego, road: road};
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.car.pos.x -= 1.0;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.car.pos.x += 1.0;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.car.speed += 0.01;
            if self.car.speed >= 2.5 {
                self.car.speed = 2.5;
            }
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.car.speed -= 0.01;
            if self.car.speed <= -1.25 {
                self.car.speed = -1.25;
            }
        }
        self.road.get_car_speed(self.car.get_speed());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.5, 0.0, 1.0]),
        );
        self.road.draw(&mut canvas, ctx);
        self.car.draw(&mut canvas, ctx);
        canvas.finish(ctx)?;
        Ok(())
    }

}

fn line_builder(seg_length: f32, spacing: f32, side: i8, speed: f32) -> Vec<[Point2<f32>; 2]> {
    let mut dashed_line = vec![];
    let x = WINDOW_W / 2.0 + CAR_W * 1.5 * side as f32;
    let mut y = speed;

    while y < WINDOW_H {
        dashed_line.push([Point2{x: x, y: y}, Point2{x: x, y: y + seg_length}]);
        y += seg_length + spacing;
    }

    dashed_line.clone()
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("racing", "ggez")
    .window_mode(WindowMode::default().dimensions(WINDOW_W, WINDOW_H));
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Racing");
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
