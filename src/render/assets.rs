use macroquad::prelude::*;

pub struct GameAssets {
    pub font: Option<Font>,
    pub cards_tex: Option<Texture2D>,
    pub card_back_tex: Option<Texture2D>,
    pub table_bg: Option<Texture2D>,
    pub characters: Option<Texture2D>,
}

impl GameAssets {
    pub async fn load() -> Self {
        let font = load_ttf_font("assets/googleicons.ttf").await.ok();
        let cards_tex = load_texture("assets/cards.png").await.ok();
        let card_back_tex = load_texture("assets/card_back.png").await.ok();
        let table_bg = load_texture("assets/table_bg.png").await.ok();
        let characters = load_texture("assets/characters.png").await.ok();
        
        if let Some(tex) = &cards_tex { tex.set_filter(FilterMode::Nearest); }
        if let Some(tex) = &card_back_tex { tex.set_filter(FilterMode::Nearest); }
        if let Some(tex) = &table_bg { tex.set_filter(FilterMode::Nearest); }
        if let Some(tex) = &characters { tex.set_filter(FilterMode::Nearest); }

        Self {
            font,
            cards_tex,
            card_back_tex,
            table_bg,
            characters,
        }
    }
}
