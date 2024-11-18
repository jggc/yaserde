#[macro_use]
extern crate yaserde;

use yaserde::RawXml;

use yaserde_derive::YaDeserialize;
use yaserde_derive::YaSerialize;

pub fn to_xml_str<T: yaserde::YaSerialize>(model: &T) -> Result<String, String> {
    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        write_document_declaration: false,
        pad_self_closing: false,
        ..Default::default()
    };
    let serialized = yaserde::ser::to_string_with_config::<T>(model, &yaserde_cfg)?;

    // Opnsense does not specify encoding in the document declaration
    //
    // yaserde / xml-rs does not allow disabling the encoding attribute in the
    // document declaration
    //
    // So here we just manually prefix the xml document with the exact document declaration
    // that opnsense uses
    Ok(format!("<?xml version=\"1.0\"?>\n{serialized}\n"))
}

#[derive(Debug, PartialEq, Default, YaDeserialize)]
pub struct Parent {
  //    pub rawxml_child: RawXml,
  pub string_child: String,
  pub child_child: Child,
}

#[derive(Debug, PartialEq, Default, YaDeserialize)]
pub struct Child {
  pub child_val: String,
  pub child_val2: String,
  pub child_option: Option<String>,
}

#[test]
fn rawxml_should_buffer_empty_element() {
  let rawxml: RawXml = yaserde::de::from_str("<something/>").unwrap();
  assert_eq!(rawxml.0, String::from("<something></something>"));
}

#[test]
fn rawxml_should_buffer_elements_with_different_case_as_they_are() {
  let xml = "<xml><Some_thing></Some_thing><something></something></xml>";
  let rawxml: RawXml = yaserde::de::from_str(xml).unwrap();
  assert_eq!(rawxml.0, String::from(xml));
}

#[test]
fn rawxml_should_buffer_elements_with_attributes() {
  let xml = r#"<xml version="ababa"><Some_thing></Some_thing><something></something></xml>"#;
  let rawxml: RawXml = yaserde::de::from_str(xml).unwrap();
  assert_eq!(rawxml.0, String::from(xml));
}

#[test]
fn rawxml_should_handle_complex_documents() {
  let xml = r#"<xml><OpenVPN version="1.0.0"><Overwrites></Overwrites><Instances></Instances><StaticKeys></StaticKeys></OpenVPN><Gateways version="0.0.1"></Gateways><HAProxy version="4.0.0"><general><enabled>1</enabled><gracefulStop>0</gracefulStop><hardStopAfter>60s</hardStopAfter><closeSpreadTime></closeSpreadTime><seamlessReload>0</seamlessReload><storeOcsp>0</storeOcsp><showIntro>1</showIntro><peers><enabled>0</enabled><name1></name1><listen1></listen1><port1>1024</port1><name2></name2><listen2></listen2><port2>1024</port2></peers><tuning><root>0</root><maxConnections></maxConnections><nbthread>1</nbthread><sslServerVerify>ignore</sslServerVerify><maxDHSize>2048</maxDHSize><bufferSize>16384</bufferSize></tuning></general></HAProxy></xml>"#;
  let rawxml: RawXml = yaserde::de::from_str(xml).unwrap();
  assert_eq!(rawxml.0, String::from(xml));
}

#[test]
fn rawxml_should_serialize_simple_documents() {
  let xml = r#"<?xml version="1.0" encoding="utf-8"?><xml />"#;
  let rawxml: RawXml = yaserde::de::from_str(xml).unwrap();
  assert_eq!(yaserde::ser::to_string(&rawxml).unwrap(), xml);
}

#[test]
fn rawxml_should_serialize_complex_documents() {
  let xml = r#"<?xml version="1.0"?>
<xml>
  <OpenVPN version="1.0.0">
    <Overwrites/>
    <Instances/>
    <StaticKeys/>
  </OpenVPN>
  <Gateways version="0.0.1"/>
  <HAProxy version="4.0.0">
    <general>
      <enabled>1</enabled>
      <gracefulStop>0</gracefulStop>
      <hardStopAfter>60s</hardStopAfter>
      <closeSpreadTime/>
      <seamlessReload>0</seamlessReload>
      <storeOcsp>0</storeOcsp>
      <showIntro>1</showIntro>
      <peers>
        <enabled>0</enabled>
        <name1/>
        <listen1/>
        <port1>1024</port1>
        <name2/>
        <listen2/>
        <port2>1024</port2>
      </peers>
      <tuning>
        <root>0</root>
        <maxConnections/>
        <nbthread>1</nbthread>
        <sslServerVerify>ignore</sslServerVerify>
        <maxDHSize>2048</maxDHSize>
        <bufferSize>16384</bufferSize>
      </tuning>
    </general>
  </HAProxy>
</xml>
"#;
  let rawxml: RawXml = yaserde::de::from_str(xml).unwrap();
  assert_eq!(to_xml_str(&rawxml).unwrap(), xml);
}

#[test]
fn rawxml_should_allow_siblings_before() {
  #[derive(YaDeserialize, YaSerialize)]
  struct Config {
    paul: Vec<String>,
    raw: RawXml,
  }
  let xml = r#"<?xml version="1.0" encoding="utf-8"?><Config><paul>bobob</paul><paul>patate</paul><raw>allo something</raw></Config>"#;
  let config: Config = yaserde::de::from_str(xml).unwrap();
  assert_eq!(yaserde::ser::to_string(&config).unwrap(), xml);
}

#[test]
fn rawxml_should_allow_siblings_after() {
  #[derive(YaDeserialize, YaSerialize)]
  struct Config {
    raw: RawXml,
    paul: Vec<String>,
  }
  let xml = r#"<?xml version="1.0" encoding="utf-8"?><Config><raw>allo something</raw><paul>bobob</paul><paul>patate</paul></Config>"#;
  let config: Config = yaserde::de::from_str(xml).unwrap();
  assert_eq!(config.paul.get(0).unwrap(), "bobob");
  assert_eq!(config.paul.get(1).unwrap(), "patate");
  assert_eq!(config.paul.len(), 2);
  assert_eq!(config.raw.0, "<raw>allo something</raw>");
  assert_eq!(yaserde::ser::to_string(&config).unwrap(), xml);
}

#[test]
fn rawxml_should_allow_being_end_of_document() {
  let xml = r#"<?xml version="1.0" encoding="utf-8"?><Config><raw>allo something</raw><paul>bobob</paul><paul>patate</paul></Config>"#;
  let config: RawXml = yaserde::de::from_str(xml).unwrap();
  assert_eq!(yaserde::ser::to_string(&config).unwrap(), xml);
}
