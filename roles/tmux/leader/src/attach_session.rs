//! Attach-to-session picker (**A** on root). Session pills only; **Enter** / **1–9** / **Tab**
//! switches the client to the selected session. **Esc** returns to root.

use crate::keynode::KeyNode;

/// Placeholder layer (no letter keys); session pills drive the UI.
pub static NODES: &[KeyNode] = &[];
