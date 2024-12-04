use std::fmt;

use cssparser::ToCss;
use derive_more::Debug;
use selectors::{
    parser::{self, SelectorImpl, SelectorParseErrorKind},
    Element,
};
use string_cache::DefaultAtom;

use crate::element::DOMTraverser;

#[derive(Debug, Clone)]
struct ElementImpl<'a>(DOMTraverser<'a>);

impl<'a> Element for ElementImpl<'a> {
    type Impl = Simple;

    fn opaque(&self) -> selectors::OpaqueElement {
        selectors::OpaqueElement::new(self)
    }

    fn parent_element(&self) -> Option<Self> {
        self.0.goto_parent().map(ElementImpl)
    }

    // TODO: Implement `facet` or declarative-shadow-dom
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
        self.0.goto_prev_sibling().map(ElementImpl)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.0.goto_next_sibling().map(ElementImpl)
    }

    fn first_element_child(&self) -> Option<Self> {
        self.0.goto_first_child().map(ElementImpl)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        true
    }

    fn has_local_name(&self, local_name: &<Self::Impl as SelectorImpl>::BorrowedLocalName) -> bool {
        *local_name.0 == *self.0.curr.tag_name
    }

    fn has_namespace(&self, _ns: &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl) -> bool {
        true
    }

    fn is_same_type(&self, other: &Self) -> bool {
        *self.0.curr.tag_name == *other.0.curr.tag_name
    }

    fn attr_matches(
        &self,
        _ns: &selectors::attr::NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>,
        local_name: &<Self::Impl as SelectorImpl>::LocalName,
        operation: &selectors::attr::AttrSelectorOperation<
            &<Self::Impl as SelectorImpl>::AttrValue,
        >,
    ) -> bool {
        let Some(attr) = self.0.curr.attrs.get(&DefaultAtom::from(local_name.0.as_str())) else {
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
        *self.0.curr.tag_name == *"a"
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
            self.0.curr.attrs.get(&DefaultAtom::from("id")).unwrap_or(&String::from("")).as_bytes(),
            id.0.as_bytes(),
        )
    }

    fn has_class(
        &self,
        name: &<Self::Impl as SelectorImpl>::Identifier,
        case_sensitivity: selectors::attr::CaseSensitivity,
    ) -> bool {
        let Some(classes) = self.0.curr.attrs.get(&DefaultAtom::from("class")) else {
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
        self.0.curr.children.is_empty()
    }

    fn is_root(&self) -> bool {
        *self.0.curr.tag_name == *"html"
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
