use std::{cell::RefCell, collections::HashMap, fmt};

use cssparser::ToCss;
use derive_more::Debug;
use selectors::{
    context::{
        IgnoreNthChildForInvalidation, MatchingContext, MatchingMode, NeedsSelectorFlags,
        QuirksMode,
    },
    parser::{self, ParseRelative, SelectorImpl, SelectorParseErrorKind},
    Element, NthIndexCache, OpaqueElement, SelectorList,
};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Node, Query, QueryCursor, TreeCursor};

use crate::search;

/// This struct wraps around a tree sitter [`Node`] in order to allow
/// using [`selectors`] to query it
#[derive(Debug, Clone)]
pub struct HTMLNode<'a> {
    pub tree: Node<'a>,
    #[debug("{:?}", &self.code[self.tree.start_byte()..self.tree.end_byte()])]
    pub code: &'a str,
    #[debug(skip)]
    pub cursor: TreeCursor<'a>,
    pub tag_name: &'a str,
    pub attrs: HashMap<&'a str, &'a str>,
}

impl<'a> HTMLNode<'a> {
    pub fn new(tree: Node<'a>, code: &'a str, cursor: TreeCursor<'a>) -> Self {
        assert_eq!(tree.kind(), "element");
        let source = "
          (start_tag
             (tag_name) @tag_name
             (attribute
                 (attribute_name) @attr_name
                 [(attribute_value) @attr_value (quoted_attribute_value (attribute_value) @attr_value)])*)";
        let query = Query::new(&tree.language(), source).unwrap();

        let mut query_cursor = QueryCursor::new();
        let mut matches = query_cursor.matches(&query, tree, code.as_bytes());
        let current = matches.next().unwrap();

        let c = &current.captures;
        let tag_name = c.iter().find(|c| c.index == 0).unwrap();

        let attrs = c
            .iter()
            .filter(|c| c.index == 1)
            .zip(c.iter().filter(|c| c.index == 2))
            .map(|(name, val)| {
                (
                    &code[name.node.start_byte()..name.node.end_byte()],
                    &code[val.node.start_byte()..val.node.end_byte()],
                )
            })
            .collect();

        Self {
            tree,
            code,
            cursor,
            tag_name: &code[tag_name.node.start_byte()..tag_name.node.end_byte()],
            attrs,
        }
    }

    pub fn matches(&self, selector: &SelectorList<Simple>, scope: Option<&Self>) -> bool {
        self.matches_with_cache(selector, scope, &mut NthIndexCache::default())
    }
    pub fn matches_with_cache(
        &self,
        selector: &SelectorList<Simple>,
        scope: Option<&Self>,
        cache: &mut NthIndexCache,
    ) -> bool {
        let mut context = MatchingContext::new(
            MatchingMode::Normal,
            None,
            cache,
            QuirksMode::NoQuirks,
            NeedsSelectorFlags::No,
            IgnoreNthChildForInvalidation::Yes,
        );
        context.scope_element = scope.map(OpaqueElement::new);
        selectors::matching::matches_selector_list(selector, self, &mut context)
    }

    /// Selects all elements matching `selector` in the tree
    pub fn select(&self, selector: &str) -> Result<impl Iterator<Item = HTMLNode<'_>>, String> {
        let mut parserinput = cssparser::ParserInput::new(selector);
        let mut parser = cssparser::Parser::new(&mut parserinput);
        let selectorlist = SelectorList::parse(&MyParser, &mut parser, ParseRelative::ForNesting)
            .map_err(|err| format!("Failed to parse selector: {err:?}"))?;
        let search = search::Matches::new(self.cursor.clone(), move |node| {
            node.kind() == "element"
                && HTMLNode::new(node, self.code, self.cursor.clone())
                    .matches(&selectorlist, Some(self))
        });

        Ok(search.map(|n| HTMLNode::new(n, self.code, self.cursor.clone())))
    }
}

impl<'a> Element for HTMLNode<'a> {
    type Impl = Simple;

    fn opaque(&self) -> selectors::OpaqueElement {
        selectors::OpaqueElement::new(self)
    }

    fn parent_element(&self) -> Option<Self> {
        let mut cursor = self.cursor.clone();
        if cursor.goto_parent() {
            let parent = cursor.node();
            if parent.kind() == "element" {
                Some(HTMLNode::new(parent, self.code, cursor.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        let mut cursor = self.cursor.clone();
        loop {
            if !cursor.goto_previous_sibling() {
                return None;
            }
            if cursor.node().kind() == "element" {
                return Some(HTMLNode::new(cursor.node(), self.code, self.cursor.clone()));
            }
        }
    }

    fn next_sibling_element(&self) -> Option<Self> {
        let mut cursor = self.cursor.clone();
        while cursor.goto_next_sibling() {
            if cursor.node().kind() == "element" {
                return Some(HTMLNode::new(cursor.node(), self.code, cursor.clone()));
            }
        }
        None
    }

    fn first_element_child(&self) -> Option<Self> {
        let mut cursor = self.cursor.clone();
        if !cursor.goto_first_child() {
            return None;
        }
        while cursor.goto_next_sibling() {
            if cursor.node().kind() == "element" {
                return Some(HTMLNode::new(cursor.node(), self.code, cursor.clone()));
            }
        }
        None
    }

    fn is_html_element_in_html_document(&self) -> bool {
        true
    }

    fn has_local_name(&self, local_name: &<Self::Impl as SelectorImpl>::BorrowedLocalName) -> bool {
        local_name.0 == self.tag_name
    }

    fn has_namespace(&self, _ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl) -> bool {
        true
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.tag_name == other.tag_name
    }

    fn attr_matches(
        &self,
        _ns: &selectors::attr::NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
        operation: &selectors::attr::AttrSelectorOperation<
            &<Self::Impl as SelectorImpl>::AttrValue,
        >,
    ) -> bool {
        let Some(attr) = self.attrs.get(local_name.0.as_str()) else {
            return false;
        };
        operation.eval_str(attr)
    }

    fn match_non_ts_pseudo_class(
        &self,
        _pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        _context: &mut selectors::context::MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    fn match_pseudo_element(
        &self,
        _pe: &<Self::Impl as SelectorImpl>::PseudoElement,
        _context: &mut selectors::context::MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    fn apply_selector_flags(&self, _flags: selectors::matching::ElementSelectorFlags) {}

    fn is_link(&self) -> bool {
        self.tag_name == "link"
    }

    fn is_html_slot_element(&self) -> bool {
        true
    }

    fn has_id(
        &self,
        id: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: selectors::attr::CaseSensitivity,
    ) -> bool {
        case_sensitivity.eq(
            self.attrs.get("id").unwrap_or(&"").as_bytes(),
            id.0.as_bytes(),
        )
    }

    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: selectors::attr::CaseSensitivity,
    ) -> bool {
        let Some(classes) = self.attrs.get("class") else {
            return false;
        };
        classes
            .split(char::is_whitespace)
            .any(|c| case_sensitivity.eq(c.as_bytes(), name.0.as_bytes()))
    }

    fn imported_part(
        &self,
        _name: &<Self::Impl as SelectorImpl>::Identifier,
    ) -> Option<<Self::Impl as SelectorImpl>::Identifier> {
        None
    }

    fn is_part(&self, _name: &<Self::Impl as SelectorImpl>::Identifier) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        self.tree.child(2).is_none()
    }

    fn is_root(&self) -> bool {
        self.tree.parent().map(|p| p.kind()) == Some("document")
    }
}

#[derive(Debug, Clone)]
pub struct Simple;

impl SelectorImpl for Simple {
    type ExtraMatchingData<'a> = ();

    type AttrValue = CssString;

    type Identifier = CssString;

    type LocalName = CssLocalName;

    type NamespaceUrl = String;

    type NamespacePrefix = CssLocalName;

    type BorrowedNamespaceUrl = String;

    type BorrowedLocalName = CssLocalName;

    type NonTSPseudoClass = NonTSPseudoClass;

    type PseudoElement = PseudoElement;
}

#[derive(Clone, PartialEq, Eq)]
pub struct CssString(pub String);

impl<'a> From<&'a str> for CssString {
    fn from(val: &'a str) -> Self {
        Self(val.to_owned())
    }
}

impl AsRef<str> for CssString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToCss for CssString {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        write!(dest, "{}", self.0)
    }
}
/// Non Tree-Structural Pseudo-Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonTSPseudoClass {}

impl parser::NonTSPseudoClass for NonTSPseudoClass {
    type Impl = Simple;

    fn is_active_or_hover(&self) -> bool {
        false
    }

    fn is_user_action_state(&self) -> bool {
        false
    }
}

impl ToCss for NonTSPseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str("")
    }
}

/// CSS Pseudo-Element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PseudoElement {}

impl parser::PseudoElement for PseudoElement {
    type Impl = Simple;
}

impl ToCss for PseudoElement {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str("")
    }
}

/// Wraps [`LocalName`] so that it can be used with [`selectors`]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CssLocalName(pub String);

impl<'a> From<&'a str> for CssLocalName {
    fn from(val: &'a str) -> Self {
        Self(val.into())
    }
}

impl ToCss for CssLocalName {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

struct MyParser;

impl<'a> selectors::Parser<'a> for MyParser {
    type Impl = Simple;
    type Error = SelectorParseErrorKind<'a>;
}
