// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_ClusterSharedVolume struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_ClusterSharedVolume {
    #[serde(flatten)]
    pub base: MSCluster_LogicalElement,

/// 
    #[serde(rename = "BackupState")]
    pub backup_state: Option<u32>,

/// 
    #[serde(rename = "FaultState")]
    pub fault_state: Option<u32>,

/// 
    #[serde(rename = "VolumeName")]
    pub volume_name: Option<String>,

/// 
    #[serde(rename = "VolumeOffset")]
    pub volume_offset: Option<u64>,
}

impl MSCluster_ClusterSharedVolume {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_LogicalElement::new(),
            backup_state: None,
            fault_state: None,
            volume_name: None,
            volume_offset: None,
        }
    }


    /// Sets the value of BackupState
    pub fn set_backup_state(&mut self, value: u32) {
        self.backup_state = Some(value);
    }

    /// Gets the value of BackupState
    pub fn get_backup_state(&self) -> Option<&u32> {
        self.backup_state.as_ref()
    }

    /// Sets the value of FaultState
    pub fn set_fault_state(&mut self, value: u32) {
        self.fault_state = Some(value);
    }

    /// Gets the value of FaultState
    pub fn get_fault_state(&self) -> Option<&u32> {
        self.fault_state.as_ref()
    }

    /// Sets the value of VolumeName
    pub fn set_volume_name(&mut self, value: String) {
        self.volume_name = Some(value);
    }

    /// Gets the value of VolumeName
    pub fn get_volume_name(&self) -> Option<&String> {
        self.volume_name.as_ref()
    }

    /// Sets the value of VolumeOffset
    pub fn set_volume_offset(&mut self, value: u64) {
        self.volume_offset = Some(value);
    }

    /// Gets the value of VolumeOffset
    pub fn get_volume_offset(&self) -> Option<&u64> {
        self.volume_offset.as_ref()
    }

/// 
    pub fn turn_on_maintenance(&self) -> Result<(), WmiError> {
        self.invoke_method("TurnOnMaintenance", &[])

    }


/// 
    pub fn turn_off_maintenance(&self) -> Result<(), WmiError> {
        self.invoke_method("TurnOffMaintenance", &[])

    }


/// 
    pub fn turn_on_redirected_access(&self) -> Result<(), WmiError> {
        self.invoke_method("TurnOnRedirectedAccess", &[])

    }


/// 
    pub fn turn_off_redirected_access(&self) -> Result<(), WmiError> {
        self.invoke_method("TurnOffRedirectedAccess", &[])

    }


/// 

    /// * `host_name` -  (String)
    pub fn move_to_new_host(&self, host_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "HostName".to_string(), value: host_name.into() });
        self.invoke_method("MoveToNewHost", &args)

    }

}

