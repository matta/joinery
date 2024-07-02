use smol_str::SmolStr;

use super::keyboard::{self, ModifiersKeyState, ModifiersKeys};

/// Describes a keyboard input targeting a window.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyEvent {
    // Allowing `broken_intra_doc_links` for `logical_key`, because
    // `key_without_modifiers` is not available on all platforms
    #[cfg_attr(
        not(any(windows_platform, macos_platform, x11_platform, wayland_platform)),
        allow(rustdoc::broken_intra_doc_links)
    )]
    /// This value is affected by all modifiers except <kbd>Ctrl</kbd>.
    ///
    /// This has two use cases:
    /// - Allows querying whether the current input is a Dead key.
    /// - Allows handling key-bindings on platforms which don't
    /// support [`key_without_modifiers`].
    ///
    /// If you use this field (or [`key_without_modifiers`] for that matter) for keyboard
    /// shortcuts, **it is important that you provide users with a way to configure your
    /// application's shortcuts so you don't render your application unusable for users with an
    /// incompatible keyboard layout.**
    ///
    /// ## Platform-specific
    /// - **Web:** Dead keys might be reported as the real key instead
    /// of `Dead` depending on the browser/OS.
    ///
    /// [`key_without_modifiers`]: crate::platform::modifier_supplement::KeyEventExtModifierSupplement::key_without_modifiers
    pub logical_key: keyboard::Key,

    /// Contains the text produced by this keypress.
    ///
    /// In most cases this is identical to the content
    /// of the `Character` variant of `logical_key`.
    /// However, on Windows when a dead key was pressed earlier
    /// but cannot be combined with the character from this
    /// keypress, the produced text will consist of two characters:
    /// the dead-key-character followed by the character resulting
    /// from this keypress.
    ///
    /// An additional difference from `logical_key` is that
    /// this field stores the text representation of any key
    /// that has such a representation. For example when
    /// `logical_key` is `Key::Named(NamedKey::Enter)`, this field is `Some("\r")`.
    ///
    /// This is `None` if the current keypress cannot
    /// be interpreted as text.
    ///
    /// See also: `text_with_all_modifiers()`
    pub text: Option<SmolStr>,

    /// Whether the key is being pressed or released.
    ///
    /// See the [`ElementState`] type for more details.
    pub state: ElementState,

    /// Whether or not this key is a key repeat event.
    ///
    /// On some systems, holding down a key for some period of time causes that key to be repeated
    /// as though it were being pressed and released repeatedly. This field is `true` if and only
    /// if this event is the result of one of those repeats.
    ///
    /// # Example
    ///
    /// In games, you often want to ignore repated key events - this can be
    /// done by ignoring events where this property is set.
    ///
    /// ```
    /// use winit::event::{ElementState, KeyEvent, WindowEvent};
    /// use winit::keyboard::{KeyCode, PhysicalKey};
    /// # let window_event = WindowEvent::RedrawRequested; // To make the example compile
    /// match window_event {
    ///     WindowEvent::KeyboardInput {
    ///         event:
    ///             KeyEvent {
    ///                 physical_key: PhysicalKey::Code(KeyCode::KeyW),
    ///                 state: ElementState::Pressed,
    ///                 repeat: false,
    ///                 ..
    ///             },
    ///         ..
    ///     } => {
    ///         // The physical key `W` was pressed, and it was not a repeat
    ///     },
    ///     _ => {}, // Handle other events
    /// }
    /// ```
    pub repeat: bool,
}

/// Describes keyboard modifiers event.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    pub(crate) state: ModifiersState,

    // NOTE: Currently pressed modifiers keys.
    //
    // The field providing a metadata, it shouldn't be used as a source of truth.
    pub(crate) pressed_mods: ModifiersKeys,
}

impl Modifiers {
    /// The state of the modifiers.
    pub fn state(&self) -> ModifiersState {
        self.state
    }

    /// The state of the left shift key.
    pub fn lshift_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::LSHIFT)
    }

    /// The state of the right shift key.
    pub fn rshift_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::RSHIFT)
    }

    /// The state of the left alt key.
    pub fn lalt_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::LALT)
    }

    /// The state of the right alt key.
    pub fn ralt_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::RALT)
    }

    /// The state of the left control key.
    pub fn lcontrol_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::LCONTROL)
    }

    /// The state of the right control key.
    pub fn rcontrol_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::RCONTROL)
    }

    /// The state of the left super key.
    pub fn lsuper_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::LSUPER)
    }

    /// The state of the right super key.
    pub fn rsuper_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::RSUPER)
    }

    fn mod_state(&self, modifier: ModifiersKeys) -> ModifiersKeyState {
        if self.pressed_mods.contains(modifier) {
            ModifiersKeyState::Pressed
        } else {
            ModifiersKeyState::Unknown
        }
    }
}

impl From<ModifiersState> for Modifiers {
    fn from(value: ModifiersState) -> Self {
        Self {
            state: value,
            pressed_mods: Default::default(),
        }
    }
}

/// Describes the input state of a key.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ElementState {
    Pressed,
    Released,
}

impl ElementState {
    /// True if `self == Pressed`.
    pub fn is_pressed(self) -> bool {
        self == ElementState::Pressed
    }
}

bitflags::bitflags! {
    /// Represents the state of the keyboard modifiers (shift, control, alt, etc.).
    ///
    /// **Note:** `SUPER`, `HYPER`, and `META` can only be read if
    /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    ///
    /// Note: this taken directly from crossterm
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ModifiersState: u8 {
        const SHIFT =     0b1; // (1)
        const ALT =       0b10;       // (2)
        const CONTROL =   0b100; //      (4)
        const SUPER =     0b1000;     // (8)
        const HYPER =     0b10000;   //  (16)
        const META =      0b100000;   // (32)
        const CAPS_LOCK = 0b1000000; //  (64)
        const NUM_LOCK =  0b10000000;  // (128)
        const NONE = 0;
    }
}

impl ModifiersState {
    /// Returns `true` if the shift key is pressed.
    pub fn shift_key(&self) -> bool {
        self.intersects(Self::SHIFT)
    }

    /// Returns `true` if the control key is pressed.
    pub fn control_key(&self) -> bool {
        self.intersects(Self::CONTROL)
    }

    /// Returns `true` if the alt key is pressed.
    pub fn alt_key(&self) -> bool {
        self.intersects(Self::ALT)
    }

    /// Returns `true` if the super key is pressed.
    pub fn super_key(&self) -> bool {
        self.intersects(Self::SUPER)
    }

    /// Returns `true` if the hyper key is pressed.
    pub fn hyper_key(&self) -> bool {
        self.intersects(Self::HYPER)
    }

    /// Returns `true` if the meta key is pressed.
    pub fn meta_key(&self) -> bool {
        self.intersects(Self::META)
    }
}
