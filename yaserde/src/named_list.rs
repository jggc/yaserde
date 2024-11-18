use log::trace;

use crate::de::Deserializer;

#[derive(Default, PartialEq, Debug)]
pub struct NamedList<T: crate::YaDeserialize + crate::YaSerialize + std::fmt::Debug> {
    pub elements: Vec<(String, T)>,
}

impl<T> crate::YaDeserialize for NamedList<T>
where
  T: crate::YaDeserialize + crate::YaSerialize + std::fmt::Debug,
{
  fn deserialize<R: std::io::Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let start_depth = reader.depth();
    let mut elements: Vec<(String, T)> = Vec::new();
    {
      let curr = reader.peek()?;
      if let xml::reader::XmlEvent::StartElement {
        name,
        attributes,
        namespace,
      } = reader.peek()?.to_owned()
      {
        println!(
          "StartElement with name {} attributes {:?} depth {}",
          name,
          attributes,
          reader.depth()
        );
      }
    }

    loop {
      let current_event = reader.peek()?.to_owned();
      match current_event {
        xml::reader::XmlEvent::StartDocument {
          version,
          encoding,
          standalone,
        } => {
          unimplemented!("HashMap deserializer got StartDocument");
        }
        xml::reader::XmlEvent::EndDocument => {
          unimplemented!("HashMap deserializer got EndDocument");
        }
        xml::reader::XmlEvent::ProcessingInstruction { name, data } => {
          unimplemented!("HashMap deserializer got ProcessingInstruction")
        }
        xml::reader::XmlEvent::StartElement {
          name, namespace, ..
        } => {
          println!(
            "HashMap deserializer got StartElement name {:?} namespace {:?}",
            name, namespace,
          );
          let child = T::deserialize(reader)?;
          println!("HashMap deserialize inserting child {:?}", child);
          elements.push((name.to_string(), child));
          let peek = reader.peek()?;
          println!("Peek {:?}", peek);
          // if let xml::reader::XmlEvent::EndElement { name: peek_name } = peek {
          //   if peek_name == &name {
          //     println!("Calling next_event after processing StartElement");
          //     reader.next_event()?;
          //   }
          // }
        }
        xml::reader::XmlEvent::EndElement { name } => {
          println!("HashMap deserializer got EndElement {name}");
          println!(
            "Next_event() Peek : {:?}, depth : {}",
            reader.peek()?.to_owned(),
            reader.depth()
          );
          if reader.depth() > start_depth {
            println!(
              "Current depth {} greater than start depth {}, consuming event",
              reader.depth(),
              start_depth
            );
            reader.next_event()?;
          } else {
            println!(
              "Current depth {} is start depth {}, exiting HashMap deserializer",
              reader.depth(),
              start_depth
            );
            reader.inner_next();
            break;
          }
        }
        xml::reader::XmlEvent::CData(_) => unimplemented!("HashMap deserializer got CData"),
        xml::reader::XmlEvent::Comment(_) => unimplemented!("HashMap deserializer got Comment"),
        xml::reader::XmlEvent::Characters(_) => {
          unimplemented!("HashMap deserializer got Characters")
        }
        xml::reader::XmlEvent::Whitespace(_) => {
          unimplemented!("HashMap deserializer got Whitespace")
        }
      }
    }
    println!("HashMap Deserializer done, reader.peek() {:?}", reader.peek()?);
    reader.next_event()?;
    println!("HashMap Deserializer done, reader.peek() {:?}", reader.peek()?);

    todo!("Hey I'm done with elements {:?}", elements);
    Ok(Self { elements })
  }
}

impl<T: crate::YaDeserialize + crate::YaSerialize + std::fmt::Debug> crate::YaSerialize for NamedList<T> {
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
    let mut expected_elements: Vec<(String, RawXml)> = Vec::new();
    expected_elements.push((String::from("struct1"), RawXml::default()));

    let deserialized: NamedList<RawXml> = crate::de::from_str("<struct1><foo>foo</foo></struct1>").unwrap();
    assert_eq!(expected_elements, deserialized.elements);
  }
}
