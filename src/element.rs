use std::{collections::HashMap, sync::Arc};

use string_cache::DefaultAtom;
use tree_sitter::Tree;

/// Basically a DOM. Can be queries with selectors
/// We are using `string_cache` so we can avoid lifetimes while remaining efficient
#[derive(Debug, Clone)]
pub struct HTMLElement {
    pub tag_name: DefaultAtom,
    pub id: Option<DefaultAtom>,
    pub classes: Vec<DefaultAtom>,
    pub attrs: HashMap<DefaultAtom, String>,
    pub children: Vec<HTMLElement>,
}

impl HTMLElement {
    pub fn from_code() {}

    pub fn new(tree: Tree) -> Self {
        Self { tag_name: todo!(), id: todo!(), classes: todo!(), attrs: todo!(), children: todo!() }
    }
}

#[derive(Debug, Clone)]
pub struct DOMTraverser<'a> {
    parent: Option<Arc<DOMTraverser<'a>>>,
    my_index: usize,
    pub curr: &'a HTMLElement,
}

impl<'a> DOMTraverser<'a> {
    pub fn new(root: &'a HTMLElement) -> Self {
        DOMTraverser {
            parent: None,
            my_index: 0,
            curr: root,
        }
    }

    pub fn goto_first_child(&self) -> Option<Self> {
        Some(DOMTraverser {
            parent: Some(Arc::new(self.clone())),
            my_index: 0,
            curr: self.curr.children.get(0)?,
        })
    }
    pub fn goto_next_sibling(&self) -> Option<Self> {
        let parent = self.parent.as_ref()?;
        Some(DOMTraverser {
            parent: Some(parent.clone()),
            my_index: self.my_index + 1,
            curr: parent.curr.children.get(self.my_index + 1)?,
        })
    }
    pub fn goto_prev_sibling(&self) -> Option<Self> {
        let parent = self.parent.as_ref()?;
        Some(DOMTraverser {
            parent: Some(parent.clone()),
            my_index: self.my_index - 1,
            curr: parent.curr.children.get(self.my_index - 1)?,
        })
    }

    pub fn goto_parent(&self) -> Option<Self> {
        self.parent.clone().map(Arc::unwrap_or_clone)
    }
}
