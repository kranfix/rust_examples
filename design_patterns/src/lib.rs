use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Default)]
pub struct LightState(Option<Color>);

impl LightState {
    fn new(color: Color) -> LightState {
        LightState(Some(color))
    }

    fn color(&self) -> Option<&Color> {
        self.0.as_ref()
    }
}

#[derive(Debug)]
pub enum Turn {
    On(Color),
    Off,
}

trait Command<T> {
    // Returns true whether the command was excecuted
    fn execute(&self, light: &LightState) -> Option<LightState>;
}

impl Command<LightState> for Turn {
    fn execute(&self, light: &LightState) -> Option<LightState> {
        match self {
            Turn::On(new_color) => match light.color() {
                Some(color) if *color == *new_color => None,
                _ => Some(LightState::new(new_color.clone())),
            },
            Turn::Off => light.color().map(|_| LightState::default()),
        }
    }
}

#[derive(Debug, Default)]
pub struct LightController {
    stack: StackHistory<LightState>,
}

impl LightController {
    pub fn new(initial_value: LightState) -> LightController {
        LightController {
            stack: StackHistory::new(initial_value),
        }
    }

    pub fn run_cmd(&mut self, cmd: Turn) {
        if let Some(light) = cmd.execute(&self.stack) {
            self.stack.push(light);
        }
    }

    pub fn undo(&mut self) -> bool {
        self.stack.prev().is_some()
    }

    pub fn redo(&mut self) -> bool {
        self.stack.next().is_some()
    }

    pub fn color(&self) -> Option<&Color> {
        self.stack.color()
    }
}

trait History<State> {
    fn push(&mut self, new_state: State);
    fn next(&mut self) -> Option<&State>;
    fn prev(&mut self) -> Option<&State>;
}

#[derive(Debug)]
struct StackHistory<T> {
    stack: Vec<T>,
    index: usize,
}

impl<T: Default> Default for StackHistory<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> StackHistory<T> {
    pub fn new(initial_value: T) -> Self {
        StackHistory {
            stack: vec![initial_value],
            index: 0,
        }
    }
}

impl<T> Deref for StackHistory<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.stack[self.index]
    }
}

impl<T> History<T> for StackHistory<T> {
    fn push(&mut self, new_state: T) {
        self.index += 1;
        if self.index < self.stack.len() {
            self.stack.truncate(self.index);
        }
        self.stack.push(new_state);
    }

    fn next(&mut self) -> Option<&T> {
        if self.index < self.stack.len() {
            self.index += 1
        }
        self.stack.get(self.index)
    }

    fn prev(&mut self) -> Option<&T> {
        if self.index == 0 {
            None
        } else {
            self.index -= 1;
            self.stack.get(self.index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut dispatcher = LightController::new(LightState::default());
        assert!(dispatcher.color().is_none());

        dispatcher.run_cmd(Turn::On(Color::Red));
        assert_eq!(*dispatcher.color().unwrap(), Color::Red);

        dispatcher.run_cmd(Turn::On(Color::Green));
        assert_eq!(*dispatcher.color().unwrap(), Color::Green);

        dispatcher.run_cmd(Turn::On(Color::Green));
        assert_eq!(*dispatcher.color().unwrap(), Color::Green);

        assert!(dispatcher.undo());
        assert_eq!(*dispatcher.color().unwrap(), Color::Red);

        assert!(dispatcher.redo());
        assert_eq!(*dispatcher.color().unwrap(), Color::Green);

        assert!(dispatcher.undo());
        assert_eq!(*dispatcher.color().unwrap(), Color::Red);

        assert!(dispatcher.undo());
        assert!(dispatcher.color().is_none());

        assert!(!dispatcher.undo());
    }

    #[test]
    fn test_color_eq() {
        assert!(Color::Red == Color::Red);
        assert!(Color::Red != Color::Green);
        assert!(Color::Red != Color::Blue);

        assert!(Color::Green == Color::Green);
        assert!(Color::Green != Color::Blue);

        assert!(Color::Blue == Color::Blue);
    }
}
