
use std::fmt::Debug;

use super::dom_traits::{TDOMNode, TDOMTree};

use types::dom_types::{DOMNodeRawId, EventType};
use types::event_types::{Closure, VirtualEventType};

use serde::{Deserialize as Des, Serialize as Ser};

pub trait TCallback<A>: Debug + PartialEq + Clone {
    type InvokeOk;
    type InvokeErr;

    fn call(&self, value: A) -> Result<Self::InvokeOk, Self::InvokeErr>;

    fn call_mut(&mut self, value: A) -> Result<Self::InvokeOk, Self::InvokeErr>;

    fn call_once(self, value: A) -> Result<Self::InvokeOk, Self::InvokeErr>;
}

// See https://developer.mozilla.org/en-US/docs/Web/API/Event
pub trait TEvent: Debug + PartialEq + Ser + for<'a> Des<'a> {
    fn target(&self) -> DOMNodeRawId;

    fn ty(&self) -> EventType;
}

pub trait TUIEvent: TEvent {
    fn alt_key(&self) -> bool;

    fn ctrl_key(&self) -> bool;

    fn meta_key(&self) -> bool;

    fn shift_key(&self) -> bool;
}

// See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent
pub trait TKeyboardEvent: TUIEvent {
    type KeyCode;

    fn code(&self) -> Self::KeyCode;

    fn key(&self) -> &'static str;

    fn get_modifier_state(&self) -> bool;

    fn repeat(&self) -> bool;
}

// See https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent
pub trait TMouseEvent: TUIEvent {
    type MouseButton;

    fn button(&self) -> Self::MouseButton;

    fn client_x(&self) -> u32;

    fn client_y(&self) -> u32;

    fn offset_x(&self) -> u32;

    fn offset_y(&self) -> u32;

    fn page_x(&self) -> u32;

    fn page_y(&self) -> u32;
}

pub trait TGenericEvent: TKeyboardEvent + TMouseEvent {}

pub trait TEventManager: Debug + PartialEq + Default {
    type Target: TDOMNode;
    type KeyCode;
    type MouseButton;
    type KeyEventData;
    type MouseEventData;

    fn add_event_listener<F>(&mut self, id: <Self::Target as TDOMNode>::Id, event_type: EventType, f: F)
    where
        F: Into<Closure<<Self::Target as TDOMNode>::Event>>;

    fn remove_event_listener<F>(&mut self, id: <Self::Target as TDOMNode>::Id, event_type: EventType, f: F)
    where
        F: Into<Closure<<Self::Target as TDOMNode>::Event>>;

    fn receive_key_event(&mut self, virtual_event_type: VirtualEventType, event_data: Self::KeyEventData);

    fn receive_mouse_event(&mut self, virtual_event_type: VirtualEventType, mouse_data: Self::MouseEventData);

    fn broadcast_events<A>(&mut self, vaule: &A)
    where
        A: TDOMTree<Node = Self::Target>;

    fn intercept_events<A, F>(&mut self, value: &A, f: F)
    where
        A: TDOMTree<Node = Self::Target>,
        F: FnMut(<Self::Target as TDOMNode>::Event);
}
