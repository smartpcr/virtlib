// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TraceListenerArgument struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TraceListenerArgument {

/// The name of the argument.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The value of the argument.
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

impl TraceListenerArgument {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            name: None,
            value: None,
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

    /// Sets the value of Value
    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    /// Gets the value of Value
    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }
}

