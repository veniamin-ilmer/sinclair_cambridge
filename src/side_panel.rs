use chips::tms0800;
use chips::shifter;


pub struct SidePanel {
  address: Option<web_sys::HtmlCollection>,
  registers: Option<web_sys::HtmlCollection>,
  flags: Option<web_sys::HtmlCollection>,
  current_registers: [tms0800::alu::Register; 3],
  current_flags: [tms0800::control::Flag; 2],
}

impl SidePanel {
  
  pub fn new() -> Self {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    Self {
      address: get_tr_list(&document, "address"),
      registers: get_tr_list(&document, "registers"),
      flags: get_tr_list(&document, "flags"),
      current_registers: Default::default(),
      current_flags: Default::default(),
    }
  }
  
  pub fn run_refresh_cycle(&mut self, chip: &tms0800::TMS0800) {
    self.print_address(&chip.control);
    self.print_flags(&chip.control);
    self.print_registers(&chip.alu);
  }

  fn print_address(&self, control: &tms0800::control::ControlUnit) {
    if let Some(tr_list) = &self.address {
      let td_list = tr_list.item(0).expect("can't get tr").children();
      if let Some(td) = td_list.item(1) {
        td.set_text_content(Some(&format!("{:03X}", control.pc.value())));
      }
    }
  }

  fn print_flags(&mut self, control: &tms0800::control::ControlUnit) {
    if let Some(tr_list) = &self.flags {
      print_flag(&mut self.current_flags[0], tr_list, control.fa, 1);
      print_flag(&mut self.current_flags[1], tr_list, control.fb, 2);
    }
  }
  
  fn print_registers(&mut self, alu: &tms0800::alu::ALU) {
    if let Some(tr_list) = &self.registers {
      print_reg(&mut self.current_registers[0], tr_list, alu.a, 1);
      print_reg(&mut self.current_registers[1], tr_list, alu.b, 2);
      print_reg(&mut self.current_registers[2], tr_list, alu.c, 3);
    }
  }
}

fn print_flag(current_flag: &mut tms0800::control::Flag, tr_list: &web_sys::HtmlCollection, mut new_flag: tms0800::control::Flag, row_index: u32) {
  if new_flag != *current_flag {
    let td_list = tr_list.item(row_index).expect("can't get tr").children();
    for i in 0..11 {
      let col_index = i as u32 + 1;
      let bit = new_flag.read_bit(shifter::Direction::Left);
      if let Some(td) = td_list.item(col_index) {
        if bit {
          td.set_text_content(Some("●"));
        } else {
          td.set_text_content(Some("○"));
        }
      }
      new_flag.shift_with_bit(shifter::Direction::Left, bit);
    }
    *current_flag = new_flag.clone();
  }
}



fn print_reg(current_reg: &mut tms0800::alu::Register, tr_list: &web_sys::HtmlCollection, mut new_reg: tms0800::alu::Register, row_index: u32) {
  if new_reg != *current_reg {
    let td_list = tr_list.item(row_index).expect("can't get tr").children();
    for i in 0..11 {
      let col_index = i as u32 + 1;
      let nibble = new_reg.read_nibble(shifter::Direction::Left);
      if let Some(td) = td_list.item(col_index) {
        td.set_text_content(Some(&format!("{:X}", nibble)));
      }
      new_reg.shift_with_nibble(shifter::Direction::Left, nibble);
    }
    *current_reg = new_reg.clone();
  }
}

fn get_tr_list(document: &web_sys::Document, table_id: &str) -> Option<web_sys::HtmlCollection> {
  if let Some(table) = document.get_element_by_id(table_id) {
    if let Some(tbody) = table.children().item(0) {
      return Some(tbody.children())
    }
  }
  None
}