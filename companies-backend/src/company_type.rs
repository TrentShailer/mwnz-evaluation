use serde::{Deserialize, Serialize};

// I have decided to use the same type for XML and internal representation, this does restrict flexability should one spec change
// without the other changing, but is not a concern in this case.

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Company {
    pub id: i32, // Assuming IDs wont exceed a 32-bit integer
    pub name: String,
    pub description: String,
}

impl Company {
    /// Tries to deserialize an XML string into a Company
    pub fn try_from_xml(xml: String) -> Result<Self, quick_xml::DeError> {
        let company = quick_xml::de::from_str(&xml)?;
        Ok(company)
    }
}
