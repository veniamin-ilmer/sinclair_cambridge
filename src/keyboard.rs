use wasm_bindgen::{JsValue, JsCast};
use chips::cpu::tms0800;

pub(super) struct Keyboard {
  pending_button_var: wasm_bindgen::JsValue,
}

impl Keyboard {
  pub(super) fn new() -> Self {
    let pending_button_var = js_sys::Reflect::get(
        &wasm_bindgen::JsValue::from(web_sys::window().unwrap()),
        &wasm_bindgen::JsValue::from("getPendingButton"),
    ).unwrap();
    
    Self {
      pending_button_var,
    }
  }
  
  pub(super) fn run_refresh_cycle(&self, chip: &mut tms0800::TMS0800) {
    let pending_click_func: &js_sys::Function = self.pending_button_var.dyn_ref().unwrap();
    let click_var = pending_click_func.apply(&JsValue::null(), &js_sys::Array::new()).unwrap();
    if let Some(click_float) = click_var.as_f64() {
      let code = click_float.round(); //Wish there were a way to get an integer directly without needing to go through a float...
      
      if code == -1.0 { //Depress event
        chip.current_keypress = 0;
      } else {
        chip.current_keypress = code as u16;
      }
    }
  }

}
