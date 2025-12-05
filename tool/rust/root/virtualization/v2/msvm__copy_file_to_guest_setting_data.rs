// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CopyFileToGuestSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CopyFileToGuestSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "CreateFullPath")]
    pub create_full_path: Option<bool>,

/// 
    #[serde(rename = "DestinationPath")]
    pub destination_path: Option<String>,

/// 
    #[serde(rename = "OverwriteExisting")]
    pub overwrite_existing: Option<bool>,

/// 
    #[serde(rename = "SourcePath")]
    pub source_path: Option<String>,
}

impl Msvm_CopyFileToGuestSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            create_full_path: None,
            destination_path: None,
            overwrite_existing: None,
            source_path: None,
        }
    }


    /// Sets the value of CreateFullPath
    pub fn set_create_full_path(&mut self, value: bool) {
        self.create_full_path = Some(value);
    }

    /// Gets the value of CreateFullPath
    pub fn get_create_full_path(&self) -> Option<&bool> {
        self.create_full_path.as_ref()
    }

    /// Sets the value of DestinationPath
    pub fn set_destination_path(&mut self, value: String) {
        self.destination_path = Some(value);
    }

    /// Gets the value of DestinationPath
    pub fn get_destination_path(&self) -> Option<&String> {
        self.destination_path.as_ref()
    }

    /// Sets the value of OverwriteExisting
    pub fn set_overwrite_existing(&mut self, value: bool) {
        self.overwrite_existing = Some(value);
    }

    /// Gets the value of OverwriteExisting
    pub fn get_overwrite_existing(&self) -> Option<&bool> {
        self.overwrite_existing.as_ref()
    }

    /// Sets the value of SourcePath
    pub fn set_source_path(&mut self, value: String) {
        self.source_path = Some(value);
    }

    /// Gets the value of SourcePath
    pub fn get_source_path(&self) -> Option<&String> {
        self.source_path.as_ref()
    }
}

