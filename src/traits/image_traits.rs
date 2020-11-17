
use std::fmt::Debug;
use std::rc::Rc;

use super::key_traits::TDimensionsInfo;
use types::image_format_types::ImageEncodingFormat;
use types::image_types::ImageEncodedData;

pub trait TImageCache: Clone + 'static {
    type Image;
    type ImageId;
    type ResourceUpdates;
    type Dimensions: TDimensionsInfo;

    fn add_raw<P, T>(&mut self, p: P, t: T) -> Option<()>
    where
        T: Into<Rc<Vec<u8>>>,
        P: AsRef<str>;

    fn add_image<P, E>(&mut self, p: P, e: &E) -> Option<()>
    where
        P: AsRef<str>,
        E: TEncodedImage;

    fn add_image_with_id<E>(&mut self,image_id: Self::ImageId,e: &E) -> Option<()>
    where
        E: TEncodedImage;

    fn get_image<P>(&self, p: P) -> Option<Self::Image>
    where
        P: AsRef<str>;

    fn measure_image<P>(&self, p: P) -> Option<Self::Dimensions>
    where
        P: AsRef<str>;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}

pub trait TEncodedImage: Debug + PartialEq {
    type Error;

    fn from_bytes<T>(t: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Into<Rc<Vec<u8>>>;

    fn from_data_uri<T>(t: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Into<Rc<String>>;

    fn format(&self) -> Option<ImageEncodingFormat>;

    fn bytes(&self) -> Option<&Rc<Vec<u8>>>;

    fn data_uri(&self) -> Option<&Rc<String>>;

    fn size_info(&self) -> Option<(u32, u32)>;

    fn info(&self) -> ImageEncodedData;
}
