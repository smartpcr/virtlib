// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ShadowStorage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ShadowStorage {

/// 
    #[serde(rename = "AllocatedSpace")]
    pub allocated_space: Option<u64>,

/// 
    #[serde(rename = "DiffVolume")]
    pub diff_volume: Option<Win32_Volume>,

/// 
    #[serde(rename = "MaxSpace")]
    pub max_space: Option<u64>,

/// 
    #[serde(rename = "UsedSpace")]
    pub used_space: Option<u64>,

/// 
    #[serde(rename = "Volume")]
    pub volume: Option<Win32_Volume>,
}

impl Win32_ShadowStorage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allocated_space: None,
            diff_volume: None,
            max_space: None,
            used_space: None,
            volume: None,
        }
    }


    /// Sets the value of AllocatedSpace
    pub fn set_allocated_space(&mut self, value: u64) {
        self.allocated_space = Some(value);
    }

    /// Gets the value of AllocatedSpace
    pub fn get_allocated_space(&self) -> Option<&u64> {
        self.allocated_space.as_ref()
    }

    /// Sets the value of DiffVolume
    pub fn set_diff_volume(&mut self, value: Win32_Volume) {
        self.diff_volume = Some(value);
    }

    /// Gets the value of DiffVolume
    pub fn get_diff_volume(&self) -> Option<&Win32_Volume> {
        self.diff_volume.as_ref()
    }

    /// Sets the value of MaxSpace
    pub fn set_max_space(&mut self, value: u64) {
        self.max_space = Some(value);
    }

    /// Gets the value of MaxSpace
    pub fn get_max_space(&self) -> Option<&u64> {
        self.max_space.as_ref()
    }

    /// Sets the value of UsedSpace
    pub fn set_used_space(&mut self, value: u64) {
        self.used_space = Some(value);
    }

    /// Gets the value of UsedSpace
    pub fn get_used_space(&self) -> Option<&u64> {
        self.used_space.as_ref()
    }

    /// Sets the value of Volume
    pub fn set_volume(&mut self, value: Win32_Volume) {
        self.volume = Some(value);
    }

    /// Gets the value of Volume
    pub fn get_volume(&self) -> Option<&Win32_Volume> {
        self.volume.as_ref()
    }

/// 

    /// * `diff_volume` -  (String)
    /// * `max_space` -  (u64)
    /// * `volume` -  (String)

    /// * `return_value` -  (u32)
    pub fn create(&self, volume: &String, diff_volume: &String, max_space: u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Volume".to_string(), value: volume.into() });
        args.push(MethodParameter { name: "DiffVolume".to_string(), value: diff_volume.into() });
        args.push(MethodParameter { name: "MaxSpace".to_string(), value: max_space.into() });
        self.invoke_method("Create", &args)

    }

}

