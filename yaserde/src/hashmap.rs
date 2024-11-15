use std::collections::HashMap;

use crate::de::Deserializer;

impl<T> crate::YaDeserialize for HashMap<String, T>
where
  T: crate::YaDeserialize + crate::YaSerialize,
{
  fn deserialize<R: std::io::Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    todo!()
  }
}

impl<T: crate::YaDeserialize + crate::YaSerialize> crate::YaSerialize for HashMap<String, T> {
  fn serialize<W: std::io::Write>(
    &self,
    writer: &mut crate::ser::Serializer<W>,
  ) -> Result<(), String> {
    todo!()
  }

  fn serialize_attributes(
    &self,
    attributes: Vec<xml::attribute::OwnedAttribute>,
    namespace: xml::namespace::Namespace,
  ) -> Result<
    (
      Vec<xml::attribute::OwnedAttribute>,
      xml::namespace::Namespace,
    ),
    String,
  > {
    todo!()
  }
}

#[cfg(test)]
mod test {
  use crate::raw_xml::RawXml;

use super::*;

  #[test]
  fn deserialize_hashmap() {
    let mut expected: HashMap<String, RawXml> = HashMap::new();
    expected.insert(
      String::from("struct1"),
      RawXml::default(),
    );

    let deserialized = crate::de::from_str("<struct1><foo>foo</foo></struct1>").unwrap();
    assert_eq!(expected, deserialized);
  }
}
