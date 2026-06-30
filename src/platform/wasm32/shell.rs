use crate::fs::FS;
use crate::platform::wasm32::{backspace, init_keyboard, poll_key};
use crate::{print, println};
use alloc::string::String;
use core::cell::RefCell;
use wasm_bindgen::{closure::Closure, JsCast, JsValue};

thread_local! {
    static LINE: RefCell<String> = RefCell::new(String::new());
    static TICK_FN: RefCell<Option<JsValue>> = RefCell::new(None);
}

pub fn start() {
    init_keyboard();
    print!("> ");

    let closure = Closure::wrap(Box::new(tick) as Box<dyn FnMut()>);
    let js_fn = closure.into_js_value();
    TICK_FN.with(|f| *f.borrow_mut() = Some(js_fn));

    schedule_tick();
}

fn schedule_tick() {
    let window = web_sys::window().expect("no window");
    TICK_FN.with(|f| {
        let js_fn = f.borrow().as_ref().expect("tick fn not set").clone();
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(js_fn.unchecked_ref(), 16)
            .expect("failed to schedule tick");
    });
}

fn tick() {
    while let Some(c) = poll_key() {
        match c {
            '\n' => {
                println!();
                let line: String = LINE.with(|l| l.borrow_mut().drain(..).collect());
                dispatch(line.trim());
                print!("> ");
            }
            '\x08' => {
                let empty = LINE.with(|l| {
                    let mut line = l.borrow_mut();
                    if !line.is_empty() {
                        line.pop();
                        false
                    } else {
                        true
                    }
                });
                if !empty {
                    backspace();
                }
            }
            c if c.is_ascii() => {
                LINE.with(|l| l.borrow_mut().push(c));
                print!("{}", c);
            }
            _ => {}
        }
    }
    schedule_tick();
}

fn dispatch(input: &str) {
    let mut parts = input.splitn(2, ' ');
    let cmd = parts.next().unwrap_or("");
    let arg = parts.next().unwrap_or("").trim();

    match cmd {
        "echo" => println!("{}", arg),
        "ls" => {
            let fs = FS.lock();
            for path in fs.list() {
                println!("{}", path);
            }
        }
        "cat" => {
            let fs = FS.lock();
            match fs.read(arg) {
                Some(data) => match core::str::from_utf8(data) {
                    Ok(s) => println!("{}", s),
                    Err(_) => println!("cat: {}: binary file", arg),
                },
                None => println!("cat: {}: not found", arg),
            }
        }
        "clear" => {
            for _ in 0..25 {
                println!();
            }
        }
        "help" => println!("commands: echo  ls  cat  clear  help"),
        "" => {}
        _ => println!("{}: command not found", cmd),
    }
}
