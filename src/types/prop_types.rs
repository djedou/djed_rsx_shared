
use std::any::Any;
use std::ops::Deref;
use std::rc::Rc;

use traits::prop_traits::TProp;

#[derive(Debug, Clone)]
pub struct Prop(Rc<dyn Any>);

impl TProp for Prop {}

impl Prop {
    pub fn new<T>(value: T) -> Self
    where
        T: Any
    {
        Prop(Rc::new(value))
    }
}

impl PartialEq for Prop {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Deref for Prop {
    type Target = dyn Any;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
