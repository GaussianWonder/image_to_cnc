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
        }
      }
      self.enable = false;
    }
  }

  pub fn draw_current(&self, draw: &nannou::Draw) {
    if self.current.pen == PenDirection::DOWN && self.previous.pen == PenDirection::DOWN {
      draw.line()
        .start(pt2(self.previous.x, self.previous.y))
        .end(pt2(self.current.x, self.current.y))
        .color(YELLOW);

      draw.ellipse()
        .xy(pt2(self.current.x, self.current.y))
        .radius(10.0)
        .resolution(30.0)
        .color(WHITE);

      draw.ellipse()
        .xy(pt2(self.current.x, self.current.y))
        .radius(5.0)
        .resolution(30.0)
        .color(RED);
    }
    else {
      draw.ellipse()
        .xy(pt2(self.current.x, self.current.y))
        .radius(10.0)
        .resolution(30.0)
        .color(WHITE);
    }
  }
}
