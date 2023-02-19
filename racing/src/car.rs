use ggez::graphics::{self, Canvas, DrawParam, Color, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use mint::{Point2};

const CAR_W: f32 = 30.0;
const CAR_L: f32 = 60.0;
const HALF_CAR_W: f32 = CAR_W / 2.0;
const WHEELBASE: f32 = 40.0;
const REAR_OVERHANG: f32 = (CAR_L - WHEELBASE) / 2.0;
const REAR_AXLE_FROM_FRONT: f32 = CAR_L - REAR_OVERHANG;
const CAR_VEL: f32 = 0.01;
const FRICTION: f32 = CAR_VEL / 10.0;

pub struct Car {
    pos: Point2<f32>,
    vertices : Vec<Point2<f32>>,
    speed: f32,
    steering: f32,
    yaw: f32,
}

#[allow(dead_code)]
impl Car {
    pub fn new(pos: Point2<f32>, vertices: Vec<Point2<f32>>, speed: f32, steering: f32, yaw: f32) -> Car {
        Car{pos, vertices, speed, steering, yaw}   
    }

    pub fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let mb = &mut MeshBuilder::new();
        mb.line(&self.vertices, 2.0, Color::RED)?;
        let car = Mesh::from_data(ctx, mb.build());
        graphics::Canvas::draw(canvas, &car, DrawParam::default());
        Ok(())
    }
    
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed += speed;

        if self.speed > 2.5 {
            self.speed = 2.5;
        }
        else if self.speed < -1.25 {
            self.speed = -1.25;
        }
    }

    pub fn set_steering(&mut self, steering: f32) {
        if (steering > 0.0 && self.steering < 3.14 / 3.0) || (steering < 0.0 && self.steering > -3.14 / 3.0) {
            self.steering += steering;
        }
        else if steering == 0.0 {
            self.steering = 0.0;
        }

    }

    pub fn get_steering(&self) -> f32 {
        self.steering
    }

    pub fn set_yaw(&mut self) {
        let mut angular_vel: f32 = 0.01;
        if self.get_speed() < 0.0 {
            angular_vel = -0.01;
        }
        
        if self.steering < 0.0 {
            self.yaw -= angular_vel;
        }
        else if self.steering > 0.0 {
            self.yaw += angular_vel;
        }
    }

    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }

    pub fn apply_friction(&mut self) {
        if self.get_speed() > 0.0 {
            self.set_speed(-FRICTION);
        }
        else if self.get_speed() < 0.0 {
            self.set_speed(FRICTION);
        }
    }

    fn move_points(&mut self, speed: f32) {
        self.pos.x -= speed * f32::sin(-self.yaw);
        self.pos.y -= speed * f32::cos(-self.yaw);

        self.vertices[0] = Point2{x: self.pos.x - HALF_CAR_W, y: self.pos.y + REAR_OVERHANG};
        self.vertices[1] = Point2{x: self.pos.x + HALF_CAR_W, y: self.pos.y + REAR_OVERHANG};
        self.vertices[2] = Point2{x: self.pos.x + HALF_CAR_W, y: self.pos.y - REAR_AXLE_FROM_FRONT};
        self.vertices[3] = Point2{x: self.pos.x - HALF_CAR_W, y: self.pos.y - REAR_AXLE_FROM_FRONT};
        self.vertices[4] = self.vertices[0].clone();
    }

    fn rotate_points(&mut self, angle: f32) {
        for i in 0..self.vertices.len() - 1 {
            // Clone to not modify the value to early, causing rectangle to shrink
            let temp_x = self.vertices[i].x.clone();
            self.vertices[i].x = self.pos.x + (self.vertices[i].x - self.pos.x) * f32::cos(angle) - (self.vertices[i].y - self.pos.y) * f32::sin(angle);
            self.vertices[i].y = self.pos.y + (temp_x - self.pos.x) * f32::sin(angle) + (self.vertices[i].y - self.pos.y) * f32::cos(angle);
        }

        self.vertices[4] = self.vertices[0].clone();
    }

    pub fn update_position(&mut self) {
        self.move_points(self.speed);
        self.rotate_points(self.yaw);
    }
}