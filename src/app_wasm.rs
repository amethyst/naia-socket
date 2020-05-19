
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use gaia_client_socket::{ClientSocket, ClientSocketImpl, SocketEvent, MessageSender};

pub use crate::app::App;

impl App {
    pub fn start_loop(self) {
        fn request_animation_frame(f: &Closure<FnMut()>) {
            web_sys::window().unwrap()
                .request_animation_frame(f.as_ref().unchecked_ref())
                .expect("should register `requestAnimationFrame` OK");
        }

        info!("starting loop");

        let mut rc = Rc::new(self);
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let c = move || {
            if let Some(the_self) = Rc::get_mut(&mut rc) {
                the_self.update();
            };
            request_animation_frame(f.borrow().as_ref().unwrap());
        };

        *g.borrow_mut() = Some(Closure::wrap(Box::new(c) as Box<FnMut()>));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}