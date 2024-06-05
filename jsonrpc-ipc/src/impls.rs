use std::fmt;

use super::*;

impl fmt::Debug for Ipc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ipc")
            .field("tx", &"Sender<(Request, Sender<Response>)>")
            .field("rx", &"Receiver<(Request, Sender<Response>)>")
            .finish()
    }
}

impl Default for Ipc {
    fn default() -> Self {
        Self::new()
    }
}
