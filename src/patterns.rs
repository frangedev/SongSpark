use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub events: Vec<Event>,
    pub gain: f32,
    pub pan: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub sample: String,
    pub time: f32,
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            gain: 1.0,
            pan: 0.0,
        }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        let mut pattern = Pattern::new();
        let mut time = 0.0;
        
        for token in input.split_whitespace() {
            if let Some((sample, repeat)) = token.split_once('*') {
                let repeat = repeat.parse::<usize>().map_err(|_| "Invalid repeat count")?;
                for _ in 0..repeat {
                    pattern.events.push(Event {
                        sample: sample.to_string(),
                        time,
                    });
                    time += 0.25; // Quarter note duration
                }
            } else {
                pattern.events.push(Event {
                    sample: token.to_string(),
                    time,
                });
                time += 0.25;
            }
        }
        
        Ok(pattern)
    }

    pub fn gain(mut self, gain: f32) -> Self {
        self.gain = gain;
        self
    }

    pub fn pan(mut self, pan: f32) -> Self {
        self.pan = pan;
        self
    }
} 