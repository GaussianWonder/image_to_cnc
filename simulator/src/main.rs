// #![allow(dead_code)]
mod colors;

pub use colors::*;
use nannou::prelude::*;
use nannou::ui::*;
use nannou::ui::prelude::*;

fn main() {
    nannou::app(init)
        .update(update)
        .event(event)
        .view(view)
        .run();
}

struct Model {
    window_id: window::Id,
    ui: Ui,
    ids: Ids,

    resolution: f32,
    scale: f32,
    rotation: f32,
    color: Rgb,
    position: Point2,
}

widget_ids! {
    struct Ids {
        resolution,
        scale,
        rotation,
        random_color,
        position,
    }
}

fn init(app: &App) -> Model {
    // Create a window.
    let w_id = app
        .new_window()
        .view(view)
        .build()
        .unwrap();

    // Create the UI for our window.
    let mut ui = app.new_ui().window(w_id).build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    Model {
        window_id: w_id,
        ui: ui,
        ids: ids,

        resolution: 3.0,
        scale: 200.0,
        rotation: 0.0,
        color: rgb(1.0, 0.0, 1.0),
        position: pt2(0.0, 0.0),
    }
}

/**
 * Timed updates
 */
fn update(app: &App, model: &mut Model, update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.resolution, 3.0, 15.0)
        .top_left_with_margin(20.0)
        .label("Resolution")
        .set(model.ids.resolution, ui)
    {
        model.resolution = value.round();
    }

    for value in slider(model.scale, 10.0, 500.0)
        .down(10.0)
        .label("Scale")
        .set(model.ids.scale, ui)
    {
        model.scale = value;
    }

    for value in slider(model.rotation, -PI, PI)
        .down(10.0)
        .label("Rotation")
        .set(model.ids.rotation, ui)
    {
        model.rotation = value;
    }

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Random Color")
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.random_color, ui)
    {
        model.color = rgb(random(), random(), random());
    }

    for (x, y) in widget::XYPad::new(
        model.position.x,
        -200.0,
        200.0,
        model.position.y,
        -200.0,
        200.0,
    )
    .down(10.0)
    .w_h(200.0, 200.0)
    .label("Position")
    .label_font_size(15)
    .rgb(0.3, 0.3, 0.3)
    .label_rgb(1.0, 1.0, 1.0)
    .border(0.0)
    .set(model.ids.position, ui)
    {
        model.position = Point2::new(x, y);
    }
}

/**
 * Window Events
 * https://github.com/nannou-org/nannou/blob/master/examples/nannou_basics/all_functions.rs
 */
fn event(app: &App, model: &mut Model, event: Event) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().rgb(0.02, 0.02, 0.02);

    draw.ellipse()
        .xy(model.position)
        .radius(model.scale)
        .resolution(model.resolution)
        .rotate(model.rotation)
        .color(model.color);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}