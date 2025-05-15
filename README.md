
---

# 🪨📄✂️🦎🖖 Rock, Paper, Scissors, Lizard, Spock: The Ultimate Cosmic Showdown!

Welcome to the **most epic Rock, Paper, Scissors, Lizard, Spock (RPSLS) game ever coded in Rust**! 🚀
This isn't your grandma’s RPS—this is a **galactic battle** with power-ups, sick beats, and sound effects that'll have you laughing and fist-pumping.

> “Scissors cuts Paper, Paper covers Rock, Rock crushes Lizard, Lizard poisons Spock,
> Spock smashes Scissors, Scissors decapitates Lizard, Lizard eats Paper,
> Paper disproves Spock, Spock vaporizes Rock, and Rock crushes Scissors.”
> — *Sheldon Cooper, probably*

---

## 🎉 Why This Game Rules the Universe

* **Classic RPS, But Extra**: Lizard and Spock added for maximum chaos (inspired by *The Big Bang Theory*). 🖖
* **Power-Ups for Days**: Unlock `Double Points` or `Force Draw` after every 3 wins. 💪
* **Audio That Slaps**: Theme music + win/loss tones = epic vibes. 🎵🔊
* **Colorful Console**: Your terminal's now a cosmic disco, thanks to `colored`. 🌈
* **Smart AI**: Choose easy for chill, or hard to face an adaptive mind-reading computer. 🤖
* **Rust-Powered**: Safe, fast, and furious — the Rust way. 🦀

---

## 🎮 How to Play

* **Pick a Move**: Type `rock`, `paper`, `scissors`, `lizard`, or `spock`.
* **Power-Up**: Type `powerup` after 3 wins to use a special move.
* **Quit in Style**: Type `quit` to exit and flex your stats.
* **Jam to the Tunes**: Background music loops + victory/loss tones keep it spicy.

---

## 🛠️ Setup (Get This Party Started)

### ✅ Prerequisites

* **Rust**: Install via [rustup](https://rustup.rs).
* **Audio Device**: Plug in your speakers/headphones.
* **MP3 File**: Get a `theme.mp3` for background music.

---

### 📁 Project Structure

```
rps_game/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── assets/
│       └── theme.mp3
```

---

### 📥 Installation

```bash
# Clone the repo
git clone https://github.com/your-username/rps_game.git
cd rps_game

# Or manually create the folder structure above
```

1. **Drop Your Music**
   Place `theme.mp3` in `src/assets/`.
   Example path:
   `C:\Users\ALEMAX CYBER\rps_game\src\assets\theme.mp3`

2. **Cargo.toml Dependencies**

```toml
[package]
name = "rps_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
rodio = "0.17.3"
colored = "2.1.0"
```

3. **Run the Game**

```bash
cargo run
```

🧠 Choose a difficulty when prompted:
Type `easy` or `hard`.

---

## 🧰 Troubleshooting

* ❌ **No Music?** Check the path and file format of `theme.mp3`.
* ❌ **No Tones?** Ensure audio output is active.
* 💥 **Crash on Launch?** Run `cargo build` to catch compile issues.
* 🔇 **No Sound on Windows?** Update your audio drivers in Device Manager.

---

## 🎵 Audio Awesomeness

* 🎶 **Theme Music**: MP3 loops at 50% volume.
* 🏆 **Win Tone**: 440 Hz beep (0.5s) — victory anthem.
* 😢 **Loss Tone**: 220 Hz buzz (0.5s) — shame signal.
* 🎛️ **All tones generated in Rust — no extra files needed!**

---

## 🤓 Nerdy Details

* **Language**: Rust 🦀
* **Crates Used**:

  * `rand`: AI randomness
  * `rodio`: Audio playback
  * `colored`: Terminal color magic
* **AI**:

  * `easy`: Randomized
  * `hard`: Tracks and counters your patterns
* **Power-Ups**:

  * `Double Points`: Double score on win
  * `Force Draw`: Force a tie

---

## 😜 Fun Facts

* Inspired by *The Big Bang Theory*'s nerdy showdowns
* 440 Hz = Musical A note = Victory theme 🎼
* 220 Hz = Depressing pizza-drop tone 🍕
* No MP3? Rickroll yourself or grab one from [(https://ncs.io/)]

---

## 🚀 Future Plans

* 🔇 Add `mute` command
* 📢 Add `draw tone` (maybe 330 Hz “meh” sound)
* 🌐 Multiplayer over TCP (terminal-based)
* 🔊 Fancy tones with chords & memes

---

## 🙌 Credits

* **You**: For playing! 😎
* **Rust Community**: For building awesome tools.
* **https://ncs.io/**: For free music and effects.
* **Me**: For writing this nonsense. 😂

---

## 📜 License

**Do Whatever, Just Don’t Break the Universe License**
Remix, reuse, or use this as your spaceship's entertainment system — just credit the OG code. ✌️

---

Bazinga! Now go **crush that computer** and let the tones of victory ring!
Got bugs or ideas? Open an issue or hit me up and also be ready to buy me coffee ,im sharing too much lol.
🪨📄✂️🦎🖖

---

