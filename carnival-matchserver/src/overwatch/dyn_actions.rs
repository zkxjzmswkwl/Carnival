use std::{thread, time};

use crate::input;

// Goal: synthetic mouse inputs (static positions/timing), but allow for
// variadic keyboard inputs.

// Notes:
//  - Because we are going to be using this primarily (at first entirely) for
//    inviting players to the lobby, we can do something like
//    "InviteAction" which implements `invoke`, but can be created
//    from a player object?? From a use standpoint this seems like a
//    decently neat option.
//
// Example usage-ish
//  - InviteAction::from_str(&battletag)

#[derive(Debug)]
pub enum ActionType {
    Keyboard,
    Mouse,
}

#[derive(Debug)]
pub struct DynamicActionChain(Vec<DynamicAction>);

impl DynamicActionChain {
    // Team: 1 = blue, 2 = red
    pub fn generate_invite_chain(battletag: String, team: u8) -> Self {
        let mut ret = vec![
            DynamicAction::new(ActionType::Mouse, String::from(""), 1675, 250, 25),
            DynamicAction::new(ActionType::Mouse, String::from(""), 1244, 780, 150),
            DynamicAction::new(ActionType::Mouse, String::from(""), 1204, 867, 25),
            DynamicAction::new(ActionType::Mouse, String::from(""), 1159, 242, 25),
            DynamicAction::new(ActionType::Keyboard, battletag, 0, 0, 25),
            DynamicAction::new(ActionType::Mouse, String::from(""), 1026, 892, 500),
        ];
        // Red team invite adjustment.
        if team == 2 {
            ret.get_mut(2).unwrap().y = 906;
        }
        Self { 0: ret }
    }

    pub fn invoke(&self) {
        self.0.iter().for_each(|action| action.invoke());
    }
}

#[derive(Debug)]
pub struct DynamicAction {
    action_type: ActionType,
    input: String,
    x: i32,
    y: i32,
    delay: u64,
}

impl DynamicAction {
    pub fn new(action_type: ActionType, input: String, x: i32, y: i32, delay: u64) -> Self {
        Self {
            action_type,
            input,
            x,
            y,
            delay,
        }
    }

    pub fn invoke(&self) {
        match self.action_type {
            ActionType::Keyboard => {
                // Overwatch's UI needs some time to catch up when it comes to text input.
                thread::sleep(time::Duration::from_millis(100));
                input::type_str(&self.input, self.delay);
            },
            ActionType::Mouse    => {
                thread::sleep(time::Duration::from_millis(self.delay));
                input::click(self.x, self.y);
            }
        }
    }
}
