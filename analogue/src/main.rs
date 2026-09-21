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
use chrono_tz::Tz;
use clap::Parser;
use raylib::prelude::*;

use std::{str::FromStr, sync::{
        Arc, atomic::{AtomicBool, Ordering},
    }};

const APP_NAME: &str = "Anytime ✕ Analogue";

const DEFAULT_WINDOW_WIDTH: i32 = 460;
const DEFAULT_WINDOW_HEIGHT: i32 = 460;

#[derive(Parser)]
#[command(version, about)]
struct Args {

    /// IANA time zone name to display (e.g., Pacific/Honolulu).
    #[arg(short, long)]
    time_zone: String,

    /// Clock name.
    #[arg(short, long)]
    name: String,

}

fn main() {
    let args = Args::parse();

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

    let roboto_font = rl.load_font(&thread, "resources/roboto/Roboto-Regular.ttf")
        .expect("failed to load font");

    let time_zone = Tz::from_str(&args.time_zone).expect("Unable to parse time zone");

    while !rl.window_should_close() && running.load(Ordering::SeqCst) {

        const NAME_SIZE: f32 = 24.0;
        const FONT_SPACING: f32 = 0.0;

        let name = time_zone.to_string();

        let window_width = rl.get_screen_width();
        let window_height = rl.get_screen_height();

        let center = Vector2::new(
            window_width as f32 / 2.0,
            window_height as f32 / 2.0
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(0, 42, 66, 255));

        let now = Utc::now().with_timezone(&time_zone);

        let pi_2 = 2 as f32 * PI as f32;
        let rotation = PI as f32 / 2.0;

        const BEZEL_WIDTH: f32 = 10.0;
        const BEZEL_RADIUS: f32 = 200.0;

        const HOUR_HAND_LENGTH: f32 = 80.0;
        const HOUR_HAND_WIDTH: f32 = 12.0;
        const HOUR_HAND_COLOR: Color = Color::WHITE;

        const MINUTE_HAND_LENGTH: f32 = 140.0;
        const MINUTE_HAND_WIDTH: f32 = 6.0;
        const MINUTE_HAND_COLOR: Color = Color::WHITE;

        const SECOND_HAND_LENGTH: f32 = 160.0;
        const SECOND_HAND_WIDTH: f32 = 2.0;
        const SECOND_HAND_COLOR: Color = Color::new(170, 170, 170, 255);

        d.draw_ring(
            center,
            BEZEL_RADIUS - (BEZEL_WIDTH / 2.0),
            BEZEL_RADIUS + (BEZEL_WIDTH / 2.0),
            0.0,
            360.0,
            100,
            Color::WHITE);

        let mut draw_text = |text: &str, offset: f32, color: Color| {
            let text_width = roboto_font.measure_text(&text, NAME_SIZE, FONT_SPACING);
            let text_position = Vector2::new(center.y - (text_width.x / 2.0), offset);
            d.draw_text_codepoints(&roboto_font, &text, text_position, NAME_SIZE, FONT_SPACING, color);
        };

        let text_offset: f32 = 312.0;
        draw_text(&args.name, text_offset, SECOND_HAND_COLOR);
        let day = String::new() + &now.weekday().to_string() + " " + &now.day().to_string();
        draw_text(&day, text_offset + 30.0, Color::WHITE);

        let second = now.second() as f32 + (now.nanosecond() as f32 / 1000000000.0);
        let minute = now.minute() as f32 + (second / 60.0);
        let hour = (now.hour() % 12) as f32 + (minute / 60.0);

        let hour_angle = (pi_2 / 12.0) * hour - rotation;
        let hour_end = center + Vector2::new(hour_angle.cos(), hour_angle.sin()) * HOUR_HAND_LENGTH;
        d.draw_line_ex(center, hour_end, HOUR_HAND_WIDTH, HOUR_HAND_COLOR);
        d.draw_circle_v(hour_end, HOUR_HAND_WIDTH / 2.0, HOUR_HAND_COLOR);
        d.draw_circle_v(center, 10.0, HOUR_HAND_COLOR);

        let minute_angle = (pi_2 / 60.0) * minute - rotation;
        let minute_end = center + Vector2::new(minute_angle.cos(), minute_angle.sin()) * MINUTE_HAND_LENGTH;
        d.draw_line_ex(center, minute_end, MINUTE_HAND_WIDTH, MINUTE_HAND_COLOR);
        d.draw_circle_v(minute_end, MINUTE_HAND_WIDTH / 2.0, MINUTE_HAND_COLOR);
        d.draw_circle_v(center, 7.0, MINUTE_HAND_COLOR);

        let seconds_angle = (pi_2 / 60.0) * second - rotation;
        let second_end = center + Vector2::new(seconds_angle.cos(), seconds_angle.sin()) * SECOND_HAND_LENGTH;
        d.draw_line_ex(center, second_end, SECOND_HAND_WIDTH, SECOND_HAND_COLOR);
        d.draw_circle_v(second_end, SECOND_HAND_WIDTH / 2.0, SECOND_HAND_COLOR);
        let second_start = center - Vector2::new(seconds_angle.cos(), seconds_angle.sin()) * 30.0;
        d.draw_line_ex(center, second_start, SECOND_HAND_WIDTH, SECOND_HAND_COLOR);
        d.draw_circle_v(second_start, SECOND_HAND_WIDTH / 2.0, SECOND_HAND_COLOR);
        d.draw_circle_v(center, 4.0, SECOND_HAND_COLOR);

    }

}
