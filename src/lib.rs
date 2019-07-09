mod example_view;

use pvm_plugins::{define_plugin, views::ViewCoordinator, Plugin};

struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn init() -> Self {
        ExamplePlugin
    }

    fn view_ops(&self, vc: &mut ViewCoordinator) {
        vc.register_view_type::<example_view::ExampleView>();
    }
}

define_plugin!(ExamplePlugin);

/*
Compact form for only loading views:

define_plugin!(views => [ example_view::ExampleView, example_view::ExampleView2 ]);

*/
