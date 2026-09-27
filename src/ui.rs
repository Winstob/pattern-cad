pub mod layout;

use layout::Layout;

pub struct Ui {
    pub layout: Layout,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            layout: Layout::new(),
        }
    }

    pub fn clear(&mut self) {
        self.layout.clear();
    }
}
