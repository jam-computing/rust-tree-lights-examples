use std::env;

use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder};

const COUNT: i32 = 50;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("PLEASE SUPPYL MORE THAN ONE ARG");
    }

    let colour = &args[1];

    match colour.as_str() {
        "green" => green(),
        "red" => red(),
        "blue" => blue(),
        "alt" => alternating_colours(),
        _ => clear(),
    }
    green();
    red();
    blue();

    alternating_colours();
}

fn clear() {
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(COUNT)
                .strip_type(rs_ws281x::StripType::Ws2811Grb)
                .brightness(100)
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 0, 0];
    }
    controller.render().unwrap();
    println!("Rendered Clear");
}

// GRB
fn green() {
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(COUNT)
                .strip_type(rs_ws281x::StripType::Ws2811Grb)
                .brightness(100)
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [255, 0, 0, 0];
    }

    controller.render().unwrap();
    println!("Rendered Green");
}

fn red() {
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(COUNT)
                .strip_type(rs_ws281x::StripType::Ws2811Grb)
                .brightness(100)
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 255, 0, 0];
    }
    controller.render().unwrap();
    println!("Rendered Red");
}

fn blue() {
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(COUNT)
                .strip_type(rs_ws281x::StripType::Ws2811Grb)
                .brightness(100)
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);
    for led in leds {
        *led = [0, 0, 255, 0];
    }
    controller.render().unwrap();
    println!("Rendered Blue");
}

fn alternating_colours() {
    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0,
            ChannelBuilder::new()
                .pin(18)
                .count(COUNT)
                .strip_type(rs_ws281x::StripType::Ws2811Grb)
                .brightness(100)
                .build(),
        )
        .build()
        .unwrap();

    let mut i: isize = 0;
    loop {
        let leds = controller.leds_mut(0);
        if i > 2 {
            i = 0;
        }

        for led in &mut *leds {
            *led = [
                if i == 0 { 255 } else { 0 },
                if i == 1 { 255 } else { 0 },
                if i == 2 { 255 } else { 0 },
                0,
            ]
        }

        i += 1;

        controller.render().unwrap();
        println!("Rendered Alternating Colours");
    }
}
