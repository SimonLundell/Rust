use ggez::event;
use ggez::input::keyboard::{KeyCode};
use ggez::graphics::{self, Canvas};
use ggez::{Context, GameResult};
use ggez::conf::*;
use mint;
use rand::Rng;

const WINDOW_W: f32 = 400.0;
const WINDOW_H: f32 = 700.0;

const PLAYER_X: f32 = 180.0;
const PLAYER_Y: f32 = 20.0;
const PLAYER_WALKING_SPEED: f32 = 2.0;
const HITAREA_W: f32 = 128.0;
const HITAREA_H: f32 = 32.0;
const PLAYER_HOLDING_SPEED: f32 = 1.0;
const PLAYER_HOLDING_TIME_MIN: f32 = 40.0;
const PLAYER_HOLDING_TIME_MAX: f32 = 60.0;

const SMASHABLE_X_LEFT: f32 = 135.0;
const SMASHABLE_X_RIGHT: f32 = 255.0;
const SMASHABLE_SPAWN_FACTOR: f32 = 550.0;
const SMASHABLES_PER_SCREEN: u32 = 13;
const SMASHABLE_W: f32 = 64.0;

struct Smashable {
    x: f32,
    y: f32,
    active: bool,
    sprite: graphics::Image,
}

impl Smashable {
    fn new(ctx: &mut Context) -> Smashable {
        let mut rng= rand::thread_rng();
        let y: f32= rng.gen::<f32>() * SMASHABLE_SPAWN_FACTOR + 100.0;
        let x: f32;
        let ltr: bool = rng.gen();
        match ltr {
            true => { x = SMASHABLE_X_LEFT}
            false => { x = SMASHABLE_X_RIGHT}
        }
        let sprite = graphics::Image::from_path(ctx, "/hydrant-sprite.png").unwrap();
     
        Smashable { x: x, y: y, active: true, sprite: sprite }
    }

    pub fn draw(&mut self, canvas: &mut Canvas) -> GameResult {
        if self.active {
            let point: mint::Point2<f32> = mint::Point2{x: self.x, y: self.y};
            graphics::Canvas::draw(canvas, &self.sprite, graphics::DrawParam::default().dest(point));
        }
        Ok(())
    }
}

struct Player {
    x: f32,
    y: f32,
    sprite: graphics::Image,
    hitarea: graphics::Image,
    h_x: f32,
    h_y: f32,
    h_w: f32,
    h_h: f32,
    holding: f32,
}

impl Player {
    fn new(ctx: &mut Context) -> Player {
        Player {
        x: PLAYER_X,
        y: PLAYER_Y,
        sprite: graphics::Image::from_path(ctx, "/beyonce.png").unwrap(),
        hitarea: graphics::Image::from_path(ctx, "/swing.png").unwrap(),
        h_x: PLAYER_X,
        h_y: PLAYER_X + HITAREA_H,
        h_w: HITAREA_W,
        h_h: HITAREA_H,
        holding: 0.0,
        }
    }

    pub fn update(&mut self) {
        if self.holding == 0.0 {
            self.y = self.y % WINDOW_H + PLAYER_WALKING_SPEED;
            self.h_y = self.y + HITAREA_H;
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas) -> GameResult {
        let dest_point = mint::Point2{x: self.x, y: self.y};
        graphics::Canvas::draw(canvas, &self.sprite, graphics::DrawParam::default().dest(dest_point));
        
        let mut point = dest_point.clone();
        point.x -= 15.0;
        point.y += 5.0;
        // let hit_point: mint::Point2<f32> = mint::Point2{x: self.h_x, y: self.h_y};
        if self.holding > PLAYER_HOLDING_TIME_MIN {
            graphics::Canvas::draw(canvas, &self.hitarea, graphics::DrawParam::default().dest(point));
        }
        Ok(())
    }

    pub fn hold(&mut self) {
        if self.holding > 0.0 {
            self.holding += PLAYER_HOLDING_SPEED;
            if self.holding > PLAYER_HOLDING_TIME_MAX {
                self.unhold();
            }
        }
        else {
            self.holding = 0.1;
        }
    }
    
    pub fn unhold(&mut self) {
        self.holding = 0.0;
    }
}

struct MainState {
    player: Player,
    smashables: Vec<Smashable>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut smashables = vec![];
        for _ in 0..SMASHABLES_PER_SCREEN {
            smashables.push(Smashable::new(ctx));
        }
        let s = MainState { 
            player: Player::new(ctx),
            smashables: smashables
        };
        Ok(s)
    }

    pub fn collision(&mut self) {
        if self.player.holding > PLAYER_HOLDING_TIME_MIN {
            for s in self.smashables.iter_mut() {
                if s.active {
                    if self.player.h_x < s.x + SMASHABLE_W &&
                    self.player.h_x + self.player.h_w > s.x &&
                    self.player.h_y < s.y + SMASHABLE_W &&
                    self.player.h_y + self.player.h_h > s.y {
                        s.active = false;
                    }
                }
            }
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if _ctx.keyboard.is_key_pressed(KeyCode::Space)
        {
            self.player.hold();
        }
        else if _ctx.keyboard.is_key_just_released(KeyCode::Space) {
            self.collision();
            self.player.unhold();
        }
        self.player.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        for s in self.smashables.iter_mut() {
            s.draw(&mut canvas)?;
        }

        self.player.draw(&mut canvas)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}


pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
    .window_mode(WindowMode::default().dimensions(WINDOW_W, WINDOW_H));
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Beyonce Brawles");
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}