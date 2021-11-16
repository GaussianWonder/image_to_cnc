// #![allow(dead_code)]
mod colors;
pub use colors::*;
use nannou::prelude::*;

fn main() {
    nannou::app(init)
        .update(update)
        .event(event)
        .view(view)
        .run();
}

struct Model {
    window_id: window::Id,
}

fn init(app: &App) -> Model {
    let wnd = app.new_window().view(view).build().unwrap();
    Model {
        window_id: wnd,
    }
}

/**
 * Timed updates
 */
fn update(app: &App, model: &mut Model, update: Update) {
}

/**
 * Window Events
 * https://github.com/nannou-org/nannou/blob/master/examples/nannou_basics/all_functions.rs
 */
fn event(app: &App, model: &mut Model, event: Event) {
}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(DARK_GRAY);
    let draw = app.draw();
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}