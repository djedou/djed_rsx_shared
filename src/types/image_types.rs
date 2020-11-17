
use std::rc::Rc;
use std::sync::Arc;

use types::image_format_types::{ImageEncodingFormat, ImagePixelFormat};

#[derive(Debug, PartialEq)]
pub enum ImageEncodedData<'a> {
    Bytes {
        format: ImageEncodingFormat,
        bytes: &'a Rc<Vec<u8>>
    },
    DataUri {
        data_uri: &'a Rc<String>
    }
}

impl<'a> ImageEncodedData<'a> {
    pub fn new(format: ImageEncodingFormat, bytes: &'a Rc<Vec<u8>>) -> Self {
        ImageEncodedData::Bytes { format, bytes }
    }
}

#[derive(Debug, PartialEq)]
pub struct ImageResourceData<'a> {
    pub format: ImagePixelFormat,
    pub size: (u32, u32),
    pub pixels: &'a Arc<Vec<u8>>
}

impl<'a> ImageResourceData<'a> {
    pub fn new(format: ImagePixelFormat, size: (u32, u32), pixels: &'a Arc<Vec<u8>>) -> Self {
        ImageResourceData {
            format,
            size,
            pixels
        }
    }
}
