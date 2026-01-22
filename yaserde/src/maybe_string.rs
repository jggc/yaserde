use crate::{ser, YaDeserialize, YaSerialize};
use xml::reader::XmlEvent as ReadEvent;
use xml::writer::XmlEvent as WriteEvent;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MaybeString {
  pub field_name: String,
  pub content: Option<String>,
}

impl MaybeString {
  pub fn from_bool_as_int(field_name: &str, b: bool) -> MaybeString {
    MaybeString {
      field_name: field_name.to_string(),
      content: Some(format!("{}", b as i8)),
    }
  }

  pub fn content_string(&self) -> String {
    self.content.clone().unwrap_or_default()
  }
}

impl From<Option<String>> for MaybeString {
  fn from(value: Option<String>) -> Self {
    Self {
      field_name: String::default(),
      content: value,
    }
  }
}

impl From<String> for MaybeString {
  fn from(value: String) -> Self {
    Self {
      field_name: String::default(),
      content: Some(value),
    }
  }
}

impl From<&str> for MaybeString {
  fn from(value: &str) -> Self {
    Self {
      field_name: String::default(),
      content: Some(String::from(value)),
    }
  }
}

impl YaDeserialize for MaybeString {
  fn deserialize<R: std::io::Read>(
    reader: &mut crate::de::Deserializer<R>,
  ) -> Result<Self, String> {
    let field_name = match reader.peek()? {
      ReadEvent::StartElement {
        name, attributes, ..
      } => {
        if attributes.len() > 0 {
          return Err(String::from(
            "Attributes not currently supported by MaybeString",
          ));
        }

        name.local_name.clone()
      }
      _ => return Err(String::from("Unsupporte ReadEvent type")),
    };
    reader.next_event()?;

    let content = match reader.peek()? {
      ReadEvent::Characters(content) => Some(content.clone()),
      ReadEvent::EndElement { name } => {
        if name.local_name != field_name {
          return Err(format!(
            "Invalid EndElement, expected {field_name} but got {}",
            name.local_name
          ));
        }
        None
      }
      _ => return Err(String::from("Unsupporte ReadEvent type")),
    };

    Ok(Self {
      field_name,
      content,
    })
  }
}

impl YaSerialize for MaybeString {
  fn serialize<W: std::io::Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String> {
    if let Some(field_name) = writer.get_start_event_name() {
      let start_element_event = WriteEvent::start_element(field_name.as_str());
      writer.write(start_element_event).expect("Writer failed");
    } else {
      let start_element_event = WriteEvent::start_element(self.field_name.as_str());
      writer.write(start_element_event).expect("Writer failed");
    };

    match &self.content {
      Some(content) => {
        writer
          .write(WriteEvent::characters(content))
          .expect("Writer failed");
      }
      None => {}
    };

    writer
      .write(WriteEvent::end_element())
      .expect("Writer failed");
    Ok(())
  }

  fn serialize_attributes(
    &self,
    _attributes: Vec<xml::attribute::OwnedAttribute>,
    _namespace: xml::namespace::Namespace,
  ) -> Result<
    (
      Vec<xml::attribute::OwnedAttribute>,
      xml::namespace::Namespace,
    ),
    String,
  > {
    unimplemented!("MaybeString does not currently support attributes")
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bool_as_int_true() {
        let field_name = "test_field";
        let result = MaybeString::from_bool_as_int(field_name, true);

        assert_eq!(result.field_name, field_name);
        // Verify 'true' is converted to "1"
        assert_eq!(result.content, Some("1".to_string()));
    }

    #[test]
    fn test_from_bool_as_int_false() {
        let field_name = "test_field";
        let result = MaybeString::from_bool_as_int(field_name, false);

        assert_eq!(result.field_name, field_name);
        // Verify 'false' is converted to "0"
        assert_eq!(result.content, Some("0".to_string()));
    }
}
