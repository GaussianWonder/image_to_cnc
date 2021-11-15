use nannou::prelude::*;

fn main() {
    nannou::app(init)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn init(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(WHITE);
}