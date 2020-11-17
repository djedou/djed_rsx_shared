
use traits::util_traits::TAnyPartialEq;

impl<'a, 'b> PartialEq<dyn TAnyPartialEq + 'b> for dyn TAnyPartialEq + 'a {
    fn eq(&self, other: &(dyn TAnyPartialEq + 'b)) -> bool {
        TAnyPartialEq::eq(self, other)
    }
}
