use macroquad::prelude::*;
use crate::core::state::{GameState, GamePhase, MatchPhase, MatchState};
use crate::render::ui::{draw_card, draw_card_back, draw_button, draw_text_custom};
use crate::core::zones::ZoneCard;
use crate::render::assets::GameAssets;
use macroquad::rand::gen_range;

pub fn update_and_draw(state: &mut GameState, assets: &GameAssets) {
    if let Some(tex) = &assets.table_bg {
        draw_texture_ex(tex, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        });
    } else {
        clear_background(Color::new(0.05, 0.3, 0.1, 1.0));
    }
    
    match &mut state.phase {
        GamePhase::MainMenu => {
            draw_text_custom("SHARP", screen_width()/2.0 - 100.0, 150.0, 80.0, WHITE, assets);
            if draw_button(screen_width()/2.0 - 100.0, 300.0, 200.0, 50.0, "START RUN", assets) {
                let first_cpu = crate::core::cpu::CPUProfile::get_round_1()[0].clone();
                state.phase = GamePhase::RunActive(MatchState::new(first_cpu));
            }
        },
        GamePhase::RunActive(match_state) => {
            update_match(match_state, &mut state.run);
            draw_match(match_state, &state.run, assets);
        },
        GamePhase::Shop => {
            draw_text_custom("SHOP", 50.0, 50.0, 50.0, WHITE, assets);
            draw_text_custom(&format!("Coins: {}", state.run.coins), 50.0, 100.0, 30.0, YELLOW, assets);
            
            // Temporary generic shop display
            draw_rectangle(50.0, 150.0, 700.0, 300.0, Color::new(0.1, 0.1, 0.1, 0.8));
            draw_text_custom("Items will appear here.", 70.0, 200.0, 20.0, WHITE, assets);

            if draw_button(screen_width() - 250.0, screen_height() - 100.0, 200.0, 50.0, "NEXT MATCH", assets) {
                state.run.match_index += 1;
                let cpu = if state.run.match_index == 3 {
                    crate::core::cpu::CPUProfile::get_boss_1()
                } else {
                    crate::core::cpu::CPUProfile::get_round_1()[state.run.match_index % 3].clone()
                };
                state.phase = GamePhase::RunActive(MatchState::new(cpu));
            }
        },
        GamePhase::GameOver => {
            draw_text_custom("GAME OVER", screen_width()/2.0 - 150.0, 200.0, 60.0, RED, assets);
            if draw_button(screen_width()/2.0 - 100.0, 300.0, 200.0, 50.0, "MAIN MENU", assets) {
                state.phase = GamePhase::MainMenu;
                state.run = crate::core::state::RunState::new();
            }
        },
        GamePhase::Victory => {
            draw_text_custom("YOU WIN", screen_width()/2.0 - 120.0, 200.0, 60.0, YELLOW, assets);
            if draw_button(screen_width()/2.0 - 100.0, 300.0, 200.0, 50.0, "MAIN MENU", assets) {
                state.phase = GamePhase::MainMenu;
                state.run = crate::core::state::RunState::new();
            }
        }
    }
}

fn update_match(match_state: &mut MatchState, run: &mut crate::core::state::RunState) {
    if run.credibility == 0 {
        match_state.phase = MatchPhase::MatchOver;
        return;
    }

    match_state.match_timer += get_frame_time();

    match match_state.phase {
        MatchPhase::Draw => {
            match_state.deck.shuffle();
            match_state.player_hand = match_state.deck.draw(5);
            match_state.cpu_hand = match_state.deck.draw(5);
            match_state.phase = MatchPhase::ZoneReveal;
        },
        MatchPhase::ZoneReveal => {
            let zones = ZoneCard::get_all();
            match_state.current_zone = Some(zones[gen_range(0, zones.len() as u32) as usize].clone());
            match_state.phase = MatchPhase::SelectCards;
        },
        MatchPhase::SelectCards => {
            if match_state.player_selected.len() == 3 {
                match_state.cpu_selected = match_state.cpu_hand.iter().take(3).cloned().collect();
                
                let r: f32 = gen_range(0.0, 1.0);
                match_state.cpu_cheated_this_round = r < match_state.cpu.cheat_frequency;
                
                match_state.phase = MatchPhase::RevealAndScore;
            }
        },
        MatchPhase::RevealAndScore => {
            let p_score: u32 = match_state.player_selected.iter().map(|c| c.rank as u32).sum();
            let c_score: u32 = match_state.cpu_selected.iter().map(|c| c.rank as u32).sum();
            
            let mut actual_c_score = c_score;
            if match_state.cpu_cheated_this_round {
                actual_c_score += 5; 
            }
            
            if p_score > actual_c_score {
                match_state.player_score += 2;
                run.coins += 5;
            } else if actual_c_score > p_score {
                match_state.cpu_score += 2;
            } else {
                // Tie - no points or both 1 point, for simplicity no points.
            }
            
            match_state.phase = MatchPhase::CheatCallWindow;
        },
        MatchPhase::CheatCallWindow => {},
        MatchPhase::RoundEnd => {
            match_state.player_selected.clear();
            match_state.cpu_selected.clear();
            
            if match_state.player_score >= 12 || match_state.cpu_score >= 12 {
                match_state.phase = MatchPhase::MatchOver;
                if match_state.player_score >= 12 {
                    run.coins += match_state.cpu.reward;
                }
            } else {
                match_state.phase = MatchPhase::Draw;
            }
        },
        MatchPhase::MatchOver => {}
    }
}

fn draw_match(match_state: &mut MatchState, run: &crate::core::state::RunState, assets: &GameAssets) {
    draw_text_custom(&format!("Round: {} | Match: {} | CPU: {}", run.current_round, run.match_index + 1, match_state.cpu.name), 20.0, 30.0, 20.0, WHITE, assets);
    draw_text_custom(&format!("Coins: {} | Gems: {}", run.coins, run.gems), screen_width() - 250.0, 30.0, 20.0, YELLOW, assets);
    draw_text_custom(&format!("Credibility: {}/{}", run.credibility, run.max_credibility), screen_width() - 250.0, 60.0, 20.0, RED, assets);
    
    draw_text_custom(&format!("Player Score: {}/12", match_state.player_score), 20.0, screen_height() - 20.0, 30.0, WHITE, assets);
    draw_text_custom(&format!("CPU Score: {}/12", match_state.cpu_score), 20.0, 80.0, 30.0, ORANGE, assets);

    // CPU hand drawing with potential cheat tell
    for i in 0..5 {
        let y_pos = 100.0;
        let mut x_pos = 200.0 + i as f32 * 90.0;
        
        // Simple tell animation: vibrate if they are cheating
        if match_state.cpu_cheated_this_round && match_state.phase == MatchPhase::SelectCards {
            x_pos += (match_state.match_timer * 20.0).sin() * 2.0;
        }

        draw_card_back(x_pos, y_pos, assets);
    }
    
    if let Some(zone) = &match_state.current_zone {
        draw_rectangle(screen_width()/2.0 - 100.0, screen_height()/2.0 - 50.0, 200.0, 100.0, Color::new(0.2, 0.1, 0.3, 1.0));
        draw_text_custom(&zone.name, screen_width()/2.0 - 80.0, screen_height()/2.0 - 20.0, 20.0, WHITE, assets);
        draw_text_custom(&zone.description, screen_width()/2.0 - 80.0, screen_height()/2.0 + 10.0, 15.0, WHITE, assets);
    }

    if match_state.phase == MatchPhase::SelectCards {
        draw_text_custom("Select 3 Cards", screen_width()/2.0 - 100.0, screen_height() - 160.0, 25.0, WHITE, assets);
    }

    let mut to_select = None;
    for (i, card) in match_state.player_hand.iter().enumerate() {
        let is_selected = match_state.player_selected.contains(card);
        if draw_card(200.0 + i as f32 * 90.0, screen_height() - 140.0, card, is_selected, assets) {
            if match_state.phase == MatchPhase::SelectCards {
                to_select = Some(card.clone());
            }
        }
    }
    
    if let Some(c) = to_select {
        if match_state.player_selected.contains(&c) {
            match_state.player_selected.retain(|x| x != &c);
        } else if match_state.player_selected.len() < 3 {
            match_state.player_selected.push(c);
        }
    }
    
    if match_state.phase == MatchPhase::RevealAndScore || match_state.phase == MatchPhase::CheatCallWindow {
        for (i, card) in match_state.player_selected.iter().enumerate() {
            draw_card(screen_width()/2.0 - 150.0 + i as f32 * 90.0, screen_height()/2.0 + 60.0, card, false, assets);
        }
        for (i, card) in match_state.cpu_selected.iter().enumerate() {
            draw_card(screen_width()/2.0 - 150.0 + i as f32 * 90.0, screen_height()/2.0 - 180.0, card, false, assets);
        }
    }
}
