use alloc::collections::VecDeque;
use spin::Mutex;
use wasm_bindgen::JsCast;

static KEY_BUF: Mutex<VecDeque<char>> = Mutex::new(VecDeque::new());

pub fn init_keyboard() {
    let window = web_sys::window().expect("no window");
    let closure = wasm_bindgen::closure::Closure::wrap(Box::new(|event: web_sys::KeyboardEvent| {
        let key = event.key();
        if key == "Enter" {
            KEY_BUF.lock().push_back('\n');
        } else if key == "Backspace" {
            KEY_BUF.lock().push_back('\x08');
        } else if key.len() == 1 {
            if let Some(c) = key.chars().next() {
                if c.is_ascii() {
                    KEY_BUF.lock().push_back(c);
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .expect("failed to add keydown listener");

    closure.forget();
}

pub fn poll_key() -> Option<char> {
    KEY_BUF.lock().pop_front()
}
