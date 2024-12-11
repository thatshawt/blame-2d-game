#![allow(dead_code)]

use macroquad::prelude::*;
use macroquad_platformer::*;
use rapier2d::prelude::*;

use crate::nalgebra::Vector2;

// use std::cell::RefCell;


trait Drawable{
    fn draw(&self, world: &World);
}

struct Player {
    name: String,
    
    velocity: Vec2,
    speed: f32,

    x_length: f32,
    y_length: f32,

    collider: Option<Actor>,
}

impl Player{
    fn get_camera(&self, world: &World, screen_x_size: f32, screen_y_size: f32) -> Camera2D{
        let player_pos = world.actor_pos(self.collider.unwrap());
        let player = self;

        let camera = Camera2D::from_display_rect(Rect::new(
            player_pos.x - screen_x_size/2.0 + player.x_length/2.0,
            player_pos.y - screen_y_size/2.0 + player.y_length/2.0,
            screen_x_size,
            screen_y_size
        ));

        return camera;
    }
}

impl Drawable for Player{
    fn draw(&self, world: &World){
        let player_pos = world.actor_pos(self.collider.unwrap());
        let player = self;

        draw_rectangle(player_pos.x, player_pos.y, player.x_length, player.y_length, YELLOW);
        draw_text(&player.name, player_pos.x, player_pos.y, 30.0, DARKGRAY);
    }
}


struct Platform{
    x_length: f32,
    y_length: f32,

    collider: Option<Solid>,
}

impl Drawable for Platform{
    fn draw(&self, world: &World){
        let collider = self.collider.unwrap();
        let position = world.solid_pos(collider);

        let platform = self;

        draw_rectangle(position.x, position.y, platform.x_length, platform.y_length, RED);
    }
}


struct PhysicsStuff{
    gravity: Vector2<f32>,

    integration_parameters: IntegrationParameters,
    
    physics_pipeline: PhysicsPipeline,
    query_pipeline: QueryPipeline,
    
    ccd_solver: CCDSolver,

    island_manager: IslandManager,

    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,

    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    
    physics_hooks: (),
    event_handler: (),
}

impl PhysicsStuff{
    fn new() -> PhysicsStuff{
        let gravity = vector![0.0, 0.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();
        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();

        return PhysicsStuff{
            gravity: gravity,
            integration_parameters: integration_parameters,
            physics_pipeline: physics_pipeline,
            island_manager: island_manager,
            broad_phase: broad_phase,
            narrow_phase: narrow_phase,
            rigid_body_set: rigid_body_set,
            collider_set: collider_set,
            impulse_joint_set: impulse_joint_set,
            multibody_joint_set: multibody_joint_set,
            ccd_solver: ccd_solver,
            query_pipeline: query_pipeline,
            physics_hooks: physics_hooks,
            event_handler: event_handler,
        };
    }

    fn step(&mut self){
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}

struct GameState{
    player: Player,
    platform: Platform,
    world: World,

    physicsStuff: PhysicsStuff,

}

fn update(mut state: &mut GameState){
    let dt = get_frame_time();

    let ref mut player = state.player;
    let ref mut world = state.world;

    player.velocity = vec2(0.0, 0.0);

    if is_key_down(KeyCode::W) {
        player.velocity.y -= 1.0;
    }
    if is_key_down(KeyCode::A) {
        player.velocity.x -= 1.0;
    }
    if is_key_down(KeyCode::S) {
        player.velocity.y += 1.0;
    }
    if is_key_down(KeyCode::D) {
        player.velocity.x += 1.0;
    }

    player.velocity = player.velocity * player.speed;

    world.move_h(player.collider.unwrap(), player.velocity.x * dt);
    world.move_v(player.collider.unwrap(), player.velocity.y * dt);

}

fn draw(state: &GameState){
    clear_background(BLACK);

    let screen_x_size = 500.0;
    let screen_y_size = 400.0;

    let ref world = state.world;
    let ref player = state.player;

    let player_pos = world.actor_pos(player.collider.unwrap());

    let camera = Player::get_camera(player, world, screen_x_size, screen_y_size);

    let ref platform = state.platform;
    let platform_pos = world.solid_pos(platform.collider.unwrap());

    // draw the world relative to the
    // world in view of the player
    set_camera(&camera);

        // draw platform
        Platform::draw(platform, world);

        // draw player
        Player::draw(player, world);

        // for entity in world{
        //     enitty.draw();
        // }

    // draw stuff that is relative to screen
    set_default_camera();

        let x_str = player_pos.x.to_string();
        let y_str = player_pos.y.to_string();
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_x_str = (mouse_x as i32).to_string();
        let mouse_y_str = (mouse_y as i32).to_string();

        draw_circle(mouse_x, mouse_y, 10.0, GRAY);

        let debug_strings = [
            format!("x: {x_str} y: {y_str}"),
            format!("mouse_x: {mouse_x_str} mouse_y: {mouse_y_str}"),
        ];
        
        let mut i = 1.0;
        for debug_str in debug_strings{
            draw_text(&debug_str, 5.0, i*20.0, 30.0, WHITE);
            i += 1.0;
        }
}

#[macroquad::main("BlameGame")]
async fn main() {
    let world = World::new();

    let player = Player{
        name: "sussy baka".to_string(),
        // position: vec2(1.0, 1.0),
        velocity: vec2(0.0, 0.0),
        speed: 100.0,
        x_length: 30.0,
        y_length: 30.0,
        collider: None
    };

    let platform = Platform{
        x_length: 100.0,
        y_length: 10.0,
    
        collider: None,
    };

    let mut state = GameState{
        player: player,
        platform: platform,
        world: world,

        physicsStuff: PhysicsStuff::new(),
    };

    let player_start_pos = Vec2{x: 10.0, y: 50.0};

    state.player.collider = Some(state.world.add_actor(
            vec2(player_start_pos.x, player_start_pos.y),
            state.player.x_length as i32,
            state.player.y_length as i32
        ));

    let platform_start_pos = Vec2{x: 0.0, y: 0.0};
    state.platform.collider = Some(state.world.add_solid(
            vec2(platform_start_pos.x, platform_start_pos.y),
            state.platform.x_length as i32,
            state.platform.y_length as i32
        ));

    
    
    let mut physicsStuff = &mut state.physicsStuff;

    physicsStuff.gravity = vector![0.0, -9.81];

    let collider = ColliderBuilder::cuboid(100.0, 0.1).build();

    physicsStuff.collider_set.insert(collider);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBuilder::dynamic()
                        .translation(vector![0.0, 10.0])
                        .build();

    let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let ball_body_handle = physicsStuff.rigid_body_set.insert(rigid_body);
    physicsStuff.collider_set.insert_with_parent(collider, ball_body_handle, &mut physicsStuff.rigid_body_set);
    
    /* Run the game loop, stepping the simulation once per frame. */
    for _ in 0..200 {
        PhysicsStuff::step(&mut physicsStuff);

        let ball_body = &physicsStuff.rigid_body_set[ball_body_handle];
        println!("Ball altitude: {}", ball_body.translation().y);
    }

    loop {
        update(&mut state);

        draw(&state);

        next_frame().await
    }
}