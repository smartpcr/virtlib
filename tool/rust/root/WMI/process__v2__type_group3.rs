// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Process_V2_TypeGroup3 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Process_V2_TypeGroup3 {
    #[serde(flatten)]
    pub base: Process_V2,

/// 
    #[serde(rename = "DirectoryTableBase")]
    pub directory_table_base: Option<u32>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,
}

impl Process_V2_TypeGroup3 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Process_V2::new(),
            directory_table_base: None,
            process_id: None,
        }
    }


    /// Sets the value of DirectoryTableBase
    pub fn set_directory_table_base(&mut self, value: u32) {
        self.directory_table_base = Some(value);
    }

    /// Gets the value of DirectoryTableBase
    pub fn get_directory_table_base(&self) -> Option<&u32> {
        self.directory_table_base.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }
}

