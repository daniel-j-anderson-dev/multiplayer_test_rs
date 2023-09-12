use std::{
    io::{Write, Read},
    net::TcpStream,
};

use anyhow::anyhow;

use macroquad::prelude::*;

use game_state::{GameState, player::Player};

fn window_configuration() -> Conf {
    return Conf {
        window_title: "multiplayer test".to_owned(),
        fullscreen: false,
        window_width: 400,
        window_height: 400,
        high_dpi: false,
        sample_count: 1,
        window_resizable: false,
        // platform: ,
        // icon: ,
        ..Default::default()
    };
}

#[macroquad::main(window_configuration())]
async fn main() -> Result<(), anyhow::Error> {
    const LOCALHOST: &'static str = "127.0.0.1:8000";
    let mut client_game_state = GameState::new();
    let client_player_id: usize = 0;
    // hey server, which player id do i control?
    // i need to keep the connection open instead of making a new connection every frame
    // maybe use TcpStream::read_line instead of read_to_end?
    loop {
        clear_background(WHITE);
        let mut server = TcpStream::connect(LOCALHOST)?;
        
        let server_game_state = get_server_game_state(&mut server, &client_game_state)?;
        client_game_state = server_game_state;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        let mut client_player_velocity = Vec2::ZERO;
        if is_key_down(KeyCode::W) {
            client_player_velocity.y = -1.0;
        }
        if is_key_down(KeyCode::S) {
            client_player_velocity.y = 1.0;
        }
        if is_key_down(KeyCode::A) {
            client_player_velocity.x = -1.0;
        }
        if is_key_down(KeyCode::D) {
            client_player_velocity.x = 1.0;
        }
        match client_player_id {
            0 => client_game_state.player0.velocity = client_player_velocity,
            1 => client_game_state.player1.velocity = client_player_velocity,
            _ => {}
        }

        client_game_state.update();
        client_game_state.draw();

        next_frame().await;
    }
    return Ok(());
}


// this should just always set client_player to server_player and have the logic on the server
fn get_server_game_state(server: &mut TcpStream, client_game_state: &GameState) -> Result<GameState, anyhow::Error> {
    // tell the server how the client_game_state has changed
    let output_data = serde_json::to_vec(client_game_state)?;
    server.write_all(&output_data)?;
    println!("\n------------------------------\nClient sent\n{client_game_state:?}\n");
    server.shutdown(std::net::Shutdown::Write)?;

    // read the new server_game_state
    let mut server_input = Vec::<u8>::new();
    match server.read_to_end(&mut server_input) {
        Ok(num_bytes_read) if num_bytes_read == 0 => {
            return Err(anyhow!("nothing read from the server"));
        }
        Err(error) => {
            return Err(anyhow!("Couldn't read from server: {error}"));
        }
        Ok(num_bytes_read) => {
            println!("{num_bytes_read} bytes read\n{server_input:?}\n------------------------------\n");
        }
    }
    let server_game_state = serde_json::from_slice::<GameState>(&server_input)?.into();
    return Ok(server_game_state);
}