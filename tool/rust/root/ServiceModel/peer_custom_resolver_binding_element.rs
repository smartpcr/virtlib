// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PeerCustomResolverBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PeerCustomResolverBindingElement {
    #[serde(flatten)]
    pub base: PeerResolverBindingElement,

/// The address of the peer custom resolver.
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// The configuration name of the binding.
    #[serde(rename = "Binding")]
    pub binding: Option<String>,
}

impl PeerCustomResolverBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PeerResolverBindingElement::new(),
            address: None,
            binding: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of Binding
    pub fn set_binding(&mut self, value: String) {
        self.binding = Some(value);
    }

    /// Gets the value of Binding
    pub fn get_binding(&self) -> Option<&String> {
        self.binding.as_ref()
    }
}

