use chips::cpu::tms0800;
use chips::shifter;

pub(super) struct Display {
  display: web_sys::Element,
  current_str: String,
}

impl Display {
  pub(super) fn new() -> Self {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let display = document.get_element_by_id("display").unwrap();
    
    Self {
      display,
      current_str: String::from("         "),
    }
  }

  pub fn run_refresh_cycle(&mut self, chip: &tms0800::TMS0800) {
    let mut buffer = Vec::with_capacity(15);
    let mut a = chip.alu.a.clone();
    let mut b = chip.alu.b.clone();
    let direction = shifter::Direction::Left;
    for location in 0..9 {
      let mask = b.read_nibble(direction);
      let digit = a.read_nibble(direction);
      buffer.push(if digit.value() == 14 {
        '-'
      } else {
        if location == 0 {  //Last digit shouldn't have a number. It's either '-' or ' '.
          ' '
        } else {
          (digit.value() + 48) as char
        }
      });
      if mask.value() == 2 {
        buffer.push('.');
      }
      a.shift_with_nibble(direction, digit);
      b.shift_with_nibble(direction, mask);
    }
    
    let new_str = buffer.iter().collect::<String>();
    if self.current_str != new_str {
      self.display.set_text_content(Some(&new_str));
      self.current_str = new_str.clone();
    }
  }
}