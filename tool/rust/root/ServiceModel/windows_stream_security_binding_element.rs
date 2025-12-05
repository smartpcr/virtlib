// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WindowsStreamSecurityBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowsStreamSecurityBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// The ProtectionLevel for the TCP stream.
    #[serde(rename = "ProtectionLevel")]
    pub protection_level: Option<String>,
}

impl WindowsStreamSecurityBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            protection_level: None,
        }
    }


    /// Sets the value of ProtectionLevel
    pub fn set_protection_level(&mut self, value: String) {
        self.protection_level = Some(value);
    }

    /// Gets the value of ProtectionLevel
    pub fn get_protection_level(&self) -> Option<&String> {
        self.protection_level.as_ref()
    }
}

