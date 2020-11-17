
use std::convert::TryInto;

use super::unit_types::SharedUnit;

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedCursor;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedCursor {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedColor;

#[cfg(feature = "impl-dummy")]
impl Into<[u8; 4]> for DummyComputedColor {
    fn into(self) -> [u8; 4] {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedTextShadow;

#[cfg(feature = "impl-dummy")]
impl Into<[u8; 4]> for DummyComputedTextShadow {
    fn into(self) -> [u8; 4] {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontName;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedFontName {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontStyle;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedFontStyle {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontCaps;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedFontCaps {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontWeight;

#[cfg(feature = "impl-dummy")]
impl TryInto<u32> for DummyComputedFontWeight {
    type Error = ();

    fn try_into(self) -> Result<u32, Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontSize;

#[cfg(feature = "impl-dummy")]
impl TryInto<SharedUnit> for DummyComputedFontSize {
    type Error = ();

    fn try_into(self) -> Result<SharedUnit, Self::Error> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedFontStretch;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedFontStretch {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedVisibility;

#[cfg(feature = "impl-dummy")]
impl Into<bool> for DummyComputedVisibility {
    fn into(self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedBorderStyle;

#[cfg(feature = "impl-dummy")]
impl AsRef<str> for DummyComputedBorderStyle {
    fn as_ref(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone)]
pub struct DummyComputedBoxShadow;

#[cfg(feature = "impl-dummy")]
impl Into<[u8; 4]> for DummyComputedBoxShadow {
    fn into(self) -> [u8; 4] {
        unimplemented!()
    }
}
