use yew::prelude::*;
use crate::components::editor::Editor;
use crate::components::player::Player;
use crate::components::metronome::Metronome;
use crate::components::sample_library::SampleLibrary;
use crate::components::visualizer::Visualizer;
use crate::patterns::Pattern;
use crate::audio::AudioEngine;

#[function_component(App)]
pub fn app() -> Html {
    let current_pattern = use_state(Pattern::new);
    let audio_engine = use_state(|| AudioEngine::new().unwrap());
    let bpm = use_state(|| 120);
    
    let on_pattern_change = {
        let current_pattern = current_pattern.clone();
        Callback::from(move |pattern: Pattern| {
            current_pattern.set(pattern);
        })
    };
    
    let on_sample_added = {
        let audio_engine = audio_engine.clone();
        Callback::from(move |_| {
            // Sample was added to the audio engine
        })
    };
    
    html! {
        <div class="app">
            <header>
                <h1>{"SongSpark"}</h1>
                <p>{"Live coding music in your browser"}</p>
            </header>
            <main>
                <div class="controls">
                    <Metronome bpm={*bpm} />
                    <Player pattern={(*current_pattern).clone()} />
                </div>
                <div class="workspace">
                    <div class="editor-section">
                        <Editor on_pattern_change={on_pattern_change} />
                        <SampleLibrary on_sample_added={on_sample_added} />
                    </div>
                    <div class="visualization-section">
                        <Visualizer audio_engine={(*audio_engine).clone()} />
                    </div>
                </div>
            </main>
            <footer>
                <p>{"Built with Rust and WebAssembly"}</p>
            </footer>
        </div>
    }
} 