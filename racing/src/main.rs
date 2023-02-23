use ggez::event;
use ggez::input::keyboard::{KeyCode};
use ggez::graphics::{self};
use ggez::{Context, GameResult};
use ggez::conf::*;
use mint::{Point2};
// use rand::Rng;

mod car;
mod road;

const WINDOW_W: f32 = 600.0;
const WINDOW_H: f32 = 800.0;
const CAR_W: f32 = 30.0;
const CAR_L: f32 = 60.0;
const HALF_CAR_W: f32 = CAR_W / 2.0;
const CAR_VEL: f32 = 0.01;
const STEERING_VEL: f32 = 0.02;
const START_POS: Point2<f32> = Point2{x: WINDOW_W / 2.0, y: WINDOW_H - (4.0 * CAR_L)};
const ROAD_CENTER: [Point2<f32>; 2] = [Point2{x: WINDOW_W / 2.0, y: 0.0}, Point2{x: WINDOW_W / 2.0, y: WINDOW_H}];
const WHEELBASE: f32 = 40.0;
const REAR_OVERHANG: f32 = (CAR_L - WHEELBASE) / 2.0;
const REAR_AXLE_FROM_FRONT: f32 = CAR_L - REAR_OVERHANG;


struct MainState {
    car: car::Car,
    road: road::Road,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let pos: Point2<f32> = START_POS;
        let vertices: Vec<Point2<f32>> = vec![Point2{x: pos.x - HALF_CAR_W, y: pos.y + REAR_OVERHANG},
                                            Point2{x: pos.x + HALF_CAR_W, y: pos.y + REAR_OVERHANG}, 
                                            Point2{x: pos.x + HALF_CAR_W, y: pos.y - REAR_AXLE_FROM_FRONT}, 
                                            Point2{x: pos.x - HALF_CAR_W, y: pos.y - REAR_AXLE_FROM_FRONT},
                                            Point2{x: pos.x - HALF_CAR_W, y: pos.y + REAR_OVERHANG}];
        let ego = car::Car::new(pos, vertices, 0.0, 0.0, 0.0);
        let road = road::Road::new(ROAD_CENTER, vec![], vec![], pos, 0.0);
        let state = MainState{car: ego, road: road};
        Ok(state)
    }

    fn update_car_and_road(&mut self) {
        self.car.update_position();
        self.road.set_car_speed(self.car.get_speed());
        self.road.set_car_pos(self.car.get_pos());
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.car.apply_friction();

        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            self.car.set_steering(-STEERING_VEL);
        } 
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            self.car.set_steering(STEERING_VEL);
        }
        if ctx.keyboard.is_key_just_released(KeyCode::Left) || ctx.keyboard.is_key_just_released(KeyCode::Right) {
            self.car.set_steering(0.0);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            self.car.set_speed(CAR_VEL);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            self.car.set_speed(-2.0 * CAR_VEL);

            // Handbrake
            if ctx.keyboard.is_key_pressed(KeyCode::LShift) {
                self.car.set_speed(-self.car.get_speed());
            }
        }
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            // self.road.spline(vec![1.0,4.0,2.0,3.0,5.0]);
            // self.road.b_spline_segment(vec![Point2{x: 0.0,y: 0.0}], vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0], 3, 3.0);
        }
        if ctx.keyboard.is_key_pressed(KeyCode::R) {
            self.car.reset_car(START_POS);
        }

        if self.car.get_speed() != 0.0 {
            self.car.set_yaw();
        }

        self.update_car_and_road();
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
