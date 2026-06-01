use macroquad::prelude::*;
use crate::core::cards::{Card, Suit};
use crate::render::assets::GameAssets;

pub fn draw_text_custom(text: &str, x: f32, y: f32, size: f32, color: Color, assets: &GameAssets) {
    let font = assets.font.as_ref();
    let params = TextParams {
        font,
        font_size: size as u16,
        color,
        ..Default::default()
    };
    draw_text_ex(text, x, y, params);
}

pub fn draw_card(x: f32, y: f32, card: &Card, selected: bool, assets: &GameAssets) -> bool {
    let card_w = 80.0;
    let card_h = 120.0;
    
    let y_offset = if selected { -20.0 } else { 0.0 };
    let final_y = y + y_offset;
    
    let rect = Rect::new(x, final_y, card_w, card_h);
    
    if let Some(tex) = &assets.cards_tex {
        // Assume 4 rows (suits), 10 cols (ranks)
        let suit_idx = match card.suit {
            Suit::Iron => 0,
            Suit::Ember => 1,
            Suit::Bone => 2,
            Suit::Glass => 3,
        };
        let rank_idx = card.rank.saturating_sub(1) as usize;
        
        let src_rect = Rect::new(
            rank_idx as f32 * (tex.width() / 10.0),
            suit_idx as f32 * (tex.height() / 4.0),
            tex.width() / 10.0,
            tex.height() / 4.0
        );
        
        draw_texture_ex(tex, rect.x, rect.y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(rect.w, rect.h)),
            source: Some(src_rect),
            ..Default::default()
        });
    } else {
        // Fallback procedural
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);
        
        let suit_color = match card.suit {
            Suit::Iron => DARKGRAY,
            Suit::Ember => RED,
            Suit::Bone => LIGHTGRAY,
            Suit::Glass => BLUE,
        };
        
        let suit_str = match card.suit {
            Suit::Iron => "Irn",
            Suit::Ember => "Emb",
            Suit::Bone => "Bne",
            Suit::Glass => "Gls",
        };
        
        draw_text_custom(&format!("{}", card.rank), rect.x + 5.0, rect.y + 20.0, 20.0, suit_color, assets);
        draw_text_custom(suit_str, rect.x + 5.0, rect.y + 45.0, 20.0, suit_color, assets);
    }
    
    let mouse = mouse_position();
    let mut clicked = false;
    if rect.contains(mouse.into()) {
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, YELLOW);
        if is_mouse_button_pressed(MouseButton::Left) {
            clicked = true;
        }
    }
    
    clicked
}

pub fn draw_card_back(x: f32, y: f32, assets: &GameAssets) {
    let card_w = 80.0;
    let card_h = 120.0;
    if let Some(tex) = &assets.card_back_tex {
        draw_texture_ex(tex, x, y, WHITE, DrawTextureParams {
            dest_size: Some(vec2(card_w, card_h)),
            ..Default::default()
        });
    } else {
        draw_rectangle(x, y, card_w, card_h, DARKBLUE);
        draw_rectangle_lines(x, y, card_w, card_h, 2.0, BLACK);
        draw_text_custom("SHARP", x + 10.0, y + 60.0, 20.0, WHITE, assets);
    }
}

pub fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str, assets: &GameAssets) -> bool {
    let rect = Rect::new(x, y, w, h);
    let mouse = mouse_position();
    
    let hover = rect.contains(mouse.into());
    let bg_color = if hover { DARKGRAY } else { GRAY };
    
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, WHITE);
    
    draw_text_custom(text, x + 10.0, y + h/2.0 + 5.0, 20.0, WHITE, assets);
    
    hover && is_mouse_button_pressed(MouseButton::Left)
}
