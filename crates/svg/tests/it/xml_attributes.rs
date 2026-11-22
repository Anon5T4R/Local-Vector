//! Attribute identity at the XML level: the same attribute written twice on one element is not
//! well-formed XML (https://www.w3.org/TR/xml/#sec-logical-struct), and the import refuses the
//! file instead of keeping one of the two values (the upstream reader fix in vectorcraft
//! `cb5c52f5e6`).
use localvector_svg::{SvgError, import, import_with_report};

fn svg(body: &str) -> String {
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="100" height="100">{body}</svg>"#)
}

/// The first error of `svg`, if the import refuses it.
fn refusal(svg: &str) -> Option<SvgError> {
    import(svg).err()
}

#[test]
fn an_attribute_written_twice_on_one_element_is_refused() {
    let duplicated = svg(r#"<rect x="10" y="10" width="30" width="40" height="30" fill="red"/>"#);
    match refusal(&duplicated) {
        Some(SvgError::Parse(message)) => {
            assert!(message.contains("width"), "the error names the repeated attribute: {message}");
        }
        Some(other) => panic!("a duplicated attribute must be an XML parse error, got {other:?}"),
        None => panic!("a duplicated attribute must not be imported"),
    }
}

#[test]
fn a_duplicated_attribute_in_a_namespace_is_refused_too() {
    // The prefix does not make it a different attribute: the same `href` twice is still one
    // attribute written twice.
    let duplicated = svg(r#"<image x="0" y="0" width="10" height="10" xlink:href="a.png" xlink:href="b.png"/>"#);
    assert!(matches!(refusal(&duplicated), Some(SvgError::Parse(_))), "a repeated namespaced attribute is refused");
}

#[test]
fn a_duplicated_attribute_inside_a_nested_element_is_refused() {
    let duplicated = svg(r#"<g><rect x="0" y="0" width="10" height="10" style="fill: red" style="fill: blue"/></g>"#);
    assert!(matches!(refusal(&duplicated), Some(SvgError::Parse(_))), "the check covers every element, not only the root");
}

#[test]
fn the_same_attribute_on_different_elements_is_fine() {
    let one = svg(r#"<g><rect x="0" y="0" width="10" height="10"/><rect x="5" y="5" width="10" height="10"/></g>"#);
    assert!(refusal(&one).is_none(), "an attribute repeated on sibling elements is well formed");
    let (_, warnings) = import_with_report(&one).expect("import");
    assert!(warnings.is_empty(), "{warnings:?}");
}
