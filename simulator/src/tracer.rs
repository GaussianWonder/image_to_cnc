// Drawing tip of the cnc machine
use converter::canny::PenDirection;
use nannou::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawingTip {
  x: f32,
  y: f32,
  pen: PenDirection,
}

pub struct Tracer {
  enable: bool,
  finished: bool,
  commands: Vec<String>,

  current: DrawingTip,
  previous: DrawingTip,

  lines: Vec<(Point2, Point2)>,
}

impl Tracer {
  pub fn new(commands: Vec<String>) -> Tracer {
    Tracer {
      enable: true,
      finished: false,
      commands: commands,

      current: DrawingTip {
        x: 0.0,
        y: 0.0,
        pen: PenDirection::UP,
      },
      previous: DrawingTip {
        x: 0.0,
        y: 0.0,
        pen: PenDirection::UP,
      },

      lines: vec![],
    }
  }

  pub fn enable_execution(&mut self) {
    self.enable = true;
  }

  pub fn is_finished(&self) -> bool {
    self.finished
  }

  pub fn next_command(&mut self) -> Option<String> {
    if self.finished || !self.enable {
      return None;
    }

    let command = self.commands.remove(0);
    if self.commands.len() == 0 {
      self.finished = true;
    }

    Some(command)
  }

  pub fn get_next_command(&self) -> Option<String> {
    if self.finished || !self.enable {
      return None;
    }

    let command = self.commands.get(0);
    match command {
      Some(command) => Some(command.to_string()),
      None => None,
    }
  }

  pub fn execute_next(&mut self) {
    if self.enable {
      let command = self.next_command();
      let mut mutated = true;
      if let Some(command) = command {
        let copy_current = self.current.clone();
        match command.as_str() {
          "PEN UP" => self.current.pen = PenDirection::UP,
          "PEN DOWN" => self.current.pen = PenDirection::DOWN,
          "RESET" => {
            self.current.pen = PenDirection::UP; // safeguard
            self.current.x = 0.0;
            self.current.y = 0.0;
          }
          "END" => self.finished = true,
          _ => {
            // parse "MOVE {X} {Y}" command
            let mut iter = command.split_whitespace();
            let command = iter.next().unwrap();

            match command {
              "MOVE" => {
                let x = iter.next().unwrap().parse::<f32>().unwrap();
                let y = iter.next().unwrap().parse::<f32>().unwrap();
                self.current.x = x;
                self.current.y = y;
              }
              _ => {
                // panic!("Unknown command: {}", command);
                // nothing to do, skip
                println!("Unknown command: {}", command);
                mutated = false;
              }
            }
          }
        };

        if mutated {
          self.previous = copy_current;

          if self.current.pen == PenDirection::DOWN && self.previous.pen == PenDirection::DOWN {
            self.lines.push((
              Point2::new(self.previous.x, self.previous.y),
              Point2::new(self.current.x, self.current.y)
            ));
          }
        }
      }
      self.enable = false;
    }
  }

  pub fn draw_current(&self, draw: &nannou::Draw, scale: f32, offset: Point2) {
    for ln in &self.lines {
      let pt1 = Point2::new(ln.0.x * scale + offset.x, ln.0.y * scale + offset.y);
      let pt2 = Point2::new(ln.1.x * scale + offset.x, ln.1.y * scale + offset.y);

      draw.line()
        .start(pt1)
        .end(pt2)
        .color(YELLOW);
    }

    let pt = Point2::new(self.current.x * scale + offset.x, self.current.y * scale + offset.y);
    draw.ellipse()
      .xy(pt)
      .radius(10.0)
      .resolution(30.0)
      .color(WHITE);

    if self.current.pen == PenDirection::DOWN && self.previous.pen == PenDirection::DOWN {
      draw.ellipse()
        .xy(pt)
        .radius(5.0)
        .resolution(30.0)
        .color(RED);
    }
  }
}
