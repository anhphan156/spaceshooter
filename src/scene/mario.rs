use std::{cell::RefCell, rc::Rc};

use glam::Vec2;
use raylib::{
    color::Color,
    ffi::{KeyboardKey, Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
    RaylibHandle, RaylibThread,
};

use crate::{
    asset::AssetManager,
    component::{cbbox::CBBox, cshape::CShape, ctransform::CTransform},
    entity::{entity_manager::EntityManager, Entity},
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
            let vec = transform.velocity;
            transform.position += vec * dt;
        }
    }

    fn player_movement(player: &Player) {
        let player_input = player.borrow().c_input.clone();
        let player_velocity: &mut Vec2 = &mut player.borrow_mut().c_transform.velocity;

        player_velocity.x = if player_input.left { -200.0 } else { 0.0 };

        if player_input.up {
            player_velocity.y = -1000.0;
        } else {
            player_velocity.y += 10.0;
            player_velocity.y = f32::min(300.0, player_velocity.y);
        };
        //player_velocity.y = if player_input.down { -200.0 } else { 300.0 };
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
            let e = e.borrow();
            if e.is_alive() {
                let position = e.c_transform.position;
                match e.c_shape.shape {
                    Shape::Circle(r) => {
                        d.draw_circle(position.x as i32, position.y as i32, r, e.c_shape.color)
                    }
                    //Shape::Rectangle(w, h) => d.draw_rectangle(
                    //    position.x as i32 - w as i32,
                    //    position.y as i32 - h as i32,
                    //    w as i32 * 2,
                    //    h as i32 * 2,
                    //    e.c_shape.color,
                    //),
                    Shape::Rectangle(w, h) => {
                        if let Some(t) = asset_manager.textures.get(&"ground".to_string()) {
                            let src_rec = Rectangle {
                                x: 0.0,
                                y: 0.0,
                                width: 64.0,
                                height: 64.0,
                            };
                            let dst_rec = Rectangle {
                                x: position.x,
                                y: position.y,
                                width: w * 2.0,
                                height: h * 2.0,
                            };
                            let origin = Vector2 { x: w, y: h };

                            //d.draw_rectangle(
                            //    position.x as i32 - w as i32,
                            //    position.y as i32 - h as i32,
                            //    w as i32 * 2,
                            //    h as i32 * 2,
                            //    e.c_shape.color,
                            //);
                            d.draw_texture_pro(t, src_rec, dst_rec, origin, 0.0, Color::WHITE);
                        }
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
            let player_position = player_borrowed.c_transform.position;
            let player_bbox = &mut player_borrowed.c_bbox;
            if let Shape::Rectangle(pw, ph) = player_bbox.shape {
                if let Shape::Rectangle(ew, eh) = e.borrow().c_bbox.shape {
                    let player_top = player_position.y - ph;
                    let player_bottom = player_position.y + ph;
                    let player_left = player_position.x - pw;
                    let player_right = player_position.x + pw;

                    let e_position = e.borrow().c_transform.position;
                    let e_top = e_position.y - eh;
                    let e_bottom = e_position.y + eh;
                    let e_left = e_position.x - ew;
                    let e_right = e_position.x + ew;

                    let h_collision = player_top < e_bottom && e_top < player_bottom;
                    let v_collision = player_left < e_right && e_left < player_right;
                    player_bbox.collision_axes = (h_collision, v_collision);
                    if h_collision && v_collision {
                        let dx =
                            if f32::abs(player_left - e_right) > f32::abs(e_left - player_right) {
                                f32::abs(e_left - player_right)
                            } else {
                                f32::abs(player_left - e_right)
                            };

                        let dy =
                            if f32::abs(player_top - e_bottom) > f32::abs(e_top - player_bottom) {
                                f32::abs(e_top - player_bottom)
                            } else {
                                f32::abs(player_top - e_bottom)
                            };
                        player_bbox.overlapped_shape = (dx, dy);
                        break;
                    }
                };
            };
        }
    }

    fn collision_resolution(player: &Player) {
        let player_collision = player.borrow().is_collided();
        let player_overlap = player.borrow().c_bbox.overlapped_shape;

        if player_collision {
            player.borrow_mut().c_transform.position.y -= player_overlap.1;
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

        let player_size = 30.0;
        let player = entity_manager.add_entity("Player".to_string());
        {
            let mut p = player.borrow_mut();
            p.c_transform = CTransform {
                position,
                velocity: Vec2::new(0.0, 300.0),
                ..Default::default()
            };
            p.c_shape = CShape {
                shape: Shape::Rectangle(player_size, player_size),
                //shape: Shape::Circle(player_size),
                color: Color::WHITE,
            };
            p.c_bbox = CBBox {
                shape: Shape::Rectangle(player_size, player_size),
                ..Default::default()
            };
        }

        player
    }

    fn spawn_ground(entity_manager: &mut EntityManager) {
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
                shape: Shape::Rectangle(floor_size, floor_size),
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

        //self.shoot(dt);

        if let Some(entities) = self.entity_manager.get_entities(Some("Brick".to_string())) {
            MarioScene::collision_detection(entities, &self.player);
        }
        MarioScene::collision_resolution(&self.player);

        if let Some(entities) = self.entity_manager.get_entities(None) {
            MarioScene::move_entities(entities, dt);
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
