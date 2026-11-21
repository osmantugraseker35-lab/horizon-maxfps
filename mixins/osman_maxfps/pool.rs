use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

static CAP: AtomicUsize = AtomicUsize::new(0);
const POLL: Duration = Duration::from_millis(2);
const PREFIX: &str = "horizon-worker-";

thread_local! {
    static INDEX: Cell<Option<Option<usize>>> = const { Cell::new(None) };
}

pub fn set_cap(value: usize) {
    CAP.store(value, Ordering::Relaxed);
}

fn index() -> Option<usize> {
    INDEX.with(|cached| {
        if let Some(value) = cached.get() {
            return value;
        }
        let value = std::thread::current()
            .name()
            .and_then(|name| name.strip_prefix(PREFIX))
            .and_then(|number| number.parse().ok());
        cached.set(Some(value));
        value
    })
}

pub fn gate() {
    let Some(index) = index() else { return; };
    loop {
        let cap = CAP.load(Ordering::Relaxed);
        if cap == 0 || index < cap {
            return;
        }
        std::thread::sleep(POLL);
    }
}

#[cfg(windows)]
pub fn lower_current() {
    use windows_sys::Win32::System::Threading::{
        GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_BELOW_NORMAL,
    };
    unsafe {
        let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_BELOW_NORMAL);
    }
}

#[cfg(not(windows))]
pub fn lower_current() {}

