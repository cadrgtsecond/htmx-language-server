use log::warn;
use lsp_types::Uri;
use std::collections::HashMap;
use tl::{HTMLTag, ParserOptions, VDom};

#[derive(Debug)]
pub struct FileData {
    pub data: Box<str>,
    // We need to cheat a little to get self referential structs to work
    pub dom: VDom<'static>,
    /// Stores offset to each line in the file
    pub lines: Vec<usize>,
}

/// A large amount of magic depends on this function
///
/// It accepts two strings and effectively calculates `str2 - str1`, treating `str2` and `str1` as pointers
/// This, combined with the fact that `tl` uses the original string slice for all strings in the parse
/// tree means we can find the offset of any string in the parse tree, be it an attribute, tag name, etc
/// by just using this function
///
/// # Panics
/// Caller must guarantee str2 >= str1
pub fn str_ptr_offset(str1: &str, str2: &str) -> usize {
    unsafe {
        str2.as_ptr()
            .offset_from(str1.as_ptr())
            .try_into()
            .expect("str2 < str1")
    }
}

#[derive(Debug)]
pub enum HTMLObject<'a> {
    Tag(&'a str),
    Attr(&'a str),
    AttrValue(&'a str),
}

impl FileData {
    fn new(content: Box<str>) -> Result<Self, tl::ParseError> {
        let inner: &str = &content;
        let dom = tl::parse(
            unsafe { std::mem::transmute::<&str, &'static str>(inner) },
            ParserOptions::new().track_ids().track_classes(),
        )?;
        let lines = inner.lines().map(|l| str_ptr_offset(inner, l)).collect();
        Ok(Self {
            data: content,
            dom,
            lines,
        })
    }

    pub fn line_to_offset(&self, line: usize, chr: usize) -> usize {
        self.lines[line] + chr
    }

    /// Returns the particular object(tag, attribute, or attribute value) under the cursor
    ///
    /// Returns `None` when `&self` has no tags or when the tag names are not proper utf8
    pub fn object_under_cursor(&self, off: usize) -> Option<HTMLObject> {
        // Find node under `off`
        let curr_node = self.dom.nodes().binary_search_by_key(&off, |n| {
            let bytes = match n {
                tl::Node::Tag(htmltag) => htmltag.raw(),
                tl::Node::Raw(bytes) | tl::Node::Comment(bytes) => bytes,
            };
            str_ptr_offset(&self.data, bytes.try_as_utf8_str().unwrap())
        });
        warn!("Found {:?}", curr_node);
        let i = match curr_node {
            Ok(i) => i,
            Err(i) => i - 1,
        };
        if let tl::Node::Tag(tag) = &self.dom.nodes()[i] {
            Some(HTMLObject::Tag(tag.name().try_as_utf8_str()?))
        } else {
            None
        }
    }
}

pub struct TextStore(pub HashMap<Uri, FileData>);

impl TextStore {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, uri: Uri, content: &str) {
        let content = Box::from(content);
        self.0
            .insert(uri, FileData::new(content).expect("Failed to parse file"));
    }
}
