// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.SecurityClient
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SerializableToXml struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializableToXml {

/// This is an all-in-one state data that uses an XML format with a standard CIM DTD schema
    #[serde(rename = "PackedXml")]
    pub packed_xml: Option<String>,

/// Schema version (major, minor, build, revision)
    #[serde(rename = "SchemaVersion")]
    pub schema_version: Option<String>,
}

impl SerializableToXml {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            packed_xml: None,
            schema_version: None,
        }
    }


    /// Sets the value of PackedXml
    pub fn set_packed_xml(&mut self, value: String) {
        self.packed_xml = Some(value);
    }

    /// Gets the value of PackedXml
    pub fn get_packed_xml(&self) -> Option<&String> {
        self.packed_xml.as_ref()
    }

    /// Sets the value of SchemaVersion
    pub fn set_schema_version(&mut self, value: String) {
        self.schema_version = Some(value);
    }

    /// Gets the value of SchemaVersion
    pub fn get_schema_version(&self) -> Option<&String> {
        self.schema_version.as_ref()
    }
}

