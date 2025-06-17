use yew::prelude::*;
use crate::audio::AudioEngine;
use crate::patterns::Pattern;
use wasm_bindgen::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PlayerProps {
    #[prop_or_default]
    pub pattern: Option<Pattern>,
}

#[function_component(Player)]
pub fn player(props: &PlayerProps) -> Html {
    let is_playing = use_state(|| false);
    let audio_engine = use_state(|| AudioEngine::new().unwrap());
    
    let on_play = {
        let is_playing = is_playing.clone();
        let audio_engine = audio_engine.clone();
        let pattern = props.pattern.clone();
        
        Callback::from(move |_| {
            if let Some(pattern) = &pattern {
                for event in &pattern.events {
                    let _ = audio_engine.play_sample(&event.sample, pattern.gain, pattern.pan);
                }
            }
            is_playing.set(!*is_playing);
        })
    };
    
    html! {
        <div class="player">
            <button
                class="play-button"
                onclick={on_play}
            >
                {if *is_playing { "Stop" } else { "Play" }}
            </button>
        </div>
    }
} 