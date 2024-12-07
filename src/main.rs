#![allow(dead_code)]

use macroquad::prelude::*;
use macroquad_platformer::*;

// enum PlayerState{
//     NORMAL
// }

struct Player {
    name: String,
    
    // position: Vec2,
    velocity: Vec2,
    speed: f32,

    x_length: f32,
    y_length: f32,

    collider: Option<Actor>,
}

struct Platform{
    x_length: f32,
    y_length: f32,

    collider: Option<Solid>,
}

struct GameState{
    player: Player,
    platform: Platform,
    world: World,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let world = World::new();

    let player = Player{
        name: "sussy baka".to_string(),
        // position: vec2(1.0, 1.0),
        velocity: vec2(0.0, 0.0),
        speed: 50.0,
        x_length: 30.0,
        y_length: 30.0,
        collider: None
    };

    let platform = Platform{
        x_length: 100.0,
        y_length: 30.0,
    
        collider: None,
    };

    let mut state = GameState{
        player: player,
        platform: platform,
        world: world,
    };

    let player_start_pos = Vec2{x: 10.0, y: 50.0};

    state.player.collider = Some(state.world.add_actor(vec2(player_start_pos.x, player_start_pos.y), state.player.x_length as i32, state.player.y_length as i32));

    let platform_start_pos = Vec2{x: 0.0, y: 0.0};
    state.platform.collider = Some(state.world.add_solid(vec2(platform_start_pos.x, platform_start_pos.y), state.platform.x_length as i32, state.platform.y_length as i32));

    loop {
        {
            let ref mut player = state.player;
            let ref mut world = state.world;
            // update
            player.velocity = vec2(0.0, 0.0);

            if is_key_down(KeyCode::W) {
                player.velocity.y -= player.speed;
            }
            if is_key_down(KeyCode::A) {
                player.velocity.x -= player.speed;
            }
            if is_key_down(KeyCode::S) {
                player.velocity.y += player.speed;
            }
            if is_key_down(KeyCode::D) {
                player.velocity.x += player.speed;
            }

            world.move_h(player.collider.unwrap(), player.velocity.x * get_frame_time());
            world.move_v(player.collider.unwrap(), player.velocity.y * get_frame_time());

        }


        {
            // draw
            clear_background(BLACK);

            let screen_x_size = 500.0;
            let screen_y_size = 400.0;

            let ref world = state.world;
            let ref player = state.player;

            let player_pos = world.actor_pos(player.collider.unwrap());
            
            let camera = Camera2D::from_display_rect(Rect::new(
                player_pos.x - screen_x_size/2.0 + player.x_length/2.0,
                player_pos.y - screen_y_size/2.0 + player.y_length/2.0,
                screen_x_size,
                screen_y_size
            ));

            let ref platform = state.platform;
            let platform_pos = world.solid_pos(platform.collider.unwrap());
            // draw relative to player
            set_camera(&camera);

                // draw platform
                draw_rectangle(platform_pos.x, platform_pos.y, platform.x_length, platform.y_length, RED);

                // draw player
                draw_rectangle(player_pos.x, player_pos.y, player.x_length, player.y_length, YELLOW);
                draw_text(&player.name, player_pos.x, player_pos.y, 30.0, DARKGRAY);

            
            // draw stuff relative to screen
            set_default_camera();

                let x_str = player_pos.x.to_string();
                let y_str = player_pos.y.to_string();
                
                draw_text(&format!("x: {x_str}\ny:{y_str}"), 5.0, 20.0, 30.0, WHITE);
        }

        next_frame().await
    }
}