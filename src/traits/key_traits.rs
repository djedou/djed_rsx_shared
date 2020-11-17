
use std::fmt::Debug;
use std::hash::Hash;

use serde::{Deserialize as Des, Serialize as Ser};

use types::font_types::{FontEncodedData, FontInstanceResourceData, FontResourceData};
use types::image_types::{ImageEncodedData, ImageResourceData};

pub trait TMediaKey: Debug + Ord + Hash + Copy + Ser + for<'a> Des<'a> {}

pub trait TFontKey: Debug + Ord + Hash + Copy + Ser + for<'a> Des<'a> {}

pub trait TFontInstanceKey: Debug + Ord + Hash + Copy + Ser + for<'a> Des<'a> {}

pub trait TImageKeysAPI {
    type RootRendererAPI;
    type ResourceUpdates;
    type ImageKey: TMediaKey;

    fn new(root: Self::RootRendererAPI) -> Self;

    fn add_image(&mut self, encode_data: ImageEncodedData,resource_data: ImageResourceData) -> Self::ImageKey;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}

pub trait TFontKeysAPI {
    type RootRendererAPI;
    type ResourceUpdates;
    type FontKey: TFontKey;
    type FontInstanceKey: TFontInstanceKey;
    type GlyphInstance: TGlyphInstance;

    fn new(root: Self::RootRendererAPI) -> Self;

    fn add_font(&mut self,encode_data: FontEncodedData,resource_data: FontResourceData) -> Self::FontKey;

    fn add_font_instance(&mut self, font_key: Self::FontKey, resource_data: FontInstanceResourceData) -> Self::FontInstanceKey;

    fn take_resource_updates(&mut self) -> Self::ResourceUpdates;
}

pub trait TDimensionsInfo: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {
    type ResourceKey: TMediaKey;

    fn resource_key(&self) -> Self::ResourceKey;

    fn width(&self) -> u32;

    fn height(&self) -> u32;
}

pub trait TGlyphInstance: Debug + PartialEq + Copy + Ser + for<'a> Des<'a> {
    fn new(value1: u32, value2:  i32, value3:  i32) -> Self;
}

pub trait TGlyphStore: Debug + PartialEq + Clone + Ser + for<'a> Des<'a> {
    type FontKey: TFontKey;
    type FontInstanceKey: TFontInstanceKey;
    type Glyph: TGlyphInstance;

    fn font_key(&self) -> Self::FontKey;

    fn font_instance_key(&self) -> Self::FontInstanceKey;

    fn width_f(&self) -> f32;

    fn height_f(&self) -> f32;

    fn width_64(&self) -> i32;

    fn height_64(&self) -> i32;

    fn glyphs(&self) -> &[Self::Glyph];
}
