//! Functions to search a a tree sitter `Tree`

use tree_sitter::{Node, TreeCursor};

/// Searches for the deepest matching node within a tree, as given by its `TreeCursor`
///
/// First checks for `Pre`, then checks if any of the children match, then checks for `Post`
/// only if no children were found to match
/// Returns the node only if both `Pre` and `Post` match
/// The `cursor` will end up pointing to the matching node
pub fn for_deepest_matching<'a>(
    cursor: &mut TreeCursor<'a>,
    pre: &mut impl FnMut(Node<'a>) -> bool,
    post: &mut impl FnMut(Node<'a>) -> bool,
) -> Option<Node<'a>> {
    if !pre(cursor.node()) {
        return None;
    }
    if cursor.goto_first_child() {
        loop {
            if let Some(child) = for_deepest_matching(cursor, pre, post) {
                return Some(child);
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
    if post(cursor.node()) {
        Some(cursor.node())
    } else {
        None
    }
}

/// Searches for all matching `F` nodes within a tree, as given by its `TreeCursor`
pub struct Matches<'a, F>
where
    F: FnMut(Node<'a>) -> bool,
{
    cursor: TreeCursor<'a>,
    condition: F,
}

impl<'a, F> Matches<'a, F>
where
    F: FnMut(Node<'a>) -> bool,
{
    pub fn new(cursor: TreeCursor<'a>, condition: F) -> Self {
        Self { cursor, condition }
    }
}

impl<'a, F> Iterator for Matches<'a, F>
where
    F: FnMut(Node<'a>) -> bool,
{
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut seen = false;
        loop {
            if !seen {
                if (self.condition)(self.cursor.node()) {
                    // If successfull, then we have a new node to search
                    seen = self.cursor.goto_first_child();
                }
            } else if self.cursor.goto_next_sibling() {
                seen = false;
            } else if self.cursor.goto_parent() {
                seen = true;
            } else {
                return None;
            }
        }
    }
}
