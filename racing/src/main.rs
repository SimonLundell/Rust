use ggez::event;
use ggez::input::keyboard::{KeyCode};
use ggez::graphics::{self, Canvas, DrawParam, Color, Quad, Rect, Mesh};
use ggez::{Context, GameResult};
use ggez::conf::*;
use mint::{Point2};
use rand::Rng;

const WINDOW_W: f32 = 600.0;
const WINDOW_H: f32 = 800.0;
const CAR_W: f32 = 30.0;
const CAR_H: f32 = 60.0;
const START_POS: Point2<f32> = Point2{x: WINDOW_W / 2.0, y: WINDOW_H - (2.0 * CAR_H)};

struct Road {
    center: [Point2<f32>; 2],
    // left: Vec<f32>,
    // right: Vec<f32>,
}

impl Road {
    fn new(center: [Point2<f32>; 2]) -> Road {
        Road{center: center}
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let line = Mesh::new_line(ctx, &self.center, 2.0, Color::WHITE)?;
        graphics::Canvas::draw(canvas, &line, DrawParam::default());
        Ok(())
    }
}

struct Car {
    w: f32,
    h: f32,
    pos: Point2<f32>,
}

impl Car {
    fn new(w: f32, h: f32, pos: Point2<f32>) -> Car {
        Car{w: w, h: h, pos: pos}
    }

    fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let mut rect = Rect::new(self.pos.x, self.pos.y, self.w, self.h);
        // rect.rotate(f32::to_radians(-1.0)); Rotates entire world for some reason
        let rect_mesh = Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(4.0), rect, Color::RED)?;
        graphics::Canvas::draw(canvas, &rect_mesh, DrawParam::default());
        Ok(())
    }
}

struct MainState {
    car: Car,
    road: Road,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let pos: Point2<f32> = START_POS;
        let road = Road::new([Point2{x: WINDOW_W / 2.0, y: 0.0}, Point2{x: WINDOW_W / 2.0, y: WINDOW_H}]);
        let ego = Car::new(CAR_W, CAR_H, pos);
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
            self.car.pos.y -= 1.0;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.car.pos.y += 1.0;
        }
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

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("racing", "ggez")
    .window_mode(WindowMode::default().dimensions(WINDOW_W, WINDOW_H));
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Racing");
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
