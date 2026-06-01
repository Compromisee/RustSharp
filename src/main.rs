use macroquad::prelude::*;

pub mod core;
pub mod render;

use crate::core::state::{GameState, GamePhase, MatchPhase};
use crate::render::shader::{CRT_VERTEX, CRT_FRAGMENT};
use crate::render::assets::GameAssets;

#[macroquad::main("SHARP")]
async fn main() {
    let mut state = GameState::new();
    let assets = GameAssets::load().await;

    let render_target = render_target(800, 600);
    render_target.texture.set_filter(FilterMode::Nearest);
    
    let material = load_material(
        ShaderSource::Glsl {
            vertex: CRT_VERTEX,
            fragment: CRT_FRAGMENT,
        },
        MaterialParams {
            ..Default::default()
        },
    ).unwrap();

    loop {
        // --- LOGIC ---
        if let GamePhase::RunActive(ref mut match_state) = state.phase {
            if match_state.phase == MatchPhase::CheatCallWindow {
                if is_key_pressed(KeyCode::Space) {
                    if match_state.cpu_cheated_this_round {
                        state.run.coins += 15;
                        match_state.cpu_score = match_state.cpu_score.saturating_sub(1);
                    } else {
                        state.run.credibility = state.run.credibility.saturating_sub(1);
                    }
                    match_state.phase = MatchPhase::RoundEnd;
                } else if is_key_pressed(KeyCode::Enter) {
                    match_state.phase = MatchPhase::RoundEnd;
                }
            } else if match_state.phase == MatchPhase::MatchOver {
                if state.run.credibility == 0 {
                    state.phase = GamePhase::GameOver;
                } else if state.run.match_index == 3 && match_state.player_score >= 12 {
                    state.run.gems += 2;
                    state.run.match_index = 0;
                    state.run.current_round += 1;
                    if state.run.current_round > 10 {
                        state.phase = GamePhase::Victory;
                    } else {
                        state.phase = GamePhase::Shop;
                    }
                } else if match_state.player_score >= 12 {
                    state.phase = GamePhase::Shop;
                } else {
                    state.run.credibility = state.run.credibility.saturating_sub(1);
                    if state.run.credibility == 0 {
                        state.phase = GamePhase::GameOver;
                    } else {
                        state.phase = GamePhase::Shop;
                    }
                }
            }
        }

        // --- DRAW TO RENDER TARGET ---
        set_camera(&Camera2D {
            zoom: vec2(1.0 / 400.0, 1.0 / 300.0),
            target: vec2(400.0, 300.0),
            render_target: Some(render_target.clone()),
            ..Default::default()
        });

        render::table::update_and_draw(&mut state, &assets);
        
        if let GamePhase::RunActive(ref mut match_state) = state.phase {
            if match_state.phase == MatchPhase::CheatCallWindow {
                draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::new(0.0, 0.0, 0.0, 0.5));
                render::ui::draw_text_custom("CALL CHEAT? [SPACE] to Call, [ENTER] to Ignore", 100.0, 300.0, 30.0, RED, &assets);
            }
        }

        // --- DRAW RENDER TARGET TO SCREEN WITH CRT SHADER ---
        set_default_camera();
        clear_background(BLACK);

        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();

        next_frame().await;
    }
}
