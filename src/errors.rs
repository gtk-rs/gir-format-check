// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

pub struct Errors {
    pub messages: Vec<String>,
    pub nb_errors: usize,
}

impl Errors {
    pub fn has_errors(&self) -> bool {
        self.nb_errors > 0
    }

    pub fn to_string(&self) -> String {
        self.messages.join("\n")
    }
}
