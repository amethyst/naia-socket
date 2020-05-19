
pub use crate::app::App;

impl App {
    pub fn start_loop(&mut self) {
        loop {
            self.update();
        }
    }
}