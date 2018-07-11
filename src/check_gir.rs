// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use utils;
use errors::Errors;

use std::cmp::Ordering;
use std::path::Path;

#[derive(Debug)]
struct Elem<'a> {
    name: &'a str,
    lower: String,
    pos: usize,
}

impl<'a> Elem<'a> {
    fn new(name: &str, pos: usize) -> Elem {
        Elem {
            name,
            lower: name.to_lowercase(),
            pos,
        }
    }
}

impl<'a> Eq for Elem<'a> {}
impl<'a> PartialEq for Elem<'a> {
    fn eq(&self, other: &Elem) -> bool {
        self.lower == other.lower
    }
}

impl<'a> PartialOrd for Elem<'a> {
    fn partial_cmp(&self, other: &Elem) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Elem<'a> {
    fn cmp(&self, other: &Elem) -> Ordering {
        if self.lower.ends_with("*") { // We always it to be first "because"!
            Ordering::Less
        } else {
            self.lower.cmp(&other.lower)
        }
    }
}

pub fn check_gir_content(content: &str) -> Errors {
    let lines = content.lines().collect::<Vec<_>>();
    let mut elems: Vec<Elem> = Vec::with_capacity(10);
    let mut objects = Vec::with_capacity(10);
    let mut in_list = None;
    let mut in_object = false;
    let mut errors = 0;
    let mut messages = Vec::with_capacity(10);

    for pos in 0..lines.len() {
        if lines[pos].ends_with("[") {
            in_list = Some(pos);
            continue
        } else if in_list.is_some() && lines[pos].trim() == "]" {
            if !elems.is_empty() {
                let mut local_errors = 0;
                println!("{:?}", elems);
                for it in 0..elems.len() - 1 {
                    if elems[it] > elems[it + 1] {
                        messages.push(format!("ERROR: \"{}\" should be after \"{}\"",
                                              elems[it].name,
                                              elems[it + 1].name));
                        local_errors += 1;
                    }
                }
                if local_errors > 0 {
                    elems.sort();
                    messages.push(format!("\n== Expected output ==\n{}\n{}]",
                                          lines[in_list.unwrap()],
                                          elems.iter()
                                               .map(|l| format!("{}\n", lines[l.pos]))
                                               .collect::<String>()));
                    errors += local_errors;
                }
            }
            elems.clear();
            in_list = None;
        } else if in_list.is_some() {
            let trimmed = lines[pos].trim();
            let mut len = trimmed.len();
            if trimmed.ends_with("\",") {
                len -= 2;
            } else if trimmed.ends_with(",") {
                len -= 1;
            } else if trimmed.ends_with("\"") {
               len -= 1;
            }
            let start = if trimmed.starts_with("\"") { 1 } else { 0 };
            elems.push(Elem::new(&trimmed[start..len], pos));
        } else if lines[pos] == "[[object]]" {
            in_object = true;
        } else if in_object == true && lines[pos].starts_with("name = \"") {
            let trimmed = lines[pos].trim();
            let len = trimmed.len() - 1;
            objects.push(Elem::new(&lines[pos].trim()[8..len], pos));
        } else if lines[pos].trim().is_empty() {
            in_object = false;
        }
    }

    if !objects.is_empty() {
        let mut local_errors = 0;
        for it in 0..objects.len() - 1 {
            if objects[it] > objects[it + 1] {
                messages.push(format!("ERROR: \"{}\" should be after \"{}\"",
                                      objects[it].name,
                                      objects[it + 1].name));
                local_errors += 1;
            }
        }
        if local_errors > 0 {
            objects.sort();
            messages.push(format!("\n== Expected order ==\n{}",
                                  objects.iter()
                                         .map(|l| format!("{}\n", l.name))
                                         .collect::<String>()));
            errors += local_errors;
        }
    }

    Errors {
        nb_errors: errors,
        messages,
    }
}

pub fn check_gir_file<P: AsRef<Path>>(p: P) -> Errors {
    let content = utils::read_file(p);
    check_gir_content(&content)
}
