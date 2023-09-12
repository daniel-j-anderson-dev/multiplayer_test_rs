use std::{
    io::{Write, Read},
    net::TcpStream, sync::{Arc, Mutex, PoisonError},
};

use anyhow::anyhow;

use thread_pool::ThreadPool;
use game_state::{GameState, player::Player};

fn main() -> Result<(), anyhow::Error> {
    const LOCALHOST: &'static str = "127.0.0.1:8000";

    let listener = std::net::TcpListener::bind(LOCALHOST)?;

    let thread_pool = ThreadPool::new(2)?;

    let game_state = Arc::new(Mutex::new(GameState::new()));

    for (connection_id, possible_stream) in listener.incoming().enumerate() {
        let client = possible_stream?;

        let clone = game_state.clone();

        thread_pool.execute(move || {
            if let Err(error) = handle_connection(connection_id, client, clone) {
                eprintln!("Error while handling connection: {error}");
            }
        })?;
    }

    Ok(())
}

fn handle_connection(id: usize, mut client: TcpStream, server_game_state: Arc<Mutex<GameState>>) -> Result<(), anyhow::Error> {
    println!("\n------------------------------\nHandling connection {id}");

    // read the client_game_state
    let mut input_data = Vec::<u8>::new();
    match client.read_to_end(&mut input_data)? {
        0 => {
            return Err(anyhow!("Nothing read from client"));
        }
        bytes_read => {
            print!("Server received {bytes_read} bytes:");
        }
    }
    let client_game_state: GameState = serde_json::from_slice(&input_data)?;

    println!(" {client_game_state:?}\nraw: {input_data:?}");

    // update the server_game_state based on the client_game_state
    let mut server_game_state = server_game_state.lock().map_err(|error| anyhow!("{error}"))?;
    // TODO: check that the client_game_state is valid
    *server_game_state = client_game_state;

    let output_data = serde_json::to_vec(&server_game_state.clone())?;
    client.write_all(&output_data)?;
    client.shutdown(std::net::Shutdown::Write)?;

    println!("server state now matches client state\n------------------------------\n");
    Ok(())
}