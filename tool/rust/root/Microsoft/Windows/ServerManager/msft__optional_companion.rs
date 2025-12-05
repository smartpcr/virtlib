// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_OptionalCompanion struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_OptionalCompanion {

/// 
    #[serde(rename = "CompanionComponentName")]
    pub companion_component_name: Option<String>,

/// 
    #[serde(rename = "CompanionType")]
    pub companion_type: Option<u8>,

/// 
    #[serde(rename = "PrerequisiteEnabled")]
    pub prerequisite_enabled: Option<bool>,
}

impl MSFT_OptionalCompanion {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            companion_component_name: None,
            companion_type: None,
            prerequisite_enabled: None,
        }
    }


    /// Sets the value of CompanionComponentName
    pub fn set_companion_component_name(&mut self, value: String) {
        self.companion_component_name = Some(value);
    }

    /// Gets the value of CompanionComponentName
    pub fn get_companion_component_name(&self) -> Option<&String> {
        self.companion_component_name.as_ref()
    }

    /// Sets the value of CompanionType
    pub fn set_companion_type(&mut self, value: u8) {
        self.companion_type = Some(value);
    }

    /// Gets the value of CompanionType
    pub fn get_companion_type(&self) -> Option<&u8> {
        self.companion_type.as_ref()
    }

    /// Sets the value of PrerequisiteEnabled
    pub fn set_prerequisite_enabled(&mut self, value: bool) {
        self.prerequisite_enabled = Some(value);
    }

    /// Gets the value of PrerequisiteEnabled
    pub fn get_prerequisite_enabled(&self) -> Option<&bool> {
        self.prerequisite_enabled.as_ref()
    }
}

