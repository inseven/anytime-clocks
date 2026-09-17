// Copyright (c) 2025-2026 Jason Morley
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use chrono::prelude::*;
use clap::Parser;
use raylib::prelude::*;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

const APP_NAME: &str = "Anytime ✕ Analogue";

const DEFAULT_WINDOW_WIDTH: i32 = 460;
const DEFAULT_WINDOW_HEIGHT: i32 = 460;

#[derive(Parser)]
#[command(version, about)]
struct Args {

    /// Perform frame and state validation and exit early.
    #[arg(short, long)]
    validate_only: bool,
}

fn main() {
    let _ = Args::parse();

    // Set up an atomic boolean to respond to Ctrl + C signals.
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Failed to set cancellation handler.");

    // Initialize raylib.
    let (mut rl, thread) = raylib::init()
        .size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
        .msaa_4x()
        .title(&APP_NAME)
        .resizable()
        .build();
    rl.set_target_fps(60);

    while !rl.window_should_close() && running.load(Ordering::SeqCst) {

        // Get the screen / window size.
        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();

        let center = Vector2::new(
            window_width as f32 / 2.0,
            window_height as f32 / 2.0
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(0, 42, 66, 255));

        let local: DateTime<Local> = Local::now();

        let pi_2 = 2 as f32 * PI as f32;
        let rotation = PI as f32 / 2.0;

        const BEZEL_WIDTH: f32 = 10.0;
        const BEZEL_RADIUS: f32 = 200.0;

        const HOUR_HAND_LENGTH: f32 = 80.0;
        const HOUR_HAND_WIDTH: f32 = 12.0;

        const MINUTE_HAND_LENGTH: f32 = 140.0;
        const MINUTE_HAND_WIDTH: f32 = 6.0;

        const SECOND_HAND_LENGTH: f32 = 160.0;
        const SECOND_HAND_WIDTH: f32 = 2.0;

        d.draw_ring(
            center,
            BEZEL_RADIUS - (BEZEL_WIDTH / 2.0),
            BEZEL_RADIUS + (BEZEL_WIDTH / 2.0),
            0.0,
            360.0,
            100,
            Color::WHITE);

        let second = local.second() as f32 + (local.nanosecond() as f32 / 1000000000.0);
        let minute = local.minute() as f32 + (second / 60.0);
        let hour = (local.hour() % 12) as f32 + (minute / 60.0);

        let hour_angle = (pi_2 / 12.0) * hour - rotation;
        let hour_end = center + Vector2::new(hour_angle.cos(), hour_angle.sin()) * HOUR_HAND_LENGTH;
        d.draw_line_ex(center, hour_end, HOUR_HAND_WIDTH, Color::WHITE);
        d.draw_circle_v(hour_end, HOUR_HAND_WIDTH / 2.0, Color::WHITE);
        d.draw_circle_v(center, 10.0, Color::WHITE);

        let minute_angle = (pi_2 / 60.0) * minute - rotation;
        let minute_end = center + Vector2::new(minute_angle.cos(), minute_angle.sin()) * MINUTE_HAND_LENGTH;
        d.draw_line_ex(center, minute_end, MINUTE_HAND_WIDTH, Color::WHITE);
        d.draw_circle_v(minute_end, MINUTE_HAND_WIDTH / 2.0, Color::WHITE);
        d.draw_circle_v(center, 7.0, Color::WHITE);

        let second_hand_color = Color::new(170, 170, 170, 255);
        let seconds_angle = (pi_2 / 60.0) * second - rotation;
        let second_end = center + Vector2::new(seconds_angle.cos(), seconds_angle.sin()) * SECOND_HAND_LENGTH;
        d.draw_line_ex(center, second_end, SECOND_HAND_WIDTH, second_hand_color);
        d.draw_circle_v(second_end, SECOND_HAND_WIDTH / 2.0, second_hand_color);
        let second_start = center - Vector2::new(seconds_angle.cos(), seconds_angle.sin()) * 30.0;
        d.draw_line_ex(center, second_start, SECOND_HAND_WIDTH, second_hand_color);
        d.draw_circle_v(second_start, SECOND_HAND_WIDTH / 2.0, second_hand_color);
        d.draw_circle_v(center, 4.0, second_hand_color);

    }

}
