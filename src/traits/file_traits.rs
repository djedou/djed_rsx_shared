
use std::path::Path;
pub trait TFileCache: Clone + 'static {
    type File;
    type ResourceUpdates;

    fn add_file<P>(&mut self, value: P) -> Option<()>
    where
        P: AsRef<Path>;

    fn get_file<P>(&self, value: P) -> Option<Self::File>
    where
        P: AsRef<Path>;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}
