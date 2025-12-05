// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Condition struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Condition {
    #[serde(flatten)]
    pub base: CIM_Check,

/// 
    #[serde(rename = "Condition")]
    pub condition: Option<String>,

/// 
    #[serde(rename = "Feature")]
    pub feature: Option<String>,

/// 
    #[serde(rename = "Level")]
    pub level: Option<u16>,
}

impl Win32_Condition {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Check::new(),
            condition: None,
            feature: None,
            level: None,
        }
    }


    /// Sets the value of Condition
    pub fn set_condition(&mut self, value: String) {
        self.condition = Some(value);
    }

    /// Gets the value of Condition
    pub fn get_condition(&self) -> Option<&String> {
        self.condition.as_ref()
    }

    /// Sets the value of Feature
    pub fn set_feature(&mut self, value: String) {
        self.feature = Some(value);
    }

    /// Gets the value of Feature
    pub fn get_feature(&self) -> Option<&String> {
        self.feature.as_ref()
    }

    /// Sets the value of Level
    pub fn set_level(&mut self, value: u16) {
        self.level = Some(value);
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> Option<&u16> {
        self.level.as_ref()
    }
}

