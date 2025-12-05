// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MessageEncodingBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MessageEncodingBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// The SOAP version of the messages sent using the binding.
    #[serde(rename = "MessageVersion")]
    pub message_version: Option<String>,
}

impl MessageEncodingBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            message_version: None,
        }
    }


    /// Sets the value of MessageVersion
    pub fn set_message_version(&mut self, value: String) {
        self.message_version = Some(value);
    }

    /// Gets the value of MessageVersion
    pub fn get_message_version(&self) -> Option<&String> {
        self.message_version.as_ref()
    }
}

