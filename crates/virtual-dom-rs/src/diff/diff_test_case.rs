//! Kept in it's own file to more easily import into the Percy book.

use crate::diff::diff;
use crate::patch::Patch;
use virtual_node::{AsRefVirtualNode, VirtualNode};

/// Test that we generate the right Vec<Patch> for some start and end virtual dom.
pub struct DiffTestCase<'a> {
    // ex: "Patching root level nodes works"
    pub description: &'static str,
    // ex: html! { <div> </div> }
    pub old: Box<AsRef<VirtualNode>>,
    // ex: html! { <strong> </strong> }
    pub new: Box<AsRef<VirtualNode>>,
    // ex: vec![Patch::Replace(0, &html! { <strong></strong> })],
    pub expected: Vec<Patch<'a>>,
}

impl<'a> DiffTestCase<'a> {
    pub fn test(&self) {
        // ex: vec![Patch::Replace(0, &html! { <strong></strong> })],
        let patches = diff(AsRefVirtualNode(&self.old), AsRefVirtualNode(&self.new));

        assert_eq!(patches, self.expected, "{}", self.description);
    }
}
