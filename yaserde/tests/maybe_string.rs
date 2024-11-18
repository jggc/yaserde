#[macro_use]
extern crate yaserde_derive;
#[macro_use]
extern crate yaserde;

use yaserde::MaybeString;
use yaserde_derive::YaDeserialize;
use yaserde_derive::YaSerialize;

#[derive(Debug, PartialEq, Default, YaDeserialize, YaSerialize)]
struct TestStruct {
  maybe: MaybeString,
}

#[test]
fn maybe_string_should_deserialize_empty_element() {
  let initial_xml = "<struct><maybe/></struct>";
  let test_struct: TestStruct =
    yaserde::de::from_str(initial_xml).expect("Shoudl deserialize teststruct");
  println!("Got test_struct {:?}", test_struct);
  assert_eq!(
    test_struct,
    TestStruct {
      maybe: MaybeString {
        field_name: String::from("maybe"),
        content: None
      }
    }
  );
}

#[test]
fn maybe_string_should_deserialize_content() {
  let initial_xml = "<struct><maybe>some content</maybe></struct>";
  let test_struct: TestStruct =
    yaserde::de::from_str(initial_xml).expect("Shoudl deserialize teststruct");
  println!("Got test_struct {:?}", test_struct);
  assert_eq!(
    test_struct,
    TestStruct {
      maybe: MaybeString {
        field_name: String::from("maybe"),
        content: Some(String::from("some content"))
      }
    }
  );
}

#[test]
fn maybe_string_should_deserialize_empty_long_format() {
  let initial_xml = "<struct><maybe></maybe></struct>";
  let test_struct: TestStruct =
    yaserde::de::from_str(initial_xml).expect("Shoudl deserialize teststruct");
  println!("Got test_struct {:?}", test_struct);
  assert_eq!(
    test_struct,
    TestStruct {
      maybe: MaybeString {
        field_name: String::from("maybe"),
        content: None
      }
    }
  );
}

#[test]
fn maybe_string_should_serialize_to_empty_element() {
  let initial_xml = r#"<?xml version="1.0" encoding="utf-8"?><TestStruct><maybe /></TestStruct>"#;
  let test_struct: TestStruct =
    yaserde::de::from_str(initial_xml).expect("Shoudl deserialize teststruct");
  println!("Got test_struct {:?}", test_struct);
  assert_eq!(
    yaserde::ser::to_string(&test_struct).expect("should serialize teststruct"),
    initial_xml
  );
}

#[test]
fn maybe_string_should_serialize_content() {
  let initial_xml =
    r#"<?xml version="1.0" encoding="utf-8"?><TestStruct><maybe>some content</maybe></TestStruct>"#;
  let test_struct: TestStruct =
    yaserde::de::from_str(initial_xml).expect("Shoudl deserialize teststruct");
  println!("Got test_struct {:?}", test_struct);
  assert_eq!(
    yaserde::ser::to_string(&test_struct).expect("should serialize teststruct"),
    initial_xml
  );
}
