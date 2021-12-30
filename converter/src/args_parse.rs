use clap::{Arg, App, ArgMatches};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
  // input / output paths
  pub input_file: PathBuf,
  pub export_path: PathBuf,

  // input file relevant details
  pub input_name: String,
  pub input_extension: String,

  // canny edge detection settings
  pub low_threshold: f32,
  pub high_threshold: f32,

  // export configuration
  pub export_options: ExportOptions,
}

#[derive(Debug)]
pub struct ExportOptions {
  // point resolution for JSON export
  pub point_precision: Option<f32>,
  // export edge detected image
  pub image: bool,
  // export edge detected image with drawn points
  pub debug_preview: Option<f32>,
}

pub fn get_raw() -> ArgMatches {
  App::new("converter")
    .version("0.1.0")
    .author("Virghileanu Teodor <@GaussianWonder>")
    .about("CNC Converter")
    .arg(Arg::new("INPUT")
      .help("Sets the input image to use")
      .required(true)
      .index(1))
    .arg(Arg::new("output")
      .short('o')
      .long("output")
      .value_name("DIRECTORY PATH")
      .help("Sets a custom export path")
      .takes_value(true)
      .required(false))
    .arg(Arg::new("low_threshold")
      .short('l')
      .long("low-threshold")
      .value_name("FLOAT32")
      .help("Sets the low threshold for the Canny edge detector (>=0)")
      .takes_value(true)
      .default_value("50.0"))
    .arg(Arg::new("high_threshold")
      .short('h')
      .long("high_threshold")
      .value_name("FLOAT32")
      .help("Sets the high threshold for the Canny edge detector (<=1140.39)")
      .takes_value(true)
      .default_value("60.0"))
    .subcommand(App::new("export")
      .about("controls export features")
      .version("0.1.0")
      .author("Virghileanu Teodor <@GaussianWonder>")
      .arg(Arg::new("point_precision")
        .short('p')
        .long("p_precision")
        .value_name("FLOAT32")
        .help("Exports edge points with a given precision. This is a scale factor for the initial image resolution")
        .takes_value(true))
      .arg(Arg::new("image")
        .short('i')
        .long("image")
        .help("Export image to the given export path"))
      .arg(Arg::new("debug_preview")
        .short('d')
        .long("debug_preview")
        .value_name("FLOAT32")
        .help("Exports the image with points traced on it. This comes with its own scale value for point precision. See point_precision for details")
        .takes_value(true))
    ).get_matches()
}

fn check_input_extension(input_file: &PathBuf) -> bool {
  let accepted_extensions = vec!["jpg", "jpeg", "png", "gif", "ico", "pnm", "farbfeld"];
  if let Some(extension) = input_file.extension() {
    let lower = extension.to_ascii_lowercase();
    let ext = lower.to_str().unwrap();
    accepted_extensions.contains(&ext)
  }
  else {
    false
  }
}

fn get_export_options(args: &ArgMatches) -> ExportOptions {
  if let Some(export) = args.subcommand_matches("export") {
    let point_precision = if let Some(point_precision) = export.value_of("point_precision") {
      match point_precision.parse::<f32>() {
        Ok(p) => Some(p),
        Err(_) => {
          panic!("The point precision provided is not a valid float32.");
        }
      }
    }
    else {
      None
    };

    let image = export.is_present("image");

    let debug_preview = if let Some(debug_preview) = export.value_of("debug_preview") {
      match debug_preview.parse::<f32>() {
        Ok(p) => Some(p),
        Err(_) => {
          panic!("The point precision provided is not a valid float32.");
        }
      }
    }
    else {
      None
    };

    ExportOptions {
      point_precision,
      image,
      debug_preview,
    }
  }
  else {
    ExportOptions {
      point_precision: None,
      image: true, // by default export just the image
      debug_preview: None,
    }
  }
}

pub fn get() -> Config {
  let args = get_raw();

  let input_file = PathBuf::from(args.value_of("INPUT").unwrap());
  if !input_file.is_file() {
    panic!("The input provided does not point to a file or does not exist.");
  }
  
  if check_input_extension(&input_file) == false {
    panic!("The input file does not have a valid extension.");
  }

  let export_path = if let Some(export_path) = args.value_of("output") {
    PathBuf::from(export_path)
  }
  else {
    input_file.parent().unwrap().to_path_buf()
  };

  let file_name = input_file.with_extension("").file_name().unwrap().to_ascii_lowercase().to_str().unwrap().to_string();
  let file_extension = input_file.extension().unwrap().to_ascii_lowercase().to_str().unwrap().to_string();

  let export = get_export_options(&args);

  let low_threshold = match args.value_of("low_threshold") {
    Some(low_threshold) => {
      match low_threshold.parse::<f32>() {
        Ok(l) => l,
        Err(_) => {
          panic!("The low threshold provided is not a valid float32.");
        }
      }
    },
    None => 50.0,
  };

  let high_threshold = match args.value_of("high_threshold") {
    Some(high_threshold) => {
      match high_threshold.parse::<f32>() {
        Ok(h) => h,
        Err(_) => {
          panic!("The high threshold provided is not a valid float32.");
        }
      }
    },
    None => 60.0,
  };

  Config {
    input_file,
    export_path,
    input_name: file_name,
    input_extension: file_extension,
    low_threshold,
    high_threshold,
    export_options: export,
  }
}
