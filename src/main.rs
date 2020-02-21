use enigo::{Enigo, MouseButton, MouseControllable};
use rand::{thread_rng, Rng};
use std::thread;
use std::time::Duration;

fn main() {
    let mut rng = thread_rng();

    println!("Starting walk loop in 20s...");
    thread::sleep(Duration::from_secs(20));

    let mut enigo = Enigo::new();
    loop {
        // The sleep durations are meant to simulate realistic delays between mouse clicks

        enigo.mouse_down(MouseButton::Left);

        thread::sleep(Duration::from_millis(rng.gen_range(7, 1200)));
        enigo.mouse_down(MouseButton::Right);

        thread::sleep(Duration::from_millis(rng.gen_range(125, 1500)));
        enigo.mouse_up(MouseButton::Left);

        thread::sleep(Duration::from_millis(rng.gen_range(7, 250)));
        enigo.mouse_up(MouseButton::Right);

        let min_minutes_to_wait_ms = 5 * 60 * 1000;
        let max_minutes_to_wait_ms = 15 * 60 * 1000;
        thread::sleep(Duration::from_millis(
            rng.gen_range(min_minutes_to_wait_ms, max_minutes_to_wait_ms),
        ));
    }
}
