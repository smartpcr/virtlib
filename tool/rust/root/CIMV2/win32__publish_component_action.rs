// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PublishComponentAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PublishComponentAction {
    #[serde(flatten)]
    pub base: CIM_Action,

/// 
    #[serde(rename = "AppData")]
    pub app_data: Option<String>,

/// 
    #[serde(rename = "ComponentID")]
    pub component_id: Option<String>,

/// 
    #[serde(rename = "Qual")]
    pub qual: Option<String>,
}

impl Win32_PublishComponentAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Action::new(),
            app_data: None,
            component_id: None,
            qual: None,
        }
    }


    /// Sets the value of AppData
    pub fn set_app_data(&mut self, value: String) {
        self.app_data = Some(value);
    }

    /// Gets the value of AppData
    pub fn get_app_data(&self) -> Option<&String> {
        self.app_data.as_ref()
    }

    /// Sets the value of ComponentID
    pub fn set_component_id(&mut self, value: String) {
        self.component_id = Some(value);
    }

    /// Gets the value of ComponentID
    pub fn get_component_id(&self) -> Option<&String> {
        self.component_id.as_ref()
    }

    /// Sets the value of Qual
    pub fn set_qual(&mut self, value: String) {
        self.qual = Some(value);
    }

    /// Gets the value of Qual
    pub fn get_qual(&self) -> Option<&String> {
        self.qual.as_ref()
    }
}

