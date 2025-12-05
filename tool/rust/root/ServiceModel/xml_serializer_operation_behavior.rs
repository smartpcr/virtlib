// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// XmlSerializerOperationBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XmlSerializerOperationBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// Defines the style of the SOAP message.
    #[serde(rename = "Style")]
    pub style: Option<String>,

/// Specifies the SOAP encoding style.
    #[serde(rename = "Use")]
    pub use: Option<String>,
}

impl XmlSerializerOperationBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            style: None,
            use: None,
        }
    }


    /// Sets the value of Style
    pub fn set_style(&mut self, value: String) {
        self.style = Some(value);
    }

    /// Gets the value of Style
    pub fn get_style(&self) -> Option<&String> {
        self.style.as_ref()
    }

    /// Sets the value of Use
    pub fn set_use(&mut self, value: String) {
        self.use = Some(value);
    }

    /// Gets the value of Use
    pub fn get_use(&self) -> Option<&String> {
        self.use.as_ref()
    }
}

