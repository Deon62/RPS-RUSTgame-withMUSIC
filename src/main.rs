use std::fs::File;
use std::io::{self, BufReader};
use std::collections::HashMap;
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle, Source};
use colored::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl Move {
    fn from_str(input: &str) -> Option<Move> {
        match input.trim().to_lowercase().as_str() {
            "rock" => Some(Move::Rock),
            "paper" => Some(Move::Paper),
            "scissors" => Some(Move::Scissors),
            "lizard" => Some(Move::Lizard),
            "spock" => Some(Move::Spock),
            _ => None,
        }
    }

    fn to_emoji(&self) -> &str {
        match self {
            Move::Rock => "🪨",
            Move::Paper => "📄",
            Move::Scissors => "✂️",
            Move::Lizard => "🦎",
            Move::Spock => "🖖",
        }
    }
}

#[derive(Debug, Clone)]
enum PowerUp {
    DoublePoints,
    ForceDraw,
}

impl ToString for PowerUp {
    fn to_string(&self) -> String {
        match self {
            PowerUp::DoublePoints => "Double Points".purple().to_string(),
            PowerUp::ForceDraw => "Force Draw".cyan().to_string(),
        }
    }
}

struct GameState {
    player_score: u32,
    computer_score: u32,
    draws: u32,
    player_moves: Vec<Move>,
    power_ups: Vec<PowerUp>,
    streak: u32,
    _music_stream: Option<OutputStream>, // Keep OutputStream alive
    music_sink: Option<Sink>, // Background music sink
    stream_handle: Option<OutputStreamHandle>, // For creating new sinks
}

fn play_music(path: &str, stream_handle: &OutputStreamHandle) -> Option<Sink> {
    println!("🎵 Attempting to play music from: {}", path);
    match Sink::try_new(stream_handle) {
        Ok(sink) => {
            match File::open(path) {
                Ok(file) => {
                    match Decoder::new(BufReader::new(file)) {
                        Ok(source) => {
                            sink.set_volume(0.5); // Lower music volume to hear tones
                            sink.append(source.repeat_infinite()); // Loop music
                            println!("🎵 Music initialized successfully. Volume set to 0.5.");
                            Some(sink)
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to decode music file: {:?}", e);
                            None
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to open music file at {}: {:?}", path, e);
                    None
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create music sink: {:?}", e);
            None
        }
    }
}

fn play_sound_effect(effect_type: &str, stream_handle: &OutputStreamHandle) {
    println!("🔊 Playing tone for {}", effect_type);
    match Sink::try_new(stream_handle) {
        Ok(sink) => {
            let frequency = if effect_type == "win" { 440.0 } else { 220.0 };
            let source = rodio::source::SineWave::new(frequency)
                .take_duration(std::time::Duration::from_millis(500));
            sink.set_volume(1.0); // Full volume for tones
            sink.append(source);
            println!("🔊 Tone initialized successfully for {}.", effect_type);
        }
        Err(e) => {
            eprintln!("❌ Failed to create tone sink: {:?}", e);
        }
    }
}

fn get_computer_move(difficulty: &str, player_moves: &[Move]) -> Move {
    if difficulty == "hard" && !player_moves.is_empty() {
        let mut move_counts = HashMap::new();
        for &mv in player_moves {
            *move_counts.entry(mv).or_insert(0) += 1;
        }
        let most_common = move_counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(mv, _)| mv)
            .unwrap_or(Move::Rock);
        match most_common {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Spock,
            Move::Scissors => Move::Rock,
            Move::Lizard => Move::Scissors,
            Move::Spock => Move::Lizard,
        }
    } else {
        match rand::thread_rng().gen_range(0..=4) {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            3 => Move::Lizard,
            _ => Move::Spock,
        }
    }
}

fn determine_winner(player: &Move, computer: &Move) -> &'static str {
    use Move::*;
    match (player, computer) {
        (a, b) if a == b => "draw",
        (Rock, Scissors) | (Rock, Lizard) |
        (Paper, Rock) | (Paper, Spock) |
        (Scissors, Paper) | (Scissors, Lizard) |
        (Lizard, Spock) | (Lizard, Paper) |
        (Spock, Scissors) | (Spock, Rock) => "player",
        _ => "computer",
    }
}

fn main() {
    // Initialize audio
    let (music_stream, stream_handle) = match OutputStream::try_default() {
        Ok((stream, handle)) => (Some(stream), Some(handle)),
        Err(e) => {
            eprintln!("❌ Failed to initialize audio output stream: {:?}", e);
            (None, None)
        }
    };

    // Path to background music
    let music_path = r"C:\Users\ALEMAX CYBER\rps_game\src\assets\music.mp3";

    // Play background music
    let music_sink = stream_handle.as_ref().and_then(|handle| play_music(music_path, handle));

    println!("{}", "🪨📄✂️🦎🖖 Welcome to Rock, Paper, Scissors, Lizard, Spock!".bold().cyan());
    println!("{}", "How to play:".bold().yellow());
    println!("{}", "- Enter a move: rock, paper, scissors, lizard, or spock.".green());
    println!("{}", "- Earn power-ups every 3 wins (Double Points or Force Draw).".green());
    println!("{}", "- Type 'powerup' to use a power-up when available.".green());
    println!("{}", "- Type 'quit' to exit.".green());

    println!("{}", "\nChoose difficulty (easy/hard):".blue().bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let difficulty = input.trim().to_lowercase();
    let difficulty = if difficulty == "hard" { "hard" } else { "easy" };

    let mut state = GameState {
        player_score: 0,
        computer_score: 0,
        draws: 0,
        player_moves: Vec::new(),
        power_ups: Vec::new(),
        streak: 0,
        _music_stream: music_stream,
        music_sink,
        stream_handle,
    };

    loop {
        println!("{}", "\n-----------------------------------".dimmed());
        println!("{}", "Your move: (rock, paper, scissors, lizard, spock)".bold());
        println!("{}", "Type 'powerup' or 'quit' anytime.".dimmed());

        if !state.power_ups.is_empty() {
            let powers: Vec<String> = state.power_ups.iter().map(|p| p.to_string()).collect();
            println!("Available power-ups: {}", powers.join(", "));
        } else {
            println!("{}", "No power-ups available. Win more rounds to earn some!".dimmed());
        }

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let trimmed_input = input.trim();

        if trimmed_input.eq_ignore_ascii_case("quit") {
            println!("{}", "\n🏁 Final Scores:".bold().magenta());
            println!(
                "You: {} | Computer: {} | Draws: {}",
                state.player_score.to_string().green(),
                state.computer_score.to_string().red(),
                state.draws.to_string().yellow()
            );
            let fav_move = state.player_moves.iter()
                .max_by_key(|&&mv| state.player_moves.iter().filter(|&&x| x == mv).count())
                .map(|mv| mv.to_emoji());
            println!("Longest Win Streak: {}", state.streak);
            println!("Favorite Move: {}", fav_move.unwrap_or("🤷"));
            println!("{}", "Thanks for playing! 👋".bold());
            break;
        }

        if trimmed_input.eq_ignore_ascii_case("powerup") {
            if state.power_ups.is_empty() {
                println!("{}", "❌ No power-ups available! Win more rounds to earn one.".red());
                continue;
            }
            let power_up = state.power_ups.pop().unwrap();
            match power_up {
                PowerUp::DoublePoints => {
                    println!("{}", "🎯 Activated Double Points for your next win!".purple().bold());
                    input.clear();
                    println!("{}", "Enter your move now:".bold());
                    io::stdin().read_line(&mut input).expect("Failed to read input");
                }
                PowerUp::ForceDraw => {
                    println!("{}", "🛑 Activated Force Draw!".cyan().bold());
                    state.draws += 1;
                    println!("{}", "🤝 It's a draw!".yellow());
                    println!(
                        "📊 Score => You: {} | Computer: {} | Draws: {}",
                        state.player_score, state.computer_score, state.draws
                    );
                    continue;
                }
            }
        }

        let player_move = match Move::from_str(&input) {
            Some(mv) => mv,
            None => {
                println!("❌ Invalid input, please try again.");
                continue;
            }
        };

        state.player_moves.push(player_move);

        let computer_move = get_computer_move(&difficulty, &state.player_moves);
        println!(
            "Computer chose: {} {}",
            computer_move.to_emoji(),
            format!("{:?}", computer_move).bold()
        );

        let mut points = 1;
        if state.power_ups.iter().any(|p| matches!(p, PowerUp::DoublePoints))
            && determine_winner(&player_move, &computer_move) == "player"
        {
            points = 2;
            state.power_ups.retain(|p| !matches!(p, PowerUp::DoublePoints));
        }

        match determine_winner(&player_move, &computer_move) {
            "player" => {
                println!("{}", "🎉 You win this round!".green().bold());
                state.player_score += points;
                state.streak += 1;
                if let Some(handle) = &state.stream_handle {
                    play_sound_effect("win", handle);
                }
                if state.player_score % 3 == 0 {
                    let new_power_up = if rand::thread_rng().gen_bool(0.5) {
                        PowerUp::DoublePoints
                    } else {
                        PowerUp::ForceDraw
                    };
                    println!("🎁 You earned a power-up: {}", new_power_up.to_string());
                    state.power_ups.push(new_power_up);
                }
            }
            "computer" => {
                println!("{}", "💻 Computer wins this round!".red().bold());
                state.computer_score += 1;
                state.streak = 0;
                if let Some(handle) = &state.stream_handle {
                    play_sound_effect("loss", handle);
                }
            }
            _ => {
                println!("{}", "🤝 It's a draw!".yellow());
                state.draws += 1;
                state.streak = 0;
            }
        }

        println!(
            "📊 Score => You: {} | Computer: {} | Draws: {}",
            state.player_score.to_string().green(),
            state.computer_score.to_string().red(),
            state.draws.to_string().yellow()
        );
    }
}