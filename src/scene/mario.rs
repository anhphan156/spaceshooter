use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use raylib::{
    color::Color,
    ffi::{KeyboardKey, Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

use crate::{
    animation::Animation,
    asset::AssetManager,
    component::{canimation::CAnimation, cbbox::CBBox, cshape::CShape, ctransform::CTransform},
    entity::{entity_manager::EntityManager, Entity},
    physics,
    util::{
        constant::{WINDOW_HEIGHT, WINDOW_WIDTH},
        geometry::Shape,
    },
};

use super::Scene;

type Player = Rc<RefCell<Entity>>;

#[allow(dead_code)]
pub struct MarioScene {
    pub entity_manager: EntityManager,
    asset_manager: Rc<AssetManager>,
    player: Player,
    center: (i32, i32),
    offset: f32,
    cd: f32,
}

impl MarioScene {
    pub fn new(asset_manager: Rc<AssetManager>) -> MarioScene {
        let mut entity_manager = EntityManager::new();
        let center = (WINDOW_WIDTH as i32 / 2, WINDOW_HEIGHT as i32 / 2);

        let player = MarioScene::spawn_player(&mut entity_manager);
        MarioScene::spawn_ground(&mut entity_manager);

        MarioScene {
            entity_manager,
            asset_manager,
            player,
            center,
            offset: 0.0,
            cd: 0.0,
        }
    }

    fn draw_axes(&self, d: &mut RaylibDrawHandle) {
        d.draw_line(
            self.center.0,
            0,
            self.center.0,
            WINDOW_HEIGHT as i32,
            Color::RED,
        );
        d.draw_line(
            0,
            self.center.1,
            WINDOW_WIDTH as i32,
            self.center.1,
            Color::RED,
        );
    }

    fn move_entities(entities: &mut Vec<Rc<RefCell<Entity>>>, dt: f32) {
        for e in entities.iter_mut() {
            if !e.borrow().is_alive() {
                continue;
            }
            let transform = &mut e.borrow_mut().c_transform;
            transform.prev_position = transform.position.clone();
            transform.position += transform.velocity * dt;
        }
    }

    fn player_movement(player: &Player) {
        let mut player = player.borrow_mut();
        let player_input = player.c_input.clone();
        let on_ground = player.c_state.on_ground;
        let mut forward = player.c_state.forward;
        let mut animation_enabled = false;
        let player_velocity: &mut Vec2 = &mut player.c_transform.velocity;

        if player_input.left {
            player_velocity.x = -200.0;
            forward = false;
            animation_enabled = true;
        } else if player_input.right {
            player_velocity.x = 200.0;
            forward = true;
            animation_enabled = true;
        } else {
            player_velocity.x = 0.0;
        }

        if player_input.up && on_ground {
            player_velocity.y = -1000.0;
        } else {
            player_velocity.y += 10.0;
            player_velocity.y = f32::min(300.0, player_velocity.y);
        };

        player.c_state.forward = forward;
        player.c_animation.enabled = animation_enabled;
    }

    fn input_receiving(rl: &mut RaylibHandle, player: &Player) {
        let player_input = &mut player.borrow_mut().c_input;
        player_input.left = rl.is_key_down(KeyboardKey::KEY_LEFT);
        player_input.right = rl.is_key_down(KeyboardKey::KEY_RIGHT);
        player_input.up = rl.is_key_pressed(KeyboardKey::KEY_UP);
        player_input.down = rl.is_key_down(KeyboardKey::KEY_DOWN);
    }

    fn render(
        entities: &Vec<Rc<RefCell<Entity>>>,
        asset_manager: &Rc<AssetManager>,
        d: &mut RaylibDrawHandle,
    ) {
        for e in entities.iter() {
            // animation
            if e.borrow().c_animation.enabled {
                e.borrow_mut().c_animation.animation.update();
            }

            let e = e.borrow();
            if !e.is_alive() {
                continue;
            }

            let position = e.c_transform.position;
            let forward = e.c_state.forward;
            match e.c_shape.shape {
                Shape::Circle(r) => {
                    d.draw_circle(position.x as i32, position.y as i32, r, e.c_shape.color)
                }
                Shape::Rectangle(w, h) => d.draw_rectangle(
                    position.x as i32 - w as i32,
                    position.y as i32 - h as i32,
                    w as i32 * 2,
                    h as i32 * 2,
                    e.c_shape.color,
                ),
                Shape::RectText(src_w, src_h, dst_w, dst_h, texture_tag) => {
                    if let Some(t) = asset_manager.textures.get(&texture_tag.to_string()) {
                        let src_x = e.c_animation.animation.anim_frame as f32;
                        let src_rec = Rectangle {
                            x: src_x * src_w,
                            y: 0.0,
                            width: src_w * if forward { 1.0 } else { -1.0 },
                            height: src_h,
                        };
                        let dst_rec = Rectangle {
                            x: position.x,
                            y: position.y,
                            width: dst_w * 2.0,
                            height: dst_h * 2.0,
                        };
                        let origin = Vector2 { x: dst_w, y: dst_h };

                        d.draw_texture_pro(t, src_rec, dst_rec, origin, 0.0, Color::WHITE);
                    }
                }
            }
        }
    }
    fn collision_detection(entities: &mut Vec<Rc<RefCell<Entity>>>, player: &Player) {
        // Check collision of player against everything
        for e in entities.iter_mut() {
            if !e.borrow().is_alive() {
                continue;
            }

            let mut player_borrowed = player.borrow_mut();
            let player_bbox_shape = player_borrowed.c_bbox.shape.clone();

            if let Shape::Rectangle(pw, ph) = player_bbox_shape {
                if let Shape::Rectangle(ew, eh) = e.borrow().c_bbox.shape {
                    let player_position = player_borrowed.c_transform.position;
                    let prev_player_position = player_borrowed.c_transform.prev_position;
                    let e_position = e.borrow().c_transform.position;

                    let result = physics::aabb_collision_detection(
                        player_position,
                        e_position,
                        Vec2::new(pw, ph),
                        Vec2::new(ew, eh),
                    );

                    let player_bbox = &mut player_borrowed.c_bbox;
                    if result.is_collided() {
                        player_bbox.collision_axes = result.collision_axes;
                        player_bbox.overlapped_shape = result.overlapped_shape;

                        let prev_result = physics::aabb_collision_detection(
                            prev_player_position,
                            e_position,
                            Vec2::new(pw, ph),
                            Vec2::new(ew, eh),
                        );
                        player_bbox.prev_collision_axes = prev_result.collision_axes;

                        break;
                    } else {
                        player_bbox.collision_axes = (false, false);
                        player_bbox.prev_collision_axes = (false, false);
                        player_bbox.overlapped_shape = (0.0, 0.0);

                        player_borrowed.c_state.on_ground = false;
                    }
                };
            };
        }
    }

    fn collision_resolution(player: &Player) {
        let player_collision = player.borrow().is_collided();
        let prev_player_collision = player.borrow().c_bbox.prev_collision_axes;
        let player_overlap = player.borrow().c_bbox.overlapped_shape;

        if player_collision {
            let dir_x = f32::signum(player.borrow().c_transform.velocity.x);
            let dir_y = f32::signum(player.borrow().c_transform.velocity.y);
            if prev_player_collision.0 {
                // pushing vertically
                player.borrow_mut().c_transform.position.y -= player_overlap.1 * dir_y;

                if dir_y < 0.0 {
                    // this ensures player falls down when they hit their head
                    player.borrow_mut().c_transform.velocity.y *= -1.0;
                }
                if dir_y > 0.0 {
                    // jump cooldown
                    player.borrow_mut().c_state.on_ground = true;
                }
            } else if prev_player_collision.1 {
                // pushing horizontally
                player.borrow_mut().c_transform.position.x -= player_overlap.0 * dir_x;
            }
        }
    }

    #[allow(dead_code)]
    fn shoot(&mut self, dt: f32) {
        self.offset += 1.0;
        if self.cd > 0.0 {
            self.cd -= dt;
            return;
        }
        self.cd = 0.2;

        let center = Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);
        let count = 20;
        let angle = 6.28 / count as f32;
        let mut theta: f32 = 0.0;
        for _ in 0..count {
            let velocity = Vec2::new(f32::cos(theta + self.offset), f32::sin(theta + self.offset));

            let e = self.entity_manager.add_entity("ball".to_string());
            let mut e = e.borrow_mut();
            e.c_transform = CTransform {
                position: velocity + center,
                velocity: velocity * 200.0,
                ..Default::default()
            };
            e.c_shape = CShape {
                shape: Shape::Circle(5.0),
                //shape: Shape::Rectangle(25.0, 25.0),
                color: Color::WHITE,
            };
            theta += angle;
        }
    }

    fn spawn_player(entity_manager: &mut EntityManager) -> Rc<RefCell<Entity>> {
        let position = Vec2::new(WINDOW_WIDTH as f32 / 2.0, 0.0);

        let player_size = 60.0;
        let player = entity_manager.add_entity("Player".to_string());
        {
            let mut p = player.borrow_mut();
            p.c_transform = CTransform {
                position,
                velocity: Vec2::new(0.0, 300.0),
                ..Default::default()
            };
            p.c_shape = CShape {
                shape: Shape::RectText(246.0, 246.0, player_size, player_size, "mega_run"),
                color: Color::WHITE,
            };
            p.c_bbox = CBBox {
                shape: Shape::Rectangle(player_size - 15.0, player_size - 15.0),
                ..Default::default()
            };
            p.c_animation = CAnimation {
                enabled: true,
                animation: Animation::new(18, 3),
            }
        }

        player
    }

    fn spawn_ground(entity_manager: &mut EntityManager) {
        let floor_tex_size = 64.0;
        let floor_size: f32 = 50.0;
        let half_size = 25.0;
        let brick_count = WINDOW_WIDTH / floor_size as u32;

        for i in 0..brick_count {
            let e = entity_manager.add_entity("Brick".to_string());
            e.borrow_mut().c_transform = CTransform {
                position: Vec2::new(
                    i as f32 * floor_size * 2.0 + half_size,
                    WINDOW_HEIGHT as f32 - half_size,
                ),
                velocity: Vec2::new(0.0, 0.0),
                ..Default::default()
            };
            e.borrow_mut().c_shape = CShape {
                shape: Shape::RectText(
                    floor_tex_size,
                    floor_tex_size,
                    floor_size,
                    floor_size,
                    "ground",
                ),
                color: Color::RED,
            };
            e.borrow_mut().c_bbox = CBBox {
                shape: Shape::Rectangle(floor_size, floor_size),
                ..Default::default()
            };
        }

        let e = entity_manager.add_entity("Brick".to_string());
        e.borrow_mut().c_transform = CTransform {
            position: Vec2::new(
                WINDOW_WIDTH as f32 / 2.0 + half_size - 200.0,
                WINDOW_HEIGHT as f32 / 2.0 - half_size,
            ),
            velocity: Vec2::new(0.0, 0.0),
            ..Default::default()
        };
        e.borrow_mut().c_shape = CShape {
            shape: Shape::Rectangle(floor_size, floor_size),
            color: Color::RED,
        };
        e.borrow_mut().c_bbox = CBBox {
            shape: Shape::Rectangle(floor_size, floor_size),
            ..Default::default()
        };
    }
}

impl Scene for MarioScene {
    fn update(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) {
        self.entity_manager.update();
        MarioScene::input_receiving(rl, &self.player);
        MarioScene::player_movement(&self.player);

        let dt = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_fps(12, 12);
        self.draw_axes(&mut d);
        if let Some(entities) = self.entity_manager.get_entities(None) {
            MarioScene::move_entities(entities, dt);
        }

        if let Some(entities) = self.entity_manager.get_entities(Some("Brick".to_string())) {
            MarioScene::collision_detection(entities, &self.player);
        }
        MarioScene::collision_resolution(&self.player);

        if let Some(entities) = self.entity_manager.get_entities(None) {
            MarioScene::render(entities, &self.asset_manager, &mut d);
            d.draw_text(
                format!("{}", entities.len()).as_str(),
                self.center.0,
                0,
                30,
                Color::RED,
            );
        }
    }
}
