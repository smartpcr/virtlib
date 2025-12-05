// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Behavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Behavior {

/// The type of the behavior.
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl Behavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            type: None,
        }
    }


    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }
}

