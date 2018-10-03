#[derive(Debug)]
pub struct Switch {
    value: bool
}

impl Switch {
    ///Creates a new switch, with the default value of false.
    pub fn new() -> Switch {
        return Switch {
            value: Switch::default_value()
        };
    }

    ///The default value of the switch.
    pub fn default_value() -> bool {
        return false;
    }

    ///Initializes a new switch, given an initial value.
    pub fn init(value: bool) -> Switch {
        return Switch {
            value
        };
    }

    ///The invert method simply inverts the value that the switch is holding.
    /// Example: if the switch is false, it is true. If it is true, it is false.
    pub fn invert(&mut self) -> bool {
        self.value = !self.value;
        return self.value;
    }

    ///Returns our switch value, then inverts it.
    pub fn get(&mut self) -> bool {
        return !self.invert();
    }

    ///Returns our switch value without inverting it.
    pub fn view(&self) -> bool {
        return self.value;
    }
}