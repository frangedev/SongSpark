use yew::prelude::*;
use web_sys::{FileReader, HtmlInputElement};
use wasm_bindgen::prelude::*;
use crate::audio::AudioEngine;

#[derive(Properties, PartialEq)]
pub struct SampleLibraryProps {
    #[prop_or_default]
    pub on_sample_added: Callback<String>,
}

#[function_component(SampleLibrary)]
pub fn sample_library(props: &SampleLibraryProps) -> Html {
    let audio_engine = use_state(|| AudioEngine::new().unwrap());
    let samples = use_state(Vec::new);
    
    let on_file_change = {
        let audio_engine = audio_engine.clone();
        let samples = samples.clone();
        let on_sample_added = props.on_sample_added.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file) = input.files().and_then(|files| files.get(0)) {
                let reader = FileReader::new().unwrap();
                let audio_engine = audio_engine.clone();
                let samples = samples.clone();
                let on_sample_added = on_sample_added.clone();
                
                let onload = Closure::wrap(Box::new(move |e: Event| {
                    let reader: FileReader = e.target_unchecked_into();
                    if let Ok(array_buffer) = reader.result() {
                        let array_buffer: js_sys::ArrayBuffer = array_buffer.dyn_into().unwrap();
                        let audio_buffer = audio_engine.context.decode_audio_data(&array_buffer).unwrap();
                        
                        let name = file.name();
                        audio_engine.load_sample(&name, audio_buffer);
                        
                        let mut current_samples = (*samples).clone();
                        current_samples.push(name.clone());
                        samples.set(current_samples);
                        
                        on_sample_added.emit(name);
                    }
                }) as Box<dyn FnMut(Event)>);
                
                reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();
                
                reader.read_as_array_buffer(&file).unwrap();
            }
        })
    };
    
    html! {
        <div class="sample-library">
            <h3>{"Sample Library"}</h3>
            <input
                type="file"
                accept="audio/*"
                onchange={on_file_change}
                class="file-input"
            />
            <div class="samples-list">
                {samples.iter().map(|sample| {
                    html! {
                        <div class="sample-item" key={sample.clone()}>
                            {sample}
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
} 