// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltDeviceInformation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltDeviceInformation {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Number of Paths.
    #[serde(rename = "NumberOfPaths")]
    pub number_of_paths: Option<u32>,

/// Path Info.
    #[serde(rename = "PathInfo")]
    pub path_info: Vec<ClusBfltPathInformation>,
}

impl ClusBfltDeviceInformation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            number_of_paths: None,
            path_info: Vec::new(),
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NumberOfPaths
    pub fn set_number_of_paths(&mut self, value: u32) {
        self.number_of_paths = Some(value);
    }

    /// Gets the value of NumberOfPaths
    pub fn get_number_of_paths(&self) -> Option<&u32> {
        self.number_of_paths.as_ref()
    }

    /// Sets the value of PathInfo
    pub fn set_path_info(&mut self, value: Vec<ClusBfltPathInformation>) {
        self.path_info = value;
    }

    /// Gets the value of PathInfo
    pub fn get_path_info(&self) -> &Vec<ClusBfltPathInformation> {
        &self.path_info
    }
}

