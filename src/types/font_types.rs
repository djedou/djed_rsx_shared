
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum FontEncodedData<'a> {
    Bytes { bytes: &'a Rc<Vec<u8>> },
    DataUri { data_uri: &'a Rc<String> }
}

impl<'a> FontEncodedData<'a> {
    pub fn new(bytes: &'a Rc<Vec<u8>>) -> Self {
        FontEncodedData::Bytes { bytes }
    }
}

#[derive(Debug, PartialEq)]
pub struct FontResourceData<'a> {
    pub bytes: &'a Rc<Vec<u8>>,
    pub face_index: usize
}

impl<'a> FontResourceData<'a> {
    pub fn new(bytes: &'a Rc<Vec<u8>>, face_index: usize) -> Self {
        FontResourceData { bytes, face_index }
    }
}

#[derive(Debug, PartialEq)]
pub struct FontInstanceResourceData {
    pub size: u32,
    pub dpi: u32
}

impl FontInstanceResourceData {
    pub fn new(size: u32, dpi: u32) -> Self {
        FontInstanceResourceData { size, dpi }
    }
}
