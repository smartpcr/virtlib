// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ToDirectoryAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ToDirectoryAction {

/// 
    #[serde(rename = "DestinationDirectory")]
    pub destination_directory: Option<CIM_DirectoryAction>,

/// 
    #[serde(rename = "FileName")]
    pub file_name: Option<CIM_CopyFileAction>,
}

impl CIM_ToDirectoryAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            destination_directory: None,
            file_name: None,
        }
    }


    /// Sets the value of DestinationDirectory
    pub fn set_destination_directory(&mut self, value: CIM_DirectoryAction) {
        self.destination_directory = Some(value);
    }

    /// Gets the value of DestinationDirectory
    pub fn get_destination_directory(&self) -> Option<&CIM_DirectoryAction> {
        self.destination_directory.as_ref()
    }

    /// Sets the value of FileName
    pub fn set_file_name(&mut self, value: CIM_CopyFileAction) {
        self.file_name = Some(value);
    }

    /// Gets the value of FileName
    pub fn get_file_name(&self) -> Option<&CIM_CopyFileAction> {
        self.file_name.as_ref()
    }
}

