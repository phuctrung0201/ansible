//! Move-window target picker (**w** → **m**). Separate view like [`crate::launcher`]: session pills
//! only, **Tab** / **Shift+Tab** / **Enter** / **1–9**; **Esc** returns to the windows group.

use crate::keynode::KeyNode;

/// Placeholder layer (no letter keys); session pills drive the UI.
pub static NODES: &[KeyNode] = &[];
