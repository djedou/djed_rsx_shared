
use std::fmt::Debug;
use std::rc::Rc;

use super::key_traits::TGlyphStore;
use types::font_types::FontEncodedData;

pub trait TFontCache: Clone + 'static {
    type FontInstance;
    type FontId;
    type ResourceUpdates;
    type Glyphs: TGlyphStore;

    fn add_raw<P, T>(&mut self, p: P, t: T, value: usize) -> Option<()>
    where
        T: Into<Rc<Vec<u8>>>,
        P: AsRef<str>;

    fn add_font<P, E>(&mut self, p: P, e: &E, value: usize) -> Option<()>
    where
        P: AsRef<str>,
        E: TEncodedFont;

    fn add_font_with_id<E>(&mut self,font_id: Self::FontId,e: &E,value: usize) -> Option<()>
    where
        E: TEncodedFont;

    fn get_family_name<P>(&self, p: P) -> Option<String>
    where
        P: AsRef<str>;

    fn get_family_name_for_id(&self,font_id: Self::FontId) -> Option<String>;

    fn set_default_font<T>(&mut self, t: T, value1: u32,value2: u32)
    where
        T: AsRef<str>;

    fn get_default_font(&self) -> Option<Self::FontInstance>;

    fn get_default_font_with_size(&self,value: u32) -> Option<Self::FontInstance>;

    fn get_default_font_with_size_and_dpi(&self,value1: u32,value2: u32) -> Option<Self::FontInstance>;

    fn get_font<T>(&self, t: T) -> Option<Self::FontInstance>
    where
        T: AsRef<str>;

    fn get_font_with_size<T>(&self, t: T,value: u32) -> Option<Self::FontInstance>
    where
        T: AsRef<str>;

    fn get_font_with_size_and_dpi<T>(&self, t: T, value1: u32,value2: u32) -> Option<Self::FontInstance>
    where
        T: AsRef<str>;

    fn shape_text_h<T>(&self, font_instance: &Self::FontInstance, t: T) -> Option<Self::Glyphs>
    where
        T: AsRef<str>;

    fn shape_text_v<T>(&self, font_instance: &Self::FontInstance, t: T) -> Option<Self::Glyphs>
    where
        T: AsRef<str>;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}

pub trait TEncodedFont: Debug + PartialEq {
    type Error;

    fn from_bytes<T>(t: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Into<Rc<Vec<u8>>>;

    fn from_data_uri<T>(t: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Into<Rc<String>>;

    fn bytes(&self) -> Option<&Rc<Vec<u8>>>;

    fn data_uri(&self) -> Option<&Rc<String>>;

    fn info(&self) -> FontEncodedData;
}
