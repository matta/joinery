// Copyright 2019 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! This is a very small example of how to setup a joinery application.
//! It does the almost bare minimum while still being useful.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use joinery::app_driver::{AppDriver, DriverCtx};
use joinery::widget::{Button, Flex, Label, Portal, RootWidget, Textbox, WidgetMut};
use joinery::{Action, WidgetId};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;

struct Driver {
    next_task: String,
}

impl AppDriver for Driver {
    fn on_action(&mut self, ctx: &mut DriverCtx<'_>, _widget_id: WidgetId, action: Action) {
        match action {
            Action::ButtonPressed(_) => {
                let mut root: WidgetMut<RootWidget<Portal<Flex>>> = ctx.get_root();
                let mut root = root.get_element();
                let mut flex = root.child_mut();
                flex.add_child(Label::new(self.next_task.clone()));

                let mut first_row = flex.child_mut(0).unwrap();
                let mut first_row = first_row.downcast::<Flex>();
                let mut textbox = first_row.child_mut(0).unwrap();
                let mut textbox = textbox.downcast::<Textbox>();
                textbox.reset_text(String::new());
            }
            Action::TextChanged(new_text) => {
                self.next_task = new_text.clone();
            }
            _ => {}
        }
    }
}

pub fn main() {
    let main_widget = Portal::new(
        Flex::column()
            .with_child(
                Flex::row()
                    .with_flex_child(Textbox::new(""), 1.0)
                    .with_child(Button::new("Add task")),
            )
            .with_spacer(VERTICAL_WIDGET_SPACING),
    );

    joinery::event_loop_runner::run(
        RootWidget::new(main_widget),
        Driver {
            next_task: String::new(),
        },
    )
    .unwrap();
}
