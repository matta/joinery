pub(crate) mod event {
    pub(crate) type KeyEvent = crossterm::event::KeyEvent;
    pub(crate) type Modifiers = crossterm::event::KeyModifiers;
    pub(crate) type KeyCode = crossterm::event::KeyCode;
    pub(crate) type KeyEventKind = crossterm::event::KeyEventKind;
}
pub(crate) mod keyboard {}
