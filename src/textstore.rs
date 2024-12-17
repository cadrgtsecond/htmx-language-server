use itertools::Itertools;
use lsp_types::Uri;
use self_cell::self_cell;
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Debug,
};
use string_cache::DefaultAtom;
use tl::{HTMLTag, Node, ParseError, Parser, ParserOptions, VDom};

self_cell! {
    pub struct FileData {
        owner: Box<str>,

        #[covariant]
        dependent: VDom,
    }

    impl {Debug}
}

pub struct TextStore(pub HashMap<Uri, FileData>);

impl TextStore {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, uri: Uri, content: &str) {
        self.0.insert(
            uri,
            FileData::new(Box::from(content), |content| {
                tl::parse(content, ParserOptions::default()).expect("HTML File too long")
            }),
        );
    }
}
