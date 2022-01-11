mod colors;
mod tracer;

use tracer::*;

use nannou::prelude::*;
use nannou::ui::*;

use converter::args_parse::*;

use std::fs;
use std::io;
use std::io::prelude::*;

fn detect_image_in_folder(dir_path: &str) -> String {
    let mut image_path = String::new();
    let mut found_image = false;
    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let extension = path.extension();
            if let Some(e) = extension {
                let ext = e.to_str().unwrap(); 
                if ext == "png" || ext == "jpg" || ext == "jpeg" {
                    image_path = path.to_str().unwrap().to_string();
                    found_image = true;
                    break;
                }
            }
        }
    }
    if found_image {
        image_path
    }
    else {
        panic!("No image found in the folder.");
    }
}

fn hash_config(config: &Config) -> u64 {
    let order1 = config.export_options.point_precision.unwrap();
    let order2 = config.low_threshold * config.low_threshold;
    let order3 = config.high_threshold * config.high_threshold * config.high_threshold;
    let addition = order1 + order2 + order3;
    (addition * 1000000000.0) as u64
}

fn parse_commands_file(config: &Config) -> Vec<String> {
    let commands_path = format!("./assets/export/{}_command.txt" , config.input_name);
    let mut commands: Vec<String> = vec![];
    let f = fs::File::open(commands_path);
    if let Ok(file) = f {
        let reader = io::BufReader::new(file);
        for l in reader.lines() {
            if let Ok(line) = l {
                commands.push(line);
            }
        }
    }
    else {
        panic!("Could not open command file");
    }
    commands
}

fn main() {
    nannou::app(init)
        .update(update)
        .event(event)
        .view(view)
        .run();
}

struct Model {
    #[allow(dead_code)]
    window_id: window::Id,
    ui: Ui,
    ids: Ids,

    config_hash: u64,
    config: Config,

    commands: Vec<String>,

    tracer: Tracer,

    frame_count: u32,
    speed: u32,
}

widget_ids! {
    struct Ids {
        point_precision,
        low_threshold,
        high_threshold,
        reset_button,
        speed,
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

    let config = Config::new(
        detect_image_in_folder("./assets").as_str(),
        "./assets/export",
        50.0f32,
        60.0f32,
        false,
        0.9f32
    );

    let config_hash = hash_config(&config);
    converter::execute(&config);
    let commands = parse_commands_file(&config);

    Model {
        window_id: w_id,
        ui: ui,
        ids: ids,

        config_hash,
        config,

        commands: commands.clone(),

        tracer: Tracer::new(commands),

        frame_count: 0,
        speed: 30,
    }
}

/**
 * Timed updates
 */
#[allow(unused_variables)]
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

    for value in slider(model.config.export_options.point_precision.unwrap(), 0.0001, 1.0)
        .top_left_with_margin(20.0)
        .label("Precision")
        .set(model.ids.point_precision, ui)
    {
        model.config.export_options.point_precision = Some(value);
    }

    for value in slider(model.config.low_threshold, 0.0, 1140.39)
        .down(10.0)
        .label("Low Threshold")
        .set(model.ids.low_threshold, ui)
    {
        model.config.low_threshold = value;
    }

    for value in slider(model.config.high_threshold, 0.0, 1140.39)
        .down(10.0)
        .label("High Threshold")
        .set(model.ids.high_threshold, ui)
    {
        model.config.high_threshold = value;
    }

    for value in slider(model.speed as f32, 1.0, 60.0)
        .down(10.0)
        .label("Speed")
        .set(model.ids.speed, ui)
    {
        model.speed = value.round() as u32;
    }

    let new_hash_config = hash_config(&model.config);
    let reactive_red_color_value: f32 = if new_hash_config != model.config_hash {
        0.0
    }
    else {
        0.3
    };
    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 60.0)
        .label("Reset Config")
        .label_font_size(15)
        .rgb(reactive_red_color_value, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .set(model.ids.reset_button, ui)
    {
        if model.config_hash != new_hash_config {
            converter::execute(&model.config);
            let commands = parse_commands_file(&model.config);
            model.config_hash = new_hash_config;
            model.commands = commands;
        }
        model.tracer = Tracer::new(model.commands.clone());
    }

    // for (x, y) in widget::XYPad::new(
    //     model.position.x,
    //     -200.0,
    //     200.0,
    //     model.position.y,
    //     -200.0,
    //     200.0,
    // )
    // .down(10.0)
    // .w_h(200.0, 200.0)
    // .label("Position")
    // .label_font_size(15)
    // .rgb(0.3, 0.3, 0.3)
    // .label_rgb(1.0, 1.0, 1.0)
    // .border(0.0)
    // .set(model.ids.position, ui)
    // {
    //     model.position = Point2::new(x, y);
    // }

    model.frame_count += 1;
    if model.frame_count % model.speed == 0 {
        model.tracer.enable_execution();
    }
    model.tracer.execute_next();
}

/**
 * Window Events
 * https://github.com/nannou-org/nannou/blob/master/examples/nannou_basics/all_functions.rs
 */
#[allow(unused_variables)]
fn event(app: &App, model: &mut Model, event: Event) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // draw.background().rgb(0.02, 0.02, 0.02);

    model.tracer.draw_current(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}