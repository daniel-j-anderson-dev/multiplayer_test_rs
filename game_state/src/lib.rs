use macroquad::{
    color::{
        BLUE,
        RED,
    },
    math::Vec2, shapes::draw_rectangle,
};

pub mod player;
use self::player::{
    Player,
    PLAYER_WIDTH,
};

pub const WINDOW_WIDTH: f32 = 400.0;
pub const WINDOW_HEIGHT: f32 = 400.0;

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug)]
pub struct GameState {
    pub player0: Player,
    pub player1: Player,
}

impl GameState {
    pub fn new() -> Self {
        let player0 = Player::new(
            0,
            Vec2::new(0.0, WINDOW_HEIGHT / 2.0),
            RED,
        );

        let player1 = Player::new(
            1,
            Vec2::new(WINDOW_WIDTH - PLAYER_WIDTH, WINDOW_HEIGHT / 2.0),
            BLUE,
        );

        GameState { player0, player1 }
    }

    pub fn update(&mut self) {
        self.player0.update();
        self.player1.update();
    }

    pub fn draw(self) {
        draw_rectangle(self.player0.boundary.x, self.player0.boundary.y, self.player0.boundary.w, self.player0.boundary.h, self.player0.color);
        draw_rectangle(self.player1.boundary.x, self.player1.boundary.y, self.player1.boundary.w, self.player1.boundary.h, self.player1.color);
    }
}