
use std::any::Any;

pub trait TAnyPartialEq {
    fn as_any(&self) -> &dyn Any;

    fn eq(&self, peq: &dyn TAnyPartialEq) -> bool;
}
