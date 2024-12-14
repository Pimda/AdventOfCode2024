use std::time::Instant;

pub(crate) fn start_timer() -> Instant {
    Instant::now()
}

pub(crate) fn stop_timer_and_write(now: Instant, string: &str) {
    let elapsed = now.elapsed();
    println!("{string}");
    println!("Duration: {elapsed:#?}");
}
