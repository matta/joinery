use crate::{
    app_driver::AppDriver,
    render_root::{RenderRoot, WindowSizePolicy},
    PointerState, TextEvent, Widget, WindowEvent,
};

use std::io::stdout;

use dpi::PhysicalSize;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        self,
        event::{KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};
use smol_str::SmolStr;

type EventLoopError = std::io::Error;

struct MainState {
    render_root: RenderRoot,
    pointer_state: PointerState,
    #[allow(dead_code)] // TODO: remove me
    app_driver: Box<dyn AppDriver>,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    quit: bool,
}
impl MainState {
    fn crossterm_event(&mut self, event: crossterm::event::Event) {
        tracing::info!("event_loop_runner got crossterm event: {:?}", event);
        use crossterm::event::Event;
        match event {
            Event::FocusGained => {
                self.render_root
                    .handle_text_event(TextEvent::FocusChange(true));
            }
            Event::FocusLost => {
                self.render_root
                    .handle_text_event(TextEvent::FocusChange(false));
            }
            Event::Key(event) => {
                if event.kind == KeyEventKind::Press && event.code == KeyCode::Char('q') {
                    tracing::info!("Got q, quitting");
                    self.quit = true;
                }
                self.crossterm_key_event(event);
            }
            Event::Mouse(event) => {
                tracing::warn!("Ignoring mouse event: {:?}", event);
            }
            Event::Paste(event) => {
                tracing::warn!("Ignoring paste event: {:?}", event);
            }
            Event::Resize(width, height) => {
                let width: u32 = width.into();
                let height: u32 = height.into();
                let size = PhysicalSize::new(width, height);
                self.render_root
                    .handle_window_event(WindowEvent::Resize(size));
            }
        }
    }

    fn crossterm_key_event(&mut self, event: crossterm::event::KeyEvent) {
        self.handle_crossterm_key_event(&event);
    }

    fn handle_crossterm_key_modifiers(&mut self, event: &crossterm::event::KeyEvent) {
        use crate::terminal::{event::Modifiers, keyboard::ModifiersState};
        use crossterm::event::KeyModifiers;

        let mut modifiers = Modifiers::default();

        const MAPPINGS: [(KeyModifiers, ModifiersState); 4] = [
            (KeyModifiers::SHIFT, ModifiersState::SHIFT),
            (KeyModifiers::CONTROL, ModifiersState::CONTROL),
            (KeyModifiers::ALT, ModifiersState::ALT),
            (KeyModifiers::SUPER, ModifiersState::SUPER),
            // (KeyModifiers::HYPER, no equiavlent),
            // (KeyModifiers::META, no equivalent),
        ];
        for (a, b) in MAPPINGS {
            if event.modifiers.contains(a) {
                modifiers.state.insert(b);
            }
        }

        // if let crossterm::event::KeyCode::Modifier(modifier) = event.code {
        //     // We could possibly track the state of individual modifier keys
        //     // by looking at KeyCode::Modifier, but this is tricky work for
        //     // dubious value, so we do nothing.
        // }

        if self.pointer_state.mods != modifiers {
            self.pointer_state.mods = modifiers;
            self.render_root
                .handle_text_event(TextEvent::ModifierChange(modifiers.state()));
        }
    }

    fn handle_crossterm_key_event(&mut self, event: &crossterm::event::KeyEvent) {
        use crate::terminal::keyboard::{Key, ModifiersState, NamedKey};

        self.handle_crossterm_key_modifiers(event);

        // The textual representation of the keyboard event. This is set as a
        // side effect of computing the logical key.
        let mut text: Option<SmolStr> = None;

        let logical_key: Option<crate::terminal::keyboard::Key> = match event.code {
            KeyCode::Backspace => Some(Key::Named(crate::terminal::keyboard::NamedKey::Backspace)),
            KeyCode::Enter => Some(Key::Named(NamedKey::Enter)),
            KeyCode::Left => Some(Key::Named(NamedKey::ArrowLeft)),
            KeyCode::Right => Some(Key::Named(NamedKey::ArrowRight)),
            KeyCode::Up => Some(Key::Named(NamedKey::ArrowUp)),
            KeyCode::Down => Some(Key::Named(NamedKey::ArrowDown)),
            KeyCode::Home => Some(Key::Named(NamedKey::Home)),
            KeyCode::End => Some(Key::Named(NamedKey::End)),
            KeyCode::PageUp => Some(Key::Named(NamedKey::PageUp)),
            KeyCode::PageDown => Some(Key::Named(NamedKey::PageDown)),
            KeyCode::Tab => Some(Key::Named(NamedKey::Tab)),
            KeyCode::BackTab => {
                assert!(
                    self.pointer_state
                        .mods
                        .state
                        .contains(ModifiersState::SHIFT),
                    "BackTab without shift modifier"
                );
                Some(Key::Named(NamedKey::Tab))
            }
            KeyCode::Delete => Some(Key::Named(NamedKey::Delete)),
            KeyCode::Insert => Some(Key::Named(NamedKey::Insert)),
            KeyCode::F(n) => translate_function_key_number(n),
            KeyCode::Char(ch) => {
                let ch: SmolStr = ch.to_string().into();
                text = Some(ch.clone());
                Some(Key::Character(ch))
            }
            KeyCode::Null => {
                tracing::warn!("Received KeyCode::Null from crossterm (ignoring)");
                None
            }
            KeyCode::Esc => Some(Key::Named(NamedKey::Escape)),
            KeyCode::CapsLock => {
                tracing::warn!("Received KeyCode::CapsLock from crossterm (ignoring)");
                None
            }
            KeyCode::ScrollLock => {
                tracing::warn!("Received KeyCode::ScrollLock from crossterm (ignoring)");
                None
            }
            KeyCode::NumLock => {
                tracing::warn!("Received KeyCode::NumLock from crossterm (ignoring)");
                None
            }
            KeyCode::PrintScreen => Some(Key::Named(NamedKey::PrintScreen)),
            KeyCode::Pause => Some(Key::Named(NamedKey::Pause)),
            KeyCode::Menu => Some(Key::Named(NamedKey::ContextMenu)),
            KeyCode::KeypadBegin => {
                tracing::warn!("Received KeyCode::KeypadBegin from crossterm (ignoring)");
                None
            }
            KeyCode::Media(key) => translate_media_key_code(key),
            KeyCode::Modifier(key) => {
                tracing::warn!(
                    "Received KeyCode::Modifier({:?}) from crossterm (ignoring)",
                    key
                );
                None
            }
        };

        let state: crate::terminal::event::ElementState = match event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => {
                crate::terminal::event::ElementState::Pressed
            }
            KeyEventKind::Release => crate::terminal::event::ElementState::Released,
        };

        let repeat = event.kind == KeyEventKind::Repeat;

        if let Some(logical_key) = logical_key {
            let event = crate::terminal::event::KeyEvent {
                logical_key,
                text,
                state,
                repeat,
            };
            self.render_root.handle_text_event(TextEvent::KeyboardKey(
                event,
                self.pointer_state.mods.state(),
            ));
        } else {
            tracing::warn!("Ignoring unknown key from crossterm: {:?}", event);
        }
    }
}

fn translate_function_key_number(n: u8) -> Option<crate::terminal::keyboard::Key> {
    use crate::terminal::keyboard::{Key, NamedKey};

    if (1..=35).contains(&n) {
        let key = match n {
            1 => NamedKey::F1,
            2 => NamedKey::F2,
            3 => NamedKey::F3,
            4 => NamedKey::F4,
            5 => NamedKey::F5,
            6 => NamedKey::F6,
            7 => NamedKey::F7,
            8 => NamedKey::F8,
            9 => NamedKey::F9,
            10 => NamedKey::F10,
            11 => NamedKey::F11,
            12 => NamedKey::F12,
            13 => NamedKey::F13,
            14 => NamedKey::F14,
            15 => NamedKey::F15,
            16 => NamedKey::F16,
            17 => NamedKey::F17,
            18 => NamedKey::F18,
            19 => NamedKey::F19,
            20 => NamedKey::F20,
            21 => NamedKey::F21,
            22 => NamedKey::F22,
            23 => NamedKey::F23,
            24 => NamedKey::F24,
            25 => NamedKey::F25,
            26 => NamedKey::F26,
            27 => NamedKey::F27,
            28 => NamedKey::F28,
            29 => NamedKey::F29,
            30 => NamedKey::F30,
            31 => NamedKey::F31,
            32 => NamedKey::F32,
            33 => NamedKey::F33,
            34 => NamedKey::F34,
            35 => NamedKey::F35,
            _ => unreachable!(),
        };
        Some(Key::Named(key))
    } else {
        tracing::warn!("Received KeyCode::F({}) from crossterm (ignoring)", n);
        None
    }
}

fn translate_media_key_code(
    key: crossterm::event::MediaKeyCode,
) -> Option<crate::terminal::keyboard::Key> {
    use crate::terminal::keyboard::{Key, NamedKey};
    use crossterm::event::MediaKeyCode;

    match key {
        MediaKeyCode::Play => Some(Key::Named(crate::terminal::keyboard::NamedKey::MediaPlay)),
        MediaKeyCode::Pause => Some(Key::Named(NamedKey::MediaPause)),
        MediaKeyCode::PlayPause => Some(Key::Named(NamedKey::MediaPlayPause)),
        MediaKeyCode::Reverse => {
            tracing::warn!("Received MediaKeyCode::Reverse from crossterm (ignoring)");
            None
        }
        MediaKeyCode::Stop => Some(Key::Named(NamedKey::MediaStop)),
        MediaKeyCode::FastForward => Some(Key::Named(NamedKey::MediaFastForward)),
        MediaKeyCode::Rewind => Some(Key::Named(NamedKey::MediaRewind)),
        MediaKeyCode::TrackNext => Some(Key::Named(NamedKey::MediaTrackNext)),
        MediaKeyCode::TrackPrevious => Some(Key::Named(NamedKey::MediaTrackPrevious)),
        MediaKeyCode::Record => Some(Key::Named(NamedKey::MediaRecord)),
        MediaKeyCode::LowerVolume => Some(Key::Named(NamedKey::AudioVolumeDown)),
        MediaKeyCode::RaiseVolume => Some(Key::Named(NamedKey::AudioVolumeUp)),
        MediaKeyCode::MuteVolume => Some(Key::Named(NamedKey::AudioVolumeMute)),
    }
}

pub fn run(
    // Clearly, this API needs to be refactored, so we don't mind forcing this to be passed in here directly
    // This is passed in mostly to allow configuring the Android app
    root_widget: impl Widget,
    app_driver: impl AppDriver + 'static,
) -> Result<(), EventLoopError> {
    run_with(root_widget, app_driver)
}

pub fn run_with(
    root_widget: impl Widget,
    app_driver: impl AppDriver + 'static,
) -> Result<(), EventLoopError> {
    // If there is no default tracing subscriber, we set our own. If one has
    // already been set, we get an error which we swallow.
    // By now, we're about to take control of the event loop. The user is unlikely
    // to try to set their own subscriber once the event loop has started.
    let _ = crate::tracing_backend::try_init_tracing();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // TODO: We can't know this scale factor until later?
    let scale_factor = 1.0;
    let render_root = RenderRoot::new(root_widget, WindowSizePolicy::User, scale_factor);

    let mut main_state = MainState {
        render_root,
        pointer_state: PointerState::empty(),
        app_driver: Box::new(app_driver),
        terminal,
        quit: false,
    };

    run_app(&mut main_state)?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    #[cfg(any())]
    {
        let render_cx = RenderContext::new();
        // TODO: We can't know this scale factor until later?
        let scale_factor = 1.0;
        let mut main_state = MainState {
            render_cx,
            render_root: RenderRoot::new(root_widget, WindowSizePolicy::User, scale_factor),
            renderer: None,
            pointer_state: PointerState::empty(),
            app_driver: Box::new(app_driver),
            proxy: event_loop.create_proxy(),

            window: WindowState::Uninitialized(window),
        };

        // If there is no default tracing subscriber, we set our own. If one has
        // already been set, we get an error which we swallow.
        // By now, we're about to take control of the event loop. The user is unlikely
        // to try to set their own subscriber once the event loop has started.
        let _ = crate::tracing_backend::try_init_tracing();

        event_loop.run_app(&mut main_state)
    }

    Ok(())
}

fn run_app(main_state: &mut MainState) -> Result<(), std::io::Error> {
    while !main_state.quit {
        main_state.terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            main_state.crossterm_event(crossterm::event::read()?);
        }
    }
    Ok(())
}
