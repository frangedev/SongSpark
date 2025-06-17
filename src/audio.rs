use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, AudioBuffer, AudioBufferSourceNode, GainNode, StereoPannerNode};
use std::collections::HashMap;

#[derive(Clone)]
pub struct AudioEngine {
    context: AudioContext,
    samples: HashMap<String, AudioBuffer>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, JsValue> {
        let context = AudioContext::new()?;
        let samples = HashMap::new();
        
        Ok(Self { context, samples })
    }

    pub fn load_sample(&mut self, name: &str, buffer: AudioBuffer) {
        self.samples.insert(name.to_string(), buffer);
    }

    pub fn play_sample(&self, name: &str, gain: f32, pan: f32) -> Result<(), JsValue> {
        if let Some(buffer) = self.samples.get(name) {
            let source = AudioBufferSourceNode::new(&self.context)?;
            source.set_buffer(Some(buffer));
            
            let gain_node = GainNode::new(&self.context)?;
            gain_node.gain().set_value(gain);
            
            let panner = StereoPannerNode::new(&self.context)?;
            panner.pan().set_value(pan);
            
            source.connect_with_audio_node(&gain_node)?;
            gain_node.connect_with_audio_node(&panner)?;
            panner.connect_with_audio_node(&self.context.destination())?;
            
            source.start()?;
        }
        Ok(())
    }
} 