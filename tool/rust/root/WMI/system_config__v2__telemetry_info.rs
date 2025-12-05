// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_TelemetryInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_TelemetryInfo {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "MachineId")]
    pub machine_id: Option<serde_json::Value>,
}

impl SystemConfig_V2_TelemetryInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            machine_id: None,
        }
    }


    /// Sets the value of MachineId
    pub fn set_machine_id(&mut self, value: serde_json::Value) {
        self.machine_id = Some(value);
    }

    /// Gets the value of MachineId
    pub fn get_machine_id(&self) -> Option<&serde_json::Value> {
        self.machine_id.as_ref()
    }
}

