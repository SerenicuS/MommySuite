use std::{io, thread};
use std::io::Write;
use std::time::{Duration};
use crate::loader_animations::{get_jitter, AnimationType};
use crate::suite_constants;

pub struct MommyBootloader {
    message: String,
    base_delay_ms: u64,
}


impl MommyBootloader {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            base_delay_ms: suite_constants::BASE_DELAY_MS,
        } 
    }
    
    pub fn double_new (message: &str, message2: &str) -> Self {
        Self {
            message: format!("{} {}", message, message2),
            base_delay_ms: suite_constants::BASE_DELAY_MS,
        }
    }

    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.base_delay_ms = delay_ms;
        self
    }
    
    pub fn start(&self, animation_type: AnimationType) {
        match animation_type {
            AnimationType::Typewriter => self.typewriter_animation(),
            AnimationType::Heartbeat => self.heartbeat_animation(),
            AnimationType::Glitch(end_tag) => self.glitch_animation(end_tag),
        }
    }
    pub fn typewriter_animation(&self) {
        for c in self.message.chars() {
            print!("{}", c);
            io::stdout().flush().unwrap();

            // Add random jitter so the typing feels human/broken
            let jitter = get_jitter(suite_constants::JITTER_100);
            thread::sleep(Duration::from_millis(self.base_delay_ms + jitter));
        }
        println!();
    }

    // A flatline that occasionally spikes
    pub fn heartbeat_animation(&self) {
        print!("{} ", self.message);
        let frames = vec!["_ _ _ _", "_ _ ^ _", "_ / \\ _", "_ _ _ _", "_ _ _ _", "_ _ _ _"];

        for i in 0..15 {
            print!("\r{} {}", self.message, frames[i % frames.len()]);
            io::stdout().flush().unwrap();

            // Heartbeats aren't perfectly steady when you're panicked
            let jitter = get_jitter(suite_constants::JITTER_150);
            thread::sleep(Duration::from_millis(self.base_delay_ms + jitter));
        }
        print!("\r\x1B[2K{} [CRITICAL]\n", self.message);
    }

    // Rapidly flashes garbage characters before stabilizing
    pub fn glitch_animation(&self, end_tag: &str) {
        let garbage = vec!["0xFA", "NULL", "ERR", "####", "0x00"];
        for i in 0..20 {
            print!("\r{} {}", self.message, garbage[i % garbage.len()]);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(30));
        }
        // Prints whatever tag we pass in!
        print!("\r\x1B[2K{} {}\n", self.message, end_tag);
    }


}