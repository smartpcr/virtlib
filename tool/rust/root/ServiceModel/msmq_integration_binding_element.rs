// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsmqIntegrationBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsmqIntegrationBindingElement {
    #[serde(flatten)]
    pub base: MsmqBindingElementBase,

/// The format the binding uses to serialize messages.
    #[serde(rename = "SerializationFormat")]
    pub serialization_format: Option<String>,
}

impl MsmqIntegrationBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MsmqBindingElementBase::new(),
            serialization_format: None,
        }
    }


    /// Sets the value of SerializationFormat
    pub fn set_serialization_format(&mut self, value: String) {
        self.serialization_format = Some(value);
    }

    /// Gets the value of SerializationFormat
    pub fn get_serialization_format(&self) -> Option<&String> {
        self.serialization_format.as_ref()
    }
}

