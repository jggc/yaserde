#[macro_use]
extern crate yaserde_derive;
#[macro_use]
extern crate yaserde;

use yaserde::{RawXml, NamedList};

#[test]
fn deserialize_namedlist() {
  let mut expected_elements: Vec<(String, RawXml)> = Vec::new();
  expected_elements.push((String::from("struct1"), RawXml::default()));

  let deserialized: NamedList<RawXml> =
    yaserde::de::from_str("<struct1><foo>foo</foo></struct1>").unwrap();
  assert_eq!(expected_elements, deserialized.elements);
}
