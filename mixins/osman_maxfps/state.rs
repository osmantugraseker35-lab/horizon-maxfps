use std::cell::RefCell;

const MIN_WORKERS: usize = 2;
const STUTTER_MS: f32 = 24.0;
const RECOVER_FRAMES: u32 = 240;

#[derive(Default)]
pub struct Governor {
    cap: usize,
    smooth: u32,
    average_ms: f32,
}

impl Governor {
    pub fn update(&mut self, workers: usize, playing: bool, frame_ms: f32) -> usize {
        if !playing || workers <= MIN_WORKERS {
            self.cap = workers;
            self.smooth = 0;
            return 0;
        }
        if self.cap == 0 {
            self.cap = (workers / 2).max(MIN_WORKERS);
        }
        if frame_ms.is_finite() && frame_ms > 0.0 {
            if self.average_ms == 0.0 {
                self.average_ms = frame_ms;
            }
            let stutter = frame_ms >= STUTTER_MS
                && frame_ms > self.average_ms * 1.8
                && frame_ms > self.average_ms + 7.0;
            if stutter {
                self.cap = self.cap.saturating_sub(1).max(MIN_WORKERS);
                self.smooth = 0;
            } else {
                self.average_ms += (frame_ms - self.average_ms) * 0.04;
                self.smooth += 1;
                if self.smooth >= RECOVER_FRAMES {
                    self.cap = (self.cap + 1).min(workers);
                    self.smooth = 0;
                }
            }
        }
        if self.cap >= workers { 0 } else { self.cap }
    }
}

thread_local! {
    static GOVERNOR: RefCell<Governor> = RefCell::new(Governor::default());
}

pub fn update(workers: usize, playing: bool, frame_ms: f32) -> usize {
    GOVERNOR.with(|governor| governor.borrow_mut().update(workers, playing, frame_ms))
}

