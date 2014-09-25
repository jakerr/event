use std::intrinsics::{ get_tydesc, TypeId };
use std::any::{ Any, AnyRefExt };
use std::fmt::Show;
use input::{
    Button,
    InputEvent,
    Press,
    Release,
    Move,
    MouseCursor,
    MouseRelative,
    MouseScroll,
};

use {
    MouseCursorEvent,
    MouseRelativeEvent,
    MouseScrollEvent,
    PressEvent,
    ReleaseEvent,
};

/// Used as generic constraint for events.
///
/// Methods should not be called directly.
///
/// An `event_trait_id` is created from `TypeId::of::<Box<Trait>>()`.
/// The implementation need to know the argument type for all event traits.
/// Implementations of `GenericEvent` should be unit tested.
pub trait GenericEvent {
    /// Creates a new event.
    fn from_event(event_trait_id: TypeId, args: &Any) -> Option<Self>;
    /// When correct event type, calls closure with argument.
    fn with_event(&self, event_trait_id: TypeId, f: |&Any|);
}

/// Asserts that an event is supported correctly and is that event.
///
/// Example: `assert_event_trait::<Event, Box<Render>>(&e);`
pub fn assert_event_trait<
    E: GenericEvent + PartialEq + Show,
    T: 'static
>(e: &E) {
    let name = unsafe { (*get_tydesc::<T>()).name };
    let id = TypeId::of::<T>();

    let mut tested_equal = false;
    e.with_event(id, |ev| {
        let new_e: E = GenericEvent::from_event(id, ev).expect(
            format!(
                "Could not construct event of event trait '{}' from '{}'",
                name, e
            ).as_slice()
        );
        assert_eq!(*e, new_e);
        tested_equal = true;
    });
    if !tested_equal {
        fail!("Expected event trait '{}', found '{}'", name, e);
    }
}

impl GenericEvent for InputEvent {
    #[inline(always)]
    fn from_event(event_trait_id: TypeId, args: &Any) -> Option<InputEvent> {
        let press = TypeId::of::<Box<PressEvent>>();
        let release = TypeId::of::<Box<ReleaseEvent>>();
        let mouse_cursor = TypeId::of::<Box<MouseCursorEvent>>();
        let mouse_relative = TypeId::of::<Box<MouseRelativeEvent>>();
        let mouse_scroll = TypeId::of::<Box<MouseScrollEvent>>();
        match event_trait_id {
            x if x == press => {
                match args.downcast_ref::<Button>() {
                    Some(&button) => Some(Press(button)),
                    _ => fail!("Expected `Button`")
                }
            }
            x if x == release => {
                match args.downcast_ref::<Button>() {
                    Some(&button) => Some(Release(button)),
                    _ => fail!("Expected `Button`")
                }
            }
            x if x == mouse_cursor => {
                match args.downcast_ref::<(f64, f64)>() {
                    Some(&(x, y)) => Some(Move(MouseCursor(x, y))),
                    _ => fail!("Expected `(f64, f64)`")
                }
            }
            x if x == mouse_relative => {
                match args.downcast_ref::<(f64, f64)>() {
                    Some(&(x, y)) => Some(Move(MouseRelative(x, y))),
                    _ => fail!("Expected `(f64, f64)`")
                }
            }
            x if x == mouse_scroll => {
                match args.downcast_ref::<(f64, f64)>() {
                    Some(&(x, y)) => Some(Move(MouseScroll(x, y))),
                    _ => fail!("Expected `(f64, f64)`")
                }
            }
            _ => None
        }
    }

    #[inline(always)]
    fn with_event(&self, event_trait_id: TypeId, f: |&Any|) {
        let press = TypeId::of::<Box<PressEvent>>();
        let release = TypeId::of::<Box<ReleaseEvent>>();
        let mouse_cursor = TypeId::of::<Box<MouseCursorEvent>>();
        let mouse_relative = TypeId::of::<Box<MouseRelativeEvent>>();
        let mouse_scroll = TypeId::of::<Box<MouseScrollEvent>>();
        match event_trait_id {
            x if x == press => {
                match *self {
                    Press(ref button) => f(button),
                    _ => {}
                }
            }
            x if x == release => {
                match *self {
                    Release(ref button) => f(button),
                    _ => {}
                }
            }
            x if x == mouse_cursor => {
                match *self {
                    Move(MouseCursor(x, y)) => f(&(x, y)),
                    _ => {}
                }
            }
            x if x == mouse_relative => {
                match *self {
                    Move(MouseRelative(x, y)) => f(&(x, y)),
                    _ => {}
                }
            }
            x if x == mouse_scroll => {
                match *self {
                    Move(MouseScroll(x, y)) => f(&(x, y)),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test_input_event() {
    use input;
    use input::Keyboard;

    let ref e = PressEvent::from_button(Keyboard(input::keyboard::A)).unwrap();
    assert_event_trait::<InputEvent, Box<PressEvent>>(e);

    let ref e = ReleaseEvent::from_button(Keyboard(input::keyboard::B)).unwrap();
    assert_event_trait::<InputEvent, Box<ReleaseEvent>>(e);

    let ref e = MouseCursorEvent::from_xy(1.0, 0.0).unwrap();
    assert_event_trait::<InputEvent, Box<MouseCursorEvent>>(e);

    let ref e = MouseRelativeEvent::from_xy(0.0, 1.0).unwrap();
    assert_event_trait::<InputEvent, Box<MouseRelativeEvent>>(e);

    let ref e = MouseScrollEvent::from_xy(-1.0, 0.0).unwrap();
    assert_event_trait::<InputEvent, Box<MouseScrollEvent>>(e);
}