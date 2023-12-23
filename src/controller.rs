use gilrs::{Button, Event, EventType, GamepadId, Gilrs};

use crate::prelude::Controller::{Playstation, Switch, Xbox};
pub use gilrs::Error;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///
/// ```
/// # use simple_game_utils::controller::GameController;
/// let mut controller = GameController::new().expect("Unable to init controller");
///
/// loop {
///     controller.update();
///     if controller.action.north {
///         println!("Triangle pressed");
///     }
///#    break;
/// }
/// ```
#[derive(Debug)]
pub struct GameController {
    gilrs: Gilrs,
    pub direction: DirectionState,
    pub action: ActionState,
    pub menu: MenuState,
    last_connected: Option<GamepadId>,
}

impl GameController {
    pub fn new() -> Result<Self, Error> {
        Ok(GameController {
            gilrs: Gilrs::new()?,
            direction: DirectionState::default(),
            action: ActionState::default(),
            menu: MenuState::default(),
            last_connected: None,
        })
    }

    pub fn new_unchecked() -> Self {
        Self::new().expect("Failed to init controller")
    }

    pub fn from_state(
        direction: DirectionState,
        action: ActionState,
        menu: MenuState,
    ) -> Result<Self, Error> {
        Ok(GameController {
            gilrs: Gilrs::new()?,
            direction,
            action,
            menu,
            last_connected: None,
        })
    }

    pub fn from_state_unchecked(
        direction: DirectionState,
        action: ActionState,
        menu: MenuState,
    ) -> Self {
        Self::from_state(direction, action, menu).expect("Failed to init controller")
    }
}

impl GameController {
    pub fn any_connected(&self) -> bool {
        self.gilrs.gamepads().any(|(_, pad)| pad.is_connected())
    }

    pub fn to_state(&self) -> (DirectionState, ActionState, MenuState) {
        (
            self.direction.clone(),
            self.action.clone(),
            self.menu.clone(),
        )
    }

    pub fn mask(&self) -> u16 {
        self.menu.to_mask() | self.action.to_mask() | self.direction.to_mask()
    }

    pub fn update(&mut self) {
        while let Some(Event { id, event, time: _ }) = self.gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _code) => self.set_state(button, true),
                EventType::ButtonRepeated(_, _) => {}
                EventType::ButtonReleased(button, _code) => {
                    self.last_connected = Some(id);
                    self.set_state(button, false);
                }
                EventType::ButtonChanged(_, _, _) => {}
                EventType::AxisChanged(_, _, _) => {}
                EventType::Connected => {}
                EventType::Disconnected => {
                    if self.last_connected == Some(id) {
                        self.last_connected = None;
                    }
                }
                EventType::Dropped => {}
            }
        }
    }

    fn set_state(&mut self, button: Button, pressed: bool) {
        match button {
            Button::South => self.action.south = pressed,
            Button::East => self.action.east = pressed,
            Button::North => self.action.north = pressed,
            Button::West => self.action.west = pressed,
            Button::C => {}
            Button::Z => {}
            Button::LeftTrigger => {}
            Button::LeftTrigger2 => {}
            Button::RightTrigger => {}
            Button::RightTrigger2 => {}
            Button::Select => {}
            Button::Start => self.menu.start = pressed,
            Button::Mode => {}
            Button::LeftThumb => {}
            Button::RightThumb => {}
            Button::DPadUp => self.direction.up = pressed,
            Button::DPadDown => self.direction.down = pressed,
            Button::DPadLeft => self.direction.left = pressed,
            Button::DPadRight => self.direction.right = pressed,
            Button::Unknown => {}
        }
    }

    // using http://www.linux-usb.org/usb.ids as source
    pub fn get_controller_type(&self) -> Option<Controller> {
        if let Some(id) = self.last_connected {
            let uuid = self.gilrs.gamepad(id).uuid();
            let vendor = u16::from_le_bytes([uuid[4], uuid[5]]);
            let product = u16::from_le_bytes([uuid[8], uuid[9]]);
            match (vendor, product) {
                (0x54c, 0xdf2 | 0xce6 | 0xcda | 0x9cc | 0x5c4 | 0x268) => Some(Playstation),
                (
                    0x45e,
                    0x202 | 0x285 | 0x289 | 0x28e | 0x28f | 0x2d1 | 0x2dd | 0x2e0 | 0x2e3 | 0x2ea
                    | 0x2fd | 0xb12 | 0xb00,
                ) => Some(Xbox),
                (0x57e, 0x2009) => Some(Switch),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Controller {
    Playstation,
    Xbox,
    Switch,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct DirectionState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl DirectionState {
    pub fn to_mask(&self) -> u16 {
        (if self.up { CNTR_MASK_UP } else { 0 })
            | (if self.down { CNTR_MASK_DOWN } else { 0 })
            | (if self.left { CNTR_MASK_LEFT } else { 0 })
            | (if self.right { CNTR_MASK_RIGHT } else { 0 })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct MenuState {
    pub start: bool,
}

impl MenuState {
    pub fn to_mask(&self) -> u16 {
        if self.start {
            CNTR_MASK_START
        } else {
            0
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct ActionState {
    pub north: bool,
    pub south: bool,
    pub west: bool,
    pub east: bool,
}

impl ActionState {
    pub fn to_mask(&self) -> u16 {
        (if self.north { CNTR_MASK_NORTH } else { 0 })
            | (if self.south { CNTR_MASK_SOUTH } else { 0 })
            | (if self.west { CNTR_MASK_WEST } else { 0 })
            | (if self.east { CNTR_MASK_EAST } else { 0 })
    }
}

pub const CNTR_MASK_UP: u16 = 0b10000000_00000000;
pub const CNTR_MASK_DOWN: u16 = 0b01000000_00000000;
pub const CNTR_MASK_LEFT: u16 = 0b00100000_00000000;
pub const CNTR_MASK_RIGHT: u16 = 0b00010000_00000000;
pub const CNTR_MASK_START: u16 = 0b00000001_00000000;
pub const CNTR_MASK_NORTH: u16 = 0b00000000_00001000;
pub const CNTR_MASK_SOUTH: u16 = 0b00000000_00000100;
pub const CNTR_MASK_EAST: u16 = 0b00000000_00000010;
pub const CNTR_MASK_WEST: u16 = 0b00000000_00000001;

#[cfg(test)]
mod test {
    use crate::prelude::{GameController, CNTR_MASK_EAST, CNTR_MASK_UP};

    #[test]
    fn check_mask() {
        let mut controller = GameController::new_unchecked();
        controller.direction.up = true;

        assert_eq!(controller.mask(), CNTR_MASK_UP);

        controller.action.east = true;

        assert_eq!(controller.mask(), CNTR_MASK_UP | CNTR_MASK_EAST);
    }
}
