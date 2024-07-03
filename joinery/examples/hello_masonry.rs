// Copyright 2019 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! This is a very small example of how to setup a masonry application.
//! It does the almost bare minimum while still being useful.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use joinery::app_driver::{AppDriver, DriverCtx};
use joinery::widget::{Button, Flex, Label, RootWidget};
use joinery::{Action, WidgetId};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

struct Driver;

impl AppDriver for Driver {
    fn on_action(&mut self, _ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed(_) => {
                println!("Hello");
            }
            action => {
                eprintln!("Unexpected action {action:?}");
            }
        }
    }
}

pub fn main() {
    let label = Label::new("Hello").with_text_size(32.0);
    let button = Button::new("Say hello");

    // Arrange the two widgets vertically, with some padding
    let main_widget = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button);

    joinery::event_loop_runner::run(RootWidget::new(main_widget), Driver).unwrap();
}
