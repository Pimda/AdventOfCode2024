use colored::*;
use std::time::Instant;

pub(crate) fn start_timer() -> Instant {
    Instant::now()
}

pub(crate) fn stop_timer_and_write(now: Instant, string: &str) {
    let elapsed = now.elapsed();

    let formatted_elapsed: ColoredString;

    if elapsed.as_millis() == 0 {
        formatted_elapsed = format!("{elapsed:#?}").green().bold();
    } else if elapsed.as_secs() == 0 {
        formatted_elapsed = format!("{elapsed:#?}").yellow().bold();
    } else {
        formatted_elapsed = format!("{elapsed:#?}").red().bold();
    }

    println!("{string}");
    println!("Duration: {formatted_elapsed}");
}
