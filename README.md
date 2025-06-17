# SongSpark

SongSpark is an open-source, browser-based live coding platform for creating music through code. Built with Rust and WebAssembly, it offers a fast, lightweight way to craft rhythms and sounds using a simple syntax inspired by TidalCycles/Strudel. Perfect for beginners and coders alike, SongSpark lets you make music in real-time, no installation required.

## Features
- **Live Coding**: Write code and hear music instantly in your browser.
- **Simple Syntax**: Use easy patterns like `s("bd sd hh")` to create rhythms.
- **High Performance**: Powered by Rust and WebAssembly for low-latency audio.
- **Customizable Effects**: Add effects like gain or panning to shape your sounds.
- **Cross-Platform**: Runs in any modern browser with no dependencies.
- **Open Source**: MIT-licensed, welcoming contributions from the community.

## Installation
SongSpark runs in the browser, but you can build and host it locally for development.

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Trunk](https://trunkrs.dev/) for WebAssembly bundling
- A modern web browser (Chrome, Firefox, Edge)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/frangedev/SongSpark.git
   cd SongSpark
   ```
2. Install dependencies:
   ```bash
   cargo install trunk
   ```
3. Build and serve:
   ```bash
   trunk serve
   ```
4. Open `http://localhost:8080` in your browser to start coding music.

## Usage
1. **Open the Web App**: Visit the hosted SongSpark instance or run locally.
2. **Write Code**: Use the interactive editor to write patterns like:
   ```rust
   s("bd hh*2 sd").gain(0.8)
   ```
   - `s("bd")`: Plays a bass drum.
   - `hh*2`: Repeats hi-hat twice.
   - `.gain(0.8)`: Sets volume to 80%.
3. **Play and Edit**: Click the play button to hear your pattern. Edit code and refresh to update sounds.
4. **Explore**: Try patterns like `s("bd sd hh cp")` or add effects like `.pan(0.5)`.

### Example
```rust
s("bd sd hh*2 cp").gain(0.7).pan(0.3)
```
This creates a rhythm with a bass drum, snare, double hi-hat, and clap, at 70% volume, panned slightly right.

## Project Structure
- `src/`: Rust source code for the core logic, audio engine, and Yew frontend.
- `index.html`: Entry point for the WebAssembly app.
- `examples/`: Sample music patterns to get started.
- `Cargo.toml`: Rust dependencies and build configuration.

## Contributing
We welcome contributions! To get started:
1. Fork the repository.
2. Create a branch (`git checkout -b feature/your-feature`).
3. Commit changes (`git commit -m "Add your feature"`).
4. Push to your branch (`git push origin feature/your-feature`).
5. Open a pull request.

Please follow [Code of Conduct](CODE_OF_CONDUCT.md) and check [issues](https://github.com/frangedev/SongSpark/issues) for ideas.

## License
SongSpark is licensed under the [MIT License](LICENSE).

## Acknowledgments
- Inspired by [Strudel](https://strudel.cc) and [TidalCycles](https://tidalcycles.org).
- Built with [Rust](https://www.rust-lang.org), [Yew](https://yew.rs), and [cpal](https://crates.io/crates/cpal).
- Thanks to the open-source community for feedback and inspiration.

Happy coding, happy music-making!
