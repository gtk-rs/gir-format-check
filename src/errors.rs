// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use std::fmt;

pub struct Errors {
    pub messages: Vec<String>,
    pub nb_errors: usize,
}

impl Errors {
    pub fn has_errors(&self) -> bool {
        self.nb_errors > 0
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.messages.join("\n"))
    }
}
