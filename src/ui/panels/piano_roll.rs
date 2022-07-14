use vizia::prelude::*;

use crate::ui::Panel;

pub fn piano_roll(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Panel::new(
            cx,
            |cx| {
                Label::new(cx, "PIANO ROLL").class("small");
            },
            |cx| {
                ScrollView::new(cx, 0f32, 0.5f32, true, true, |cx| {
                    HStack::new(cx, |cx| {
                        // TODO: Extract out scale
                        let scale = 1.0;
                        keys(cx, scale);
                        lanes(cx, scale);
                    });
                });
            },
        )
        .class("piano_roll");
    })
    .row_between(Pixels(1.0))
    .class("piano_roll");
}

fn index_to_key(i: i32) -> &'static str {
    match i % 12 {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        11 => "B",
        _ => "?",
    }
}

fn is_black(i: i32) -> bool {
    match index_to_key(i) {
        "C#" | "D#" | "F#" | "G#" | "A#" => true,
        _ => false,
    }
}

fn lanes(cx: &mut Context, scale: f32) {
    VStack::new(cx, |cx| {
        for i in (0..12 * 12).rev() {
            HStack::new(cx, |cx| {})
                .class("lane")
                .class(if is_black(i) { "black_lane" } else { "white_lane" })
                .height(Pixels(10.14 * scale));
        }
        HStack::new(cx, |_| {}).class("piano_roll_bottom_padding");
    })
    .class("lanes");
}

fn keys(cx: &mut Context, scale: f32) {
    VStack::new(cx, |cx| {
        for i in (0..12 * 12).rev() {
            let octave = i / 12;
            let key = index_to_key(i);
            let is_black = is_black(i);
            let height = if is_black { 10.5 } else { 17.14 } * scale;

            let e = HStack::new(cx, |cx| {
                if key == "C" {
                    Label::new(cx, key).class("small");
                    Label::new(cx, octave).class("small");
                }
            })
            .height(Pixels(height))
            .class(if is_black { "black_piano_key" } else { "white_piano_key" });

            if is_black {
                e.top(Pixels(-height / 2.0)).bottom(Pixels(-height / 2.0));
            }
        }

        HStack::new(cx, |_| {}).class("piano_roll_bottom_padding");
    });
}
