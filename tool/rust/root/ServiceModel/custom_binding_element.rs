// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CustomBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// A string that contains the configuration name of the binding. This value is a user-defined string that acts as the identification string for the custom binding.
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl CustomBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            name: None,
        }
    }


    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

