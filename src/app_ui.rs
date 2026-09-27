use crate::ui::{
    Ui,
    layout::{
        LayoutNodeOptions,
        LayoutNodeSize,
    },
};

pub struct AppUi {
    pub ui: Ui,
}

impl AppUi {
    pub fn new() -> Self {
        AppUi {
            ui: Ui::new(),
        }
    }

    pub fn init(&mut self) {
        self.ui.clear();
        let mut root = self.ui.layout.new_column(LayoutNodeOptions::default());

        let toolbar = self.ui.layout.new_row(LayoutNodeOptions {
            size: LayoutNodeSize::Fixed(50.0),
        });
        let other_stuff = self.ui.layout.new_row(LayoutNodeOptions {
            size: LayoutNodeSize::Flex(1.0),
        });
        self.ui.layout.add_child(root, toolbar);
        self.ui.layout.add_child(root, other_stuff);
    }
}
