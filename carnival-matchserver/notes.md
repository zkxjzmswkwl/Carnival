Shit to do

- Action
    - Should represent any necessary information to invoke a state change on the client.
    - Should contain logic to invoke that action based on self-contained data.
    - Likely need something along the lines of `ActionType` to denote mouse/keyboard input.

- Actions
    - Should contain HashMap<String, Vec<Rc<Action>>>
        - Key being given name of the action.
    - Should contain logic to invoke actions in an order specified by configuration file(s).

