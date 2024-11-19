use crate::{ser, YaDeserialize as YaDeserializeTrait, YaSerialize as YaSerializeTrait};
use xml::reader::XmlEvent as ReadEvent;

#[derive(Debug, PartialEq, Default)]
pub struct RawXml(pub String);

impl YaDeserializeTrait for RawXml {
  fn deserialize<R: std::io::Read>(
    reader: &mut crate::de::Deserializer<R>,
  ) -> Result<Self, String> {
    let mut buffer = String::new();
    let mut depth = 0;

    let own_name = match reader.peek()? {
      ReadEvent::StartElement { name, .. } => name.local_name.clone(),
      _ => return Err("RawXml Should start deserializing with StartElement".to_string()),
    };
    println!("RawXml deserialize from root element name : {own_name}");
    loop {
      let current_event = reader.peek()?.to_owned();
      match current_event.clone() {
        ReadEvent::StartElement {
          name, attributes, ..
        } => {
          println!("StartElement {name} depth {depth}");
          depth += 1;
          let mut attr_string = String::new();
          attributes.iter().for_each(|a| {
            attr_string.push_str(&format!(r#" {}="{}""#, &a.name, &a.value));
          });
          buffer.push_str(&format!("<{}{}>", name, attr_string));
          let _event = reader.next_event()?;
        }
        ReadEvent::EndElement { name } => {
          println!("EndElement {name} depth {depth}");
          depth -= 1;
          buffer.push_str(&format!("</{}>", name));
          println!(
            "Checking if name.local_name {} matches own_name {} at depth {depth}",
            &name.local_name, &own_name
          );
          if name.local_name == own_name && depth == 0 {
            println!("Found next EndElement is closing my struct, breaking out of loop");
            break;
          } else {
            let _event = reader.next_event()?;
          }
        }
        ReadEvent::Characters(content) => {
          println!("Characters {content} depth {depth}");
          buffer.push_str(&content);
          let _event = reader.next_event()?;
        }
        ReadEvent::StartDocument {
          version,
          encoding,
          standalone,
        } => todo!(
          "StartDocument {:?} {:?} {:?}",
          version,
          encoding,
          standalone
        ),
        ReadEvent::EndDocument => todo!(),
        ReadEvent::ProcessingInstruction { name, data } => {
          todo!("ProcessingInstruction {:?}, {:?}", name, data)
        }
        ReadEvent::CData(cdata) => todo!("CData, {:?}", cdata),
        ReadEvent::Comment(comment) => todo!("Comment, {:?}", comment),
        ReadEvent::Whitespace(whitespace) => todo!("Whitespace, {:?}", whitespace),
      }
      let next = reader.peek()?;
      println!(
        "Processing done on \ncurrent_event : {:?} \nnext : {:?}",
        &current_event, &next
      );
    }

    println!("buffered events {buffer}");
    let next = reader.peek()?;
    println!("next : {:?}", &next);

    Ok(RawXml(buffer))
  }
}

impl YaSerializeTrait for RawXml {
  fn serialize<W: std::io::Write>(&self, writer: &mut ser::Serializer<W>) -> Result<(), String> {
    let content = self.0.clone();
    let content = xml::EventReader::from_str(content.as_str());
    let mut reader = crate::de::Deserializer::new(content);
    loop {
      let e = reader.next_event()?;
      if let ReadEvent::EndDocument = e {
        break;
      }
      writer
        .write(e.as_writer_event().unwrap())
        .expect("Writer should write write event");
    }
    Ok(())
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
