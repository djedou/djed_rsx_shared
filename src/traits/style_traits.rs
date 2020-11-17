
use std::convert::TryInto;
use std::fmt::Debug;

use serde::{Deserialize as Des, Serialize as Ser};

use types::{
    dom_types::KnownElementName,
    unit_types::SharedUnit
};
pub use num_traits::cast::FromPrimitive;

pub trait TStyleDeclarations: Debug + PartialEq + Clone {
    type LayoutStyle: Ser + for<'a> Des<'a>;
    type ThemeStyle: Ser + for<'a> Des<'a>;

    fn make_user_agent_styles<T>(t: T) -> Self
    where
        T: TryInto<KnownElementName>;

    fn for_each_layout_style<F>(&self, f: F)
    where
        F: FnMut(&Self::LayoutStyle);

    fn for_each_theme_style<F>(&self, f: F)
    where
        F: FnMut(&Self::ThemeStyle);
}

pub trait TInheritedStyles: Debug + PartialEq + Clone {
    type Styles: TStyleDeclarations;

    type Cursor: AsRef<str>;
    type Color: Into<[u8; 4]>;
    type TextShadow: Into<[u8; 4]>;
    type FontName: AsRef<str>;
    type FontStyle: AsRef<str>;
    type FontCaps: AsRef<str>;
    type FontWeight: TryInto<u32>;
    type FontSize: TryInto<SharedUnit>;
    type FontStretch: AsRef<str>;
    type Visibility: Into<bool>;

    fn inherit_styles(&mut self, style: &Self);

    fn cursor(&self) -> Self::Cursor;

    fn color(&self) -> Self::Color;

    fn font_style(&self) -> Self::FontStyle;

    fn text_shadows_copy(&self) -> Vec<Self::TextShadow>;

    fn font_names_copy(&self) -> Vec<Self::FontName>;

    fn font_caps(&self) -> Self::FontCaps;

    fn font_weight(&self) -> Self::FontWeight;

    fn font_size(&self) -> Self::FontSize;

    fn font_stretch(&self) -> Self::FontStretch;

    fn visibility(&self) -> Self::Visibility;

    fn find_font<F, O>(&self, f: F) -> Option<O>
    where
        F: FnMut(&Self::FontName) -> Option<O>;
}

pub trait TComputedStyles: TInheritedStyles {
    type BackgroundColor: Into<[u8; 4]>;
    type Opacity: Into<u32>;
    type BorderSize: Into<u32>;
    type BorderColor: Into<[u8; 4]>;
    type BorderStyle: AsRef<str>;
    type BoxShadow: Into<[u8; 4]>;

    fn make_initial_computed_styles<T>(t: T) -> Self
    where
        T: TryInto<KnownElementName>;

    fn reset_custom_styles<T>(&mut self, t: T)
    where
        T: TryInto<KnownElementName>;

    fn apply_rules<'a, I>(&mut self, iter: I)
    where
        Self::Styles: 'a,
        I: Iterator<Item = &'a Self::Styles>
    {
        iter.for_each(|styles| self.apply_styles(styles))
    }

    fn apply_styles(&mut self, style: &Self::Styles);

    fn background_color(&self) -> Self::BackgroundColor;

    fn opacity(&self) -> Self::Opacity;

    fn border_bottom_width(&self) -> Self::BorderSize;

    fn border_bottom_color(&self) -> Self::BorderColor;

    fn border_bottom_style(&self) -> Self::BorderStyle;

    fn border_left_width(&self) -> Self::BorderSize;

    fn border_left_color(&self) -> Self::BorderColor;

    fn border_left_style(&self) -> Self::BorderStyle;

    fn border_right_width(&self) -> Self::BorderSize;

    fn border_right_color(&self) -> Self::BorderColor;

    fn border_right_style(&self) -> Self::BorderStyle;

    fn border_top_width(&self) -> Self::BorderSize;

    fn border_top_color(&self) -> Self::BorderColor;

    fn border_top_style(&self) -> Self::BorderStyle;

    fn box_shadows_copy(&self) -> Vec<Self::BoxShadow>;
}
