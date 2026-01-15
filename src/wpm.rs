use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

pub struct WpmTracker {
    keystroke_times: VecDeque<Instant>,
    window_secs: f32,
    keystroke_counter: Arc<AtomicU32>,
    last_synced_count: u32,
}

impl Default for WpmTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl WpmTracker {
    pub fn new() -> Self {
        let keystroke_counter = Arc::new(AtomicU32::new(0));
        let counter_clone = Arc::clone(&keystroke_counter);

        thread::spawn(move || {
            rdev::listen(move |event| {
                if let rdev::EventType::KeyPress(_) = event.event_type {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            })
            .expect("Failed to listen to keyboard events");
        });

        Self {
            keystroke_times: VecDeque::new(),
            window_secs: 5.0,
            keystroke_counter,
            last_synced_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.prune_old_keystrokes();

        let current_count = self.keystroke_counter.load(Ordering::Relaxed);
        let new_keystrokes = current_count - self.last_synced_count;
        let now = Instant::now();

        for _ in 0..new_keystrokes {
            self.keystroke_times.push_back(now);
        }
        self.last_synced_count = current_count;
    }

    fn prune_old_keystrokes(&mut self) {
        while let Some(front) = self.keystroke_times.front() {
            if front.elapsed().as_secs_f32() > self.window_secs {
                self.keystroke_times.pop_front();
            } else {
                break;
            }
        }
    }

    pub fn calculate_wpm(&self) -> f32 {
        let chars = self.keystroke_times.len() as f32;
        (chars / 5.0) * (60.0 / self.window_secs)
    }
}