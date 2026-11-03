use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct SourceMovement {
    pub velocity: Vec3,
    pub on_ground: bool,
    pub jump_held: bool,
}

impl Default for SourceMovement {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            on_ground: false,
            jump_held: false,
        }
    }
}

const MAX_SPEED: f32 = 5.0;
const GROUND_ACCEL: f32 = 10.0;
const AIR_ACCEL: f32 = 300.0;
const AIR_MAX_WISHSPEED: f32 = 0.6;
const FRICTION: f32 = 5.0;
const GRAVITY: f32 = 20.0;
const JUMP_SPEED: f32 = 6.0;

pub fn apply_movement(
    wish_dir: Vec3,
    jump_pressed: bool,
    delta: f32,
    controller: &mut KinematicCharacterController,
    movement: &mut SourceMovement,
    grounded: bool,
) {
    movement.on_ground = grounded;

    if movement.on_ground {
        if jump_pressed {
            movement.velocity.y = JUMP_SPEED;
            movement.on_ground = false;
        } else {
            let speed = movement.velocity.length();
            if speed > 0.0 {
                let drop = speed * FRICTION * delta;
                let new_speed = (speed - drop).max(0.0);
                movement.velocity *= new_speed / speed;
            }
        }
    }

    if movement.on_ground {
        accelerate(&mut movement.velocity, wish_dir, MAX_SPEED, GROUND_ACCEL, delta);
    } else {
        air_accelerate(&mut movement.velocity, wish_dir, AIR_MAX_WISHSPEED, AIR_ACCEL, delta);
    }

    if !movement.on_ground {
        movement.velocity.y -= GRAVITY * delta;
    }

    let displacement = movement.velocity * delta;
    controller.translation = Some(displacement);
}

fn accelerate(velocity: &mut Vec3, wish_dir: Vec3, wish_speed: f32, accel: f32, delta: f32) {
    let current_speed = velocity.dot(wish_dir);
    let add_speed = wish_speed - current_speed;
    if add_speed <= 0.0 { return; }
    let accel_speed = (accel * wish_speed * delta).min(add_speed);
    *velocity += wish_dir * accel_speed;
}

fn air_accelerate(velocity: &mut Vec3, wish_dir: Vec3, wish_speed: f32, accel: f32, delta: f32) {
    let wish_spd = wish_speed.min(AIR_MAX_WISHSPEED);
    let current_speed = velocity.dot(wish_dir);
    let add_speed = wish_spd - current_speed;
    if add_speed <= 0.0 { return; }
    let accel_speed = accel * wish_spd * delta;
    let accel_speed = accel_speed.min(add_speed);
    *velocity += wish_dir * accel_speed;
}