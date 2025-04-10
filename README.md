# Macroquad-Timer
Macroquad Timer Utility

A simple and elegant timer utility for Macroquad game development in Rust.
Features

    Easy-to-use timer implementation

    Precise time tracking

    Multiple utility methods:

        Check if timer is finished

        Get elapsed time

        Get remaining time

        Reset functionality

**Installation**

Add this to your Cargo.toml:

```
[dependencies]
macroquad = "0.4"
macroquad-timer = { git = "https://github.com/neomin-2007/Macroquad-Timer" }
```

**Usage Example**

```
use macroquad::prelude::*;
use macroquad_timer::Timer;

#[macroquad::main("Timer Example")]
async fn main() {
    let mut timer = Timer::new(3.0, false);
    
    loop {
        clear_background(BLACK);
        
        if timer.is_finished() {
            draw_text("Timer finished!", 20.0, 20.0, 30.0, WHITE);
        } else {
            draw_text(
                &format!("Time remaining: {:.1}", timer.remaining()),
                20.0,
                20.0,
                30.0,
                WHITE,
            );
        }
        
        if is_key_pressed(KeyCode::Space) {
            timer.reset();
        }
        
        next_frame().await;
    }
}
```
