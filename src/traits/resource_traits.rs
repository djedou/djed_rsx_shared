
use super::{
    file_traits::TFileCache,
    font_traits::TFontCache,
    image_traits::TImageCache
};
pub trait TResourceGroup {
    type Files: TFileCache;
    type Images: TImageCache;
    type Fonts: TFontCache;

    fn files(&self) -> Self::Files;

    fn images(&self) -> Self::Images;

    fn fonts(&self) -> Self::Fonts;
}
