// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_NTEventlogFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_NTEventlogFile {
    #[serde(flatten)]
    pub base: CIM_DataFile,

/// 
    #[serde(rename = "LogfileName")]
    pub logfile_name: Option<String>,

/// 
    #[serde(rename = "MaxFileSize")]
    pub max_file_size: Option<u32>,

/// 
    #[serde(rename = "NumberOfRecords")]
    pub number_of_records: Option<u32>,

/// 
    #[serde(rename = "OverwriteOutDated")]
    pub overwrite_out_dated: Option<u32>,

/// 
    #[serde(rename = "OverWritePolicy")]
    pub over_write_policy: Option<String>,

/// 
    #[serde(rename = "Sources")]
    pub sources: Vec<String>,
}

impl Win32_NTEventlogFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_DataFile::new(),
            logfile_name: None,
            max_file_size: None,
            number_of_records: None,
            overwrite_out_dated: None,
            over_write_policy: None,
            sources: Vec::new(),
        }
    }


    /// Sets the value of LogfileName
    pub fn set_logfile_name(&mut self, value: String) {
        self.logfile_name = Some(value);
    }

    /// Gets the value of LogfileName
    pub fn get_logfile_name(&self) -> Option<&String> {
        self.logfile_name.as_ref()
    }

    /// Sets the value of MaxFileSize
    pub fn set_max_file_size(&mut self, value: u32) {
        self.max_file_size = Some(value);
    }

    /// Gets the value of MaxFileSize
    pub fn get_max_file_size(&self) -> Option<&u32> {
        self.max_file_size.as_ref()
    }

    /// Sets the value of NumberOfRecords
    pub fn set_number_of_records(&mut self, value: u32) {
        self.number_of_records = Some(value);
    }

    /// Gets the value of NumberOfRecords
    pub fn get_number_of_records(&self) -> Option<&u32> {
        self.number_of_records.as_ref()
    }

    /// Sets the value of OverwriteOutDated
    pub fn set_overwrite_out_dated(&mut self, value: u32) {
        self.overwrite_out_dated = Some(value);
    }

    /// Gets the value of OverwriteOutDated
    pub fn get_overwrite_out_dated(&self) -> Option<&u32> {
        self.overwrite_out_dated.as_ref()
    }

    /// Sets the value of OverWritePolicy
    pub fn set_over_write_policy(&mut self, value: String) {
        self.over_write_policy = Some(value);
    }

    /// Gets the value of OverWritePolicy
    pub fn get_over_write_policy(&self) -> Option<&String> {
        self.over_write_policy.as_ref()
    }

    /// Sets the value of Sources
    pub fn set_sources(&mut self, value: Vec<String>) {
        self.sources = value;
    }

    /// Gets the value of Sources
    pub fn get_sources(&self) -> &Vec<String> {
        &self.sources
    }

/// 

    /// * `archive_file_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn clear_eventlog(&self, archive_file_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ArchiveFileName".to_string(), value: archive_file_name.into() });
        self.invoke_method("ClearEventlog", &args)

    }


/// 

    /// * `archive_file_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn backup_eventlog(&self, archive_file_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ArchiveFileName".to_string(), value: archive_file_name.into() });
        self.invoke_method("BackupEventlog", &args)

    }

}

