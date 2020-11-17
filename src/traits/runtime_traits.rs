
use super::{
    dom_traits::TDOMTree,
    resource_traits::TResourceGroup
};
pub trait TRuntime {
    type RootRendererAPI;
    type DOMResources: TResourceGroup;
    type DOMTree: TDOMTree;
    type VirtualEventMetadata;
    type ReflowMetadata;
    type BuiltDisplayList;
    type ResourceUpdates;

    fn new<S, R>(render: &Self::RootRendererAPI, s: S, r: R) -> Self
    where
        S: Fn(&mut Self::DOMResources),
        R: Fn() -> Self::DOMTree;

    fn should_set_window_position(&mut self) -> Option<(i32, i32)>;

    fn should_set_window_size(&mut self) -> Option<(u32, u32)>;

    fn should_redraw(&mut self) -> bool;

    fn handle_event(&mut self, metadata: Self::VirtualEventMetadata) -> bool;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;

    fn generate_display_list(&mut self,metadata: Self::ReflowMetadata) -> Self::BuiltDisplayList;
}

pub trait TRunner {
    type Runtime: TRuntime;

    fn run<F>(f: F)
    where
        F: FnMut(&<Self::Runtime as TRuntime>::RootRendererAPI) -> Self::Runtime;
}
