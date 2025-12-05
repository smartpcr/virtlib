// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AspNetCompatibilityRequirementsAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AspNetCompatibilityRequirementsAttribute {
    #[serde(flatten)]
    pub base: Behavior,

/// Indicates if Asp.Net compatibility mode is active.
    #[serde(rename = "RequirementsMode")]
    pub requirements_mode: Option<String>,
}

impl AspNetCompatibilityRequirementsAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            requirements_mode: None,
        }
    }


    /// Sets the value of RequirementsMode
    pub fn set_requirements_mode(&mut self, value: String) {
        self.requirements_mode = Some(value);
    }

    /// Gets the value of RequirementsMode
    pub fn get_requirements_mode(&self) -> Option<&String> {
        self.requirements_mode.as_ref()
    }
}

