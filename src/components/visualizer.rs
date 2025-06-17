use yew::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::prelude::*;
use crate::audio::AudioEngine;

#[derive(Properties, PartialEq)]
pub struct VisualizerProps {
    #[prop_or_default]
    pub audio_engine: AudioEngine,
}

#[function_component(Visualizer)]
pub fn visualizer(props: &VisualizerProps) -> Html {
    let canvas_ref = use_node_ref();
    
    {
        let canvas_ref = canvas_ref.clone();
        let audio_engine = props.audio_engine.clone();
        
        use_effect_with_deps(
            move |_| {
                if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                    let context = canvas
                        .get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>()
                        .unwrap();
                    
                    let analyser = audio_engine.context.create_analyser().unwrap();
                    analyser.set_fft_size(2048);
                    
                    let data_array = js_sys::Uint8Array::new_with_length(analyser.frequency_bin_count() as u32);
                    
                    let animate = Closure::wrap(Box::new(move || {
                        analyser.get_byte_frequency_data(&data_array);
                        
                        let width = canvas.width() as f64;
                        let height = canvas.height() as f64;
                        let bar_width = width / data_array.length() as f64;
                        
                        context.clear_rect(0.0, 0.0, width, height);
                        context.set_fill_style(&"#3498db".into());
                        
                        for i in 0..data_array.length() {
                            let bar_height = (data_array.get_index(i) as f64 / 255.0) * height;
                            context.fill_rect(
                                i as f64 * bar_width,
                                height - bar_height,
                                bar_width - 1.0,
                                bar_height,
                            );
                        }
                        
                        request_animation_frame(animate.as_ref().unchecked_ref());
                    }) as Box<dyn FnMut()>);
                    
                    animate.forget();
                }
            },
            (),
        );
    }
    
    html! {
        <div class="visualizer">
            <canvas
                ref={canvas_ref}
                width="800"
                height="200"
                class="visualizer-canvas"
            />
        </div>
    }
} 