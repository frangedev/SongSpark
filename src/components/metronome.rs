use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorNode, GainNode};

#[derive(Properties, PartialEq)]
pub struct MetronomeProps {
    #[prop_or_default]
    pub bpm: u32,
    #[prop_or_default]
    pub is_active: bool,
}

#[function_component(Metronome)]
pub fn metronome(props: &MetronomeProps) -> Html {
    let is_playing = use_state(|| false);
    let audio_context = use_state(|| AudioContext::new().unwrap());
    
    let toggle_metronome = {
        let is_playing = is_playing.clone();
        let audio_context = audio_context.clone();
        let bpm = props.bpm;
        
        Callback::from(move |_| {
            if !*is_playing {
                let context = audio_context.clone();
                let interval = (60.0 / bpm as f32 * 1000.0) as i32;
                
                let _ = web_sys::window()
                    .unwrap()
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        &Closure::wrap(Box::new(move || {
                            let oscillator = OscillatorNode::new(&context).unwrap();
                            let gain = GainNode::new(&context).unwrap();
                            
                            oscillator.set_type(web_sys::OscillatorType::Sine);
                            oscillator.frequency().set_value(1000.0);
                            gain.gain().set_value(0.1);
                            
                            oscillator.connect_with_audio_node(&gain).unwrap();
                            gain.connect_with_audio_node(&context.destination()).unwrap();
                            
                            oscillator.start().unwrap();
                            oscillator.stop_with_when(context.current_time() + 0.1).unwrap();
                        }) as Box<dyn FnMut()>),
                        interval,
                    );
            }
            is_playing.set(!*is_playing);
        })
    };
    
    html! {
        <div class="metronome">
            <button
                class="metronome-button"
                onclick={toggle_metronome}
            >
                {if *is_playing { "Stop Metronome" } else { "Start Metronome" }}
            </button>
            <div class="bpm-display">
                {format!("{} BPM", props.bpm)}
            </div>
        </div>
    }
} 