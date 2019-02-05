use serde::ser::SerializeMap;
use serde::Serialize;
use serde::Serializer;

use crate::dsl::schema::DocumentRoot;
use crate::output::JsonSchema;
use crate::output::serialization::properties::serialize_schema;

// we output Draft 4 of the Json Schema specification because the downstream consumers
// of the JSON schema we produce fully support Draft 4, and not really Draft 7;
// in general most of the tools and libraries on the internet understand Draft 4 but have some problems with Draft 7
const SCHEMA_URL: &str = "http://json-schema.org/draft-04/schema#";

impl<'a> Serialize for JsonSchema<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("$schema", &self.schema_url)?;

        serialize_schema(&self.root, &mut map)?;

        map.end()
    }
}

impl<'a> From<DocumentRoot> for JsonSchema<'a> {
    fn from(root: DocumentRoot) -> Self {
        JsonSchema {
            root: root.schema(),
            schema_url: SCHEMA_URL,
        }
    }
}
