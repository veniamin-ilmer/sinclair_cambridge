//! Each clock cycle ends up taking 280 microseconds. (3.671 kHz)

#![forbid(unsafe_code)]

use wasm_bindgen::prelude::wasm_bindgen;
use chips::cpu::TMS0800;

mod display;
mod keyboard;
mod rom;
mod side_panel;

const REFRESH_RATE: i32 = 20; //20 milliseconds => display at 50 hz.
const CLOCK_RATE: i32 = 264;  //Per the patent: "an instruction cycle is about 264 to 400 microseconds long, or instruction cycles occurring at a rate of roughly 2 to 4 KHz."
const CYCLES: i32 = REFRESH_RATE * 1000 / CLOCK_RATE;

/// Loop {
///   Sleep till the next screen refresh
///   Calculate how much instructions we should have run.
///   Run those instructions.
///   Run instructions until the calculated cycles matches the actual cycles.
///   Run IO instructions which should only work on each refresh
///   Draw everything.
/// }
#[wasm_bindgen]
pub async fn run() {
  std::panic::set_hook(Box::new(console_error_panic_hook::hook)); //Panics appear more descriptive in the browser console.

  //wasm_log::init(wasm_log::Config::new(log::Level::Trace));
  let mut chip = TMS0800::new(rom::decode(), rom::ALU_CONFIG, rom::WORD_SELECTS, rom::CONSTANTS);
  let mut side_panel = side_panel::SidePanel::new();
  let keyboard = keyboard::Keyboard::new();
  let mut display = display::Display::new();

  let window = web_sys::window().unwrap();

  loop {
    for _ in 0..CYCLES {
      chip.run_cycle();
    }

    keyboard.run_refresh_cycle(&mut chip);
    display.run_refresh_cycle(&chip);
    
    side_panel.run_refresh_cycle(&chip);

    sleep(&window, REFRESH_RATE).await;
  }
}

/// A trick to get browsers to "sleep" by awaiting a set_timeout
async fn sleep(window: &web_sys::Window, ms: i32) {
  let promise = js_sys::Promise::new(&mut |resolve, _| {
    window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms).unwrap();
  });
  let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}
