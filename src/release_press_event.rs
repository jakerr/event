
use {
    EventType,
    Field,
    KeyType,
    Observer,
    Triggered,
    Value,
};

/// A context event that can be triggered after certain key was released.
pub struct ReleasePressEvent<'a, 'b> {
    /// The key which was released.
    pub key: Field<'a, &'b KeyType>,
}

impl<'a, 'b> Clone for ReleasePressEvent<'a, 'b> {
    fn clone(&self) -> ReleasePressEvent<'static, 'b> {
        ReleasePressEvent {
            key: Value(*self.key.get()),
        }
    }
}

impl<'a, 'b> Triggered for ReleasePressEvent<'a, 'b> {
    fn get_observer(&self) -> Box<Observer> {
        box ReleasePressEventObserver::new(*self.key.get()) as Box<Observer>
    }
}

struct ReleasePressEventObserver<'a> {
    key: &'a KeyType,
    can_trigger: bool,
    is_pressed: bool,
}

impl<'a> ReleasePressEventObserver<'a> {
    pub fn new(key: &'a KeyType) -> ReleasePressEventObserver<'a>{
        ReleasePressEventObserver {
            key: key,
            can_trigger: false,
            is_pressed: false,
        }
    }
}

impl<'a> Observer for ReleasePressEventObserver<'a> {
    fn reset(&mut self) {
        self.is_pressed = false;
        self.can_trigger = false;
    }

    fn can_trigger(&self) -> bool {
        self.can_trigger
    }

    fn after_trigger(&mut self) {
        self.can_trigger = false
    }

    fn on_event(&mut self, e: &EventType) {
        if e.is_press_key(self.key) {
            self.is_pressed = true;
        } else if e.is_release_key(self.key) {
            if self.is_pressed {
                self.can_trigger = true;
            }
            self.is_pressed = false;
        }
    }
}
