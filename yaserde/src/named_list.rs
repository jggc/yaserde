use log::debug;

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
    log::trace!("NamedList peek {:?}", reader.peek()?);
    if let xml::reader::XmlEvent::StartElement {
      name, attributes, ..
    } = reader.peek()?.to_owned()
    {
      log::trace!(
        "StartElement with name {} attributes {:?} depth {}",
        name,
        attributes,
        reader.depth()
      );
      // Get inside the List.
      // We skip the opening StartElement to get to the list itself
      reader.next_event()?;
    } else {
      return Err(String::from(
        "Expected a StartElement as first event of NamedList",
      ));
    }

    let mut elements: Vec<(String, T)> = Vec::new();
    let start_depth = reader.depth();
    loop {
      let current_event = reader.peek()?.to_owned();
      log::trace!("NamedList loop iterating on event {:?}", current_event);
      match current_event {
        xml::reader::XmlEvent::StartDocument { .. } => {
          unimplemented!("NamedList deserializer got StartDocument");
        }
        xml::reader::XmlEvent::EndDocument => {
          unimplemented!("NamedList deserializer got EndDocument");
        }
        xml::reader::XmlEvent::ProcessingInstruction { .. } => {
          unimplemented!("NamedList deserializer got ProcessingInstruction")
        }
        xml::reader::XmlEvent::StartElement {
          name, namespace, ..
        } => {
          log::trace!(
            "NamedList deserializer got StartElement name {:?} namespace {:?}",
            name, namespace,
          );
          let child = T::deserialize(reader)?;
          log::trace!("NamedList deserialize inserting child {:?}", child);
          elements.push((name.to_string(), child));
        }
        xml::reader::XmlEvent::EndElement { name } => {
          log::trace!("NamedList deserializer got EndElement {name}");
          log::trace!(
            "Next_event() Peek : {:?}, depth : {}",
            reader.peek()?.to_owned(),
            reader.depth()
          );
          if reader.depth() > start_depth {
            log::trace!(
              "Current depth {} greater than start depth {}, consuming event",
              reader.depth(),
              start_depth
            );
            reader.next_event()?;
          } else {
            log::trace!(
              "Current depth {} is start depth {}, exiting NamedList deserializer",
              reader.depth(),
              start_depth
            );
            //reader.inner_next()?;
            break;
          }
        }
        xml::reader::XmlEvent::CData(_) => unimplemented!("NamedList deserializer got CData"),
        xml::reader::XmlEvent::Comment(_) => unimplemented!("NamedList deserializer got Comment"),
        xml::reader::XmlEvent::Characters(_) => {
          unimplemented!("NamedList deserializer got Characters")
        }
        xml::reader::XmlEvent::Whitespace(_) => {
          unimplemented!("NamedList deserializer got Whitespace")
        }
      }
    }
    log::trace!(
      "NamedList Deserializer done, reader.peek() {:?}",
      reader.peek()?
    );

    Ok(Self { elements })
  }
}

impl<T: crate::YaDeserialize + crate::YaSerialize + std::fmt::Debug> crate::YaSerialize
  for NamedList<T>
{
  fn serialize<W: std::io::Write>(
    &self,
    writer: &mut crate::ser::Serializer<W>,
  ) -> Result<(), String> {
    let yaserde_label = writer
      .get_start_event_name()
      .unwrap_or_else(|| "Interface".to_string());

    log::trace!("NamedList serialization starting with start event name {yaserde_label}");

    let struct_start_event = xml::writer::XmlEvent::start_element(yaserde_label.as_ref());
    writer
      .write(struct_start_event)
      .map_err(|_e| format!("Start element {yaserde_label:?} write failed"))?;

    for (name, value) in &self.elements {
      writer.set_skip_start_end(true);
      debug!(
        "Serializing element with name {} and value {:?}",
        name, value
      );

      let element = xml::writer::XmlEvent::start_element(name.as_str());
      log::trace!("NamedList writing start element {name}");
      writer
        .write(element)
        .map_err(|_e| format!("Start element {name:?} write failed"))?;

      log::trace!("Serializing value {:?}", value);
      value.serialize(writer)?;

      // Write end element
      let element = xml::writer::XmlEvent::end_element();
      log::trace!("NamedList writing end element {name}");
      writer
        .write(element)
        .map_err(|_e| format!("End element {name:?} write failed"))?;
    }

    let element = xml::writer::XmlEvent::end_element();
    log::trace!("NamedList writing FINAL end element");
    writer
      .write(element)
      .map_err(|_e| format!("NamedList FINAL End element write failed"))?;

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
    unimplemented!("NamedList does not support attributes at the moment")
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

    let deserialized: NamedList<RawXml> =
      crate::de::from_str("<struct1><foo>foo</foo></struct1>").unwrap();
    assert_eq!(expected_elements, deserialized.elements);
  }
}
