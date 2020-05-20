
cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use std::cell::RefCell;
        use std::rc::Rc;

        use log::{info};

        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        use crate::app::App;

        pub fn start_loop(app: App) {
            fn request_animation_frame(f: &Closure<dyn FnMut()>) {
                web_sys::window().unwrap()
                    .request_animation_frame(f.as_ref().unchecked_ref())
                    .expect("should register `requestAnimationFrame` OK");
            }

            info!("starting loop");

            let mut rc = Rc::new(app);
            let f = Rc::new(RefCell::new(None));
            let g = f.clone();

            let c = move || {
                if let Some(the_app) = Rc::get_mut(&mut rc) {
                    the_app.update();
                };
                request_animation_frame(f.borrow().as_ref().unwrap());
            };

            *g.borrow_mut() = Some(Closure::wrap(Box::new(c) as Box<dyn FnMut()>));

            request_animation_frame(g.borrow().as_ref().unwrap());
        }
    } else {}
}