// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

mod check_gir;
mod errors;
mod utils;

pub use check_gir::{check_gir_content, check_gir_file};
pub use errors::Errors;

#[test]
fn simple_check() {
    let content = r#"something = [
    test1,
    test3,
    test2,
]"#;

    let errors = check_gir_content(content);
    assert_eq!(errors.nb_errors, 1);
}

#[test]
fn simple_check2() {
    let content = r#"
[[object]]
name = "abc"
    [[object.func]]
    data = "lol"

[[object]]
name = "aac"
     [[object.func]]
     data = "another"
"#;

    let errors = check_gir_content(content);
    assert_eq!(errors.nb_errors, 1);
}

#[test]
fn simple_check3() {
    let content = r#"something = [
    test1,
    test2,
    test2
]"#;

    let errors = check_gir_content(content);
    assert_eq!(errors.nb_errors, 0);
}

#[test]
fn simple_check4() {
    let content = r#"something = [
    "test1",
    "test3",
    "test2",
]"#;

    let errors = check_gir_content(content);
    assert_eq!(errors.nb_errors, 1);
}

#[test]
fn check_order_with_comments() {
    let content = r#"something = [
    "test1",
    #just a comment
    # on multiple lines
    "test3",
    "test2",
]"#;

    let errors = check_gir_content(content);
    assert_eq!(errors.nb_errors, 1);
    assert_eq!(errors.to_string(), r#"ERROR: "test3" should be after "test2"

== Expected output ==
something = [
    "test1",
    "test2",
    #just a comment
    # on multiple lines
    "test3",
]"#);
}

#[test]
fn check_order_with_comments2() {
    let content = r#"something = [
    #just a comment
    # on multiple lines
    "test3",
    "test1",
    "test2",
]"#;

    let errors = check_gir_content(content);
    assert_eq!(errors.nb_errors, 1);
    assert_eq!(errors.to_string(), r#"ERROR: "test3" should be after "test1"

== Expected output ==
something = [
    "test1",
    "test2",
    #just a comment
    # on multiple lines
    "test3",
]"#);
}
