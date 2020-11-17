
use std::fmt::Debug;

use serde::{Deserialize as Des, Serialize as Ser};

use super::{
    style_traits::{TComputedStyles, TStyleDeclarations},
    event_traits::TGenericEvent,
    layout_traits::TLayoutNode,
    memory_traits::{TMemoryAPI, TMemoryAddrAPI},
};

use types::dom_types::KnownElementName;

pub trait TDOMText: Debug + Ord + Clone + AsRef<str> + Ser + for<'a> Des<'a> {}

pub trait TDOMTree: Debug + PartialEq {
    type Node: TDOMNode;

    fn get_node(&self, node_id: <Self::Node as TDOMNode>::Id) -> &Self::Node;

    fn get_node_mut(&mut self, node_id: <Self::Node as TDOMNode>::Id) -> &mut Self::Node;

    fn get_node_mut_pair(&mut self, node_id: (<Self::Node as TDOMNode>::Id, <Self::Node as TDOMNode>::Id)) -> (&mut Self::Node, &mut Self::Node);
}

pub trait TDOMNode: Debug + PartialEq {
    type Id: Debug + PartialEq;
    type Data: Debug + PartialEq;
    type Event: TGenericEvent;
    type Styles: TStyleDeclarations;
    type ComputedStyles: TComputedStyles;
    type LayoutNode: TLayoutNode;

    fn data(&self) -> &Self::Data;

    fn is_void(&self) -> bool;

    fn is_shadow_host(&self) -> bool;

    fn is_text(&self) -> bool;

    fn is_normal(&self) -> bool;

    fn is_known(&self, name: KnownElementName) -> bool;

    fn computed_styles(&self) -> &Self::ComputedStyles;

    fn layout_node(&self) -> &Self::LayoutNode;

    fn reflow_subtree(&mut self, flow: u32, subtree: u32, reflow_direction: <Self::LayoutNode as TLayoutNode>::ReflowDirection);

    fn set_computed_client_position(&mut self, client_position: <Self::LayoutNode as TLayoutNode>::ClientPosition);

    fn get_local_bounding_client_rect(&self) -> <Self::LayoutNode as TLayoutNode>::BoundingClientRect;

    fn get_global_bounding_client_rect(&self) -> <Self::LayoutNode as TLayoutNode>::BoundingClientRect;

    fn get_measured_image(&self) -> &<Self::LayoutNode as TLayoutNode>::MeasuredImage;

    fn get_shaped_text(&self) -> &<Self::LayoutNode as TLayoutNode>::ShapedText;
}

impl<T> TMemoryAPI for T
where
    T: TDOMNode
{
}

impl<T> TMemoryAddrAPI for T
where
    T: TDOMNode
{
}
