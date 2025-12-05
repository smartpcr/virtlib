// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CompositeDuplexBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompositeDuplexBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// The base address of the client.
    #[serde(rename = "ClientBaseAddress")]
    pub client_base_address: Option<String>,
}

impl CompositeDuplexBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            client_base_address: None,
        }
    }


    /// Sets the value of ClientBaseAddress
    pub fn set_client_base_address(&mut self, value: String) {
        self.client_base_address = Some(value);
    }

    /// Gets the value of ClientBaseAddress
    pub fn get_client_base_address(&self) -> Option<&String> {
        self.client_base_address.as_ref()
    }
}

