
use std::convert::TryInto;
use std::fmt::Debug;

use serde::{Deserialize as Des, Serialize as Ser};

use super::{
    dom_traits::TDOMText,
    resource_traits::TResourceGroup,
    style_traits::TStyleDeclarations
};
use types::dom_types::KnownElementName;

pub trait TClientRect: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {
    type Position: TClientPosition;
    type Size: TClientSize;

    fn position(&self) -> Self::Position;

    fn size(&self) -> Self::Size;

    fn offset_from_page(&self, page: (u32, u32)) -> (u32, u32);

    fn client_from_page(&self, page: (u32, u32)) -> (u32, u32);

    fn contains_point(&self, page: (u32, u32)) -> bool;
}

pub trait TClientPosition: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {}

pub trait TClientSize: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {}

pub trait TMeasuredImage: Debug + PartialEq + Ser + for<'a> Des<'a> {}

pub trait TShapedText: Debug + PartialEq + Ser + for<'a> Des<'a> {}

pub trait TLayoutNode: Debug + PartialEq {
    type Styles: TStyleDeclarations;
    type Resources: TResourceGroup;
    type TextMeasureMetadata;
    type ImageMeasureMetadata;
    type NormalMeasureMetadata;
    type ReflowDirection;
    type ClientPosition: TClientPosition + 'static;
    type BoundingClientRect: TClientRect + 'static;
    type MeasuredImage: TMeasuredImage + 'static;
    type ShapedText: TShapedText + 'static;

    fn make_initial_layout_node<T>(t: T) -> Self
    where
        T: TryInto<KnownElementName>;

    fn reset_custom_styles<T>(&mut self, style: T)
    where
        T: TryInto<KnownElementName>;

    fn is_tainted(&self) -> bool;

    fn insert_child(&mut self, child: &mut Self,value: usize);

    fn append_child(&mut self, child: &mut Self);

    fn remove_child(&mut self, child: &mut Self);

    fn apply_rules<'a, I>(&mut self, iter: I)
    where
        Self::Styles: 'a,
        I: Iterator<Item = &'a Self::Styles>
    {
        iter.for_each(|styles| self.apply_styles(styles));
    }

    fn apply_styles(&mut self, styles: &Self::Styles);

    fn mark_dirty(&mut self);

    fn measure_self_as_image<T>(&mut self, resources: &Self::Resources, t: &T, metadata: &Self::ImageMeasureMetadata)
    where
        T: TDOMText;

    fn measure_self_as_text<T>(&mut self, resources: &Self::Resources, t: &T, metadata: &Self::TextMeasureMetadata)
    where
        T: TDOMText;

    fn measure_self_as_normal(&mut self,resource: &Self::Resources,metadata: &Self::NormalMeasureMetadata);

    fn reflow_subtree(&mut self, value1: u32,value2: u32,flow_direction: Self::ReflowDirection);

    fn set_computed_client_position(&mut self,client: Self::ClientPosition);

    fn get_local_bounding_client_rect(&self) -> Self::BoundingClientRect;

    fn get_global_bounding_client_rect(&self) -> Self::BoundingClientRect;

    fn get_measured_image(&self) -> &Self::MeasuredImage;

    fn get_shaped_text(&self) -> &Self::ShapedText;
}
