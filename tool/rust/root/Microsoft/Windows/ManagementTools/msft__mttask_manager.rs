// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ManagementTools
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MTTaskManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MTTaskManager {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "CurrentIndex")]
    pub current_index: Option<u16>,

/// 
    #[serde(rename = "IntervalSeconds")]
    pub interval_seconds: Option<u16>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

impl MSFT_MTTaskManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            current_index: None,
            interval_seconds: None,
            name: None,
        }
    }


    /// Sets the value of CurrentIndex
    pub fn set_current_index(&mut self, value: u16) {
        self.current_index = Some(value);
    }

    /// Gets the value of CurrentIndex
    pub fn get_current_index(&self) -> Option<&u16> {
        self.current_index.as_ref()
    }

    /// Sets the value of IntervalSeconds
    pub fn set_interval_seconds(&mut self, value: u16) {
        self.interval_seconds = Some(value);
    }

    /// Gets the value of IntervalSeconds
    pub fn get_interval_seconds(&self) -> Option<&u16> {
        self.interval_seconds.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

/// 

    /// * `seconds` -  (u16)

    /// * `return_value` -  (u32)
    pub fn set_interval(&self, seconds: u16) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Seconds".to_string(), value: seconds.into() });
        self.invoke_method("SetInterval", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn force_refresh(&self) -> Result<(), WmiError> {
        self.invoke_method("ForceRefresh", &[])

    }

}

