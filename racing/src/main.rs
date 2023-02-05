use ggez::event;
use ggez::input::keyboard::{KeyCode};
use ggez::graphics::{self, Canvas, DrawParam, Color, Rect, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use ggez::conf::*;
use mint::{Point2};
// use rand::Rng;

const WINDOW_W: f32 = 600.0;
const WINDOW_H: f32 = 800.0;
const CAR_W: f32 = 30.0;
const CAR_H: f32 = 60.0;
const HALF_CAR_W: f32 = CAR_W / 2.0;
const HALF_CAR_H: f32 = CAR_H / 2.0;
const CAR_VEL: f32 = 0.01;
const STEERING_VEL: f32 = 0.02;
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
        self.left = self.line_builder(10.0, 5.0, -1, self.pos);
        self.right = self.line_builder(10.0, 5.0, 1, self.pos);
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

    fn line_builder(&self, seg_length: f32, spacing: f32, side: i8, speed: f32) -> Vec<[Point2<f32>; 2]> {
        let mut dashed_line = vec![];
        let x = WINDOW_W / 2.0 + CAR_W * 1.5 * side as f32;
        let mut y = speed;

        while y < WINDOW_H {
            dashed_line.push([Point2{x: x, y: y}, Point2{x: x, y: y + seg_length}]);
            y += seg_length + spacing;
        }

        dashed_line.clone()
    }
}


struct Car {
    pos: Point2<f32>,
    corners : Vec<Point2<f32>>,
    speed: f32,
    heading: f32,
}

impl Car {
    fn new(pos: Point2<f32>, corners: Vec<Point2<f32>>, speed: f32, heading: f32) -> Car {
        Car{pos: pos, corners: corners, speed: speed, heading: heading}
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let mb = &mut MeshBuilder::new();
        mb.line(&self.corners, 2.0, Color::RED)?;
        let car = Mesh::from_data(ctx, mb.build());
        graphics::Canvas::draw(canvas, &car, DrawParam::default());
        Ok(())
    }
    
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    fn set_speed(&mut self, speed: f32) {
        self.speed += speed;

        if self.speed > 2.5 {
            self.speed = 2.5;
        }
        else if self.speed < -1.25 {
            self.speed = -1.25;
        }
    }

    fn set_heading(&mut self, heading: f32) {
        self.heading += heading;
    }

    fn move_points(&mut self, direction: f32) {
        self.pos.x -= direction * f32::sin(-self.heading);
        self.pos.y -= direction * f32::cos(-self.heading);

        self.corners[0].x = self.pos.x - HALF_CAR_W;
        self.corners[0].y = self.pos.y + HALF_CAR_H;
        self.corners[1].x = self.pos.x + HALF_CAR_W; 
        self.corners[1].y = self.pos.y + HALF_CAR_H; 
        self.corners[2].x = self.pos.x + HALF_CAR_W;
        self.corners[2].y = self.pos.y - HALF_CAR_H; 
        self.corners[3].x = self.pos.x - HALF_CAR_W;
        self.corners[3].y = self.pos.y - HALF_CAR_H;
        self.corners[4] = self.corners[0].clone();
    }

    fn rotate_points(&mut self, angle: f32) {
        for i in 0..self.corners.len() - 1 {
            // Clone to not modify the value to early, causing rectangle to shrink
            let temp_x = self.corners[i].x.clone();
            self.corners[i].x = self.pos.x + (self.corners[i].x - self.pos.x) * f32::cos(angle) - (self.corners[i].y - self.pos.y) * f32::sin(angle);
            self.corners[i].y = self.pos.y + (temp_x - self.pos.x) * f32::sin(angle) + (self.corners[i].y - self.pos.y) * f32::cos(angle);
        }

        self.corners[4] = self.corners[0].clone();
    }

    fn update_position(&mut self) {
        self.move_points(self.speed);
        self.rotate_points(self.heading);
    }
}

struct MainState {
    car: Car,
    road: Road,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let pos: Point2<f32> = START_POS;
        let corners: Vec<Point2<f32>> = vec![Point2{x: pos.x - HALF_CAR_W, y: pos.y + HALF_CAR_H},
                                            Point2{x: pos.x + HALF_CAR_W, y: pos.y + HALF_CAR_H}, 
                                            Point2{x: pos.x + HALF_CAR_W, y: pos.y - HALF_CAR_H}, 
                                            Point2{x: pos.x - HALF_CAR_W, y: pos.y - HALF_CAR_H},
                                            Point2{x: pos.x - HALF_CAR_W, y: pos.y + HALF_CAR_H}];
        let ego = Car::new(pos, corners, 0.0, 0.0);
        let road = Road::new(ROAD_CENTER, vec![], vec![], 0.0, 0.0);
        let state = MainState{car: ego, road: road};
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.car.set_heading(-STEERING_VEL);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.car.set_heading(STEERING_VEL);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.car.set_speed(CAR_VEL);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            // Handbrake
            if ctx.keyboard.is_key_pressed(KeyCode::LShift) {
                self.car.set_speed(-self.car.get_speed());
            }

            self.car.set_speed(-CAR_VEL);
        }
    
        self.car.update_position();
        self.road.get_car_speed(self.car.get_speed());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.5, 0.0, 1.0]),
        );
        self.road.draw(&mut canvas, ctx)?;
        self.car.draw(&mut canvas, ctx)?;
        canvas.finish(ctx)?;
        Ok(())
    }

}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("racing", "ggez")
    .window_mode(WindowMode::default().dimensions(WINDOW_W, WINDOW_H));
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Racing");
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
