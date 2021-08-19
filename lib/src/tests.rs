use tempfile::NamedTempFile;

use crate::ShedConfig;

#[test]
fn shed() {
  let mut tmp = NamedTempFile::new().unwrap();
  let shed: ShedConfig = ShedConfig::from_str(
    r#"#![enable(implicit_some)]
(
  id: "12345",
  shed_path: ".",
  pkg_path: "pkg",
  contrib_path: "contrib",
)

"#,
  )
  .unwrap();

  assert!(shed.write(tmp.path()).is_ok());
  assert!(ShedConfig::load(tmp.path().to_str().unwrap()).is_ok());
}
