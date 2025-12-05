// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V1_Services struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V1_Services {
    #[serde(flatten)]
    pub base: SystemConfig_V1,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Vec<char>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "ProcessName")]
    pub process_name: Vec<char>,

/// 
    #[serde(rename = "ServiceName")]
    pub service_name: Vec<char>,
}

impl SystemConfig_V1_Services {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V1::new(),
            display_name: Vec::new(),
            process_id: None,
            process_name: Vec::new(),
            service_name: Vec::new(),
        }
    }


    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: Vec<char>) {
        self.display_name = value;
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> &Vec<char> {
        &self.display_name
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of ProcessName
    pub fn set_process_name(&mut self, value: Vec<char>) {
        self.process_name = value;
    }

    /// Gets the value of ProcessName
    pub fn get_process_name(&self) -> &Vec<char> {
        &self.process_name
    }

    /// Sets the value of ServiceName
    pub fn set_service_name(&mut self, value: Vec<char>) {
        self.service_name = value;
    }

    /// Gets the value of ServiceName
    pub fn get_service_name(&self) -> &Vec<char> {
        &self.service_name
    }
}

