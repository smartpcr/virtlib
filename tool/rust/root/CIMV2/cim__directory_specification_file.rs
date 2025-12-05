// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DirectorySpecificationFile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DirectorySpecificationFile {

/// 
    #[serde(rename = "DirectorySpecification")]
    pub directory_specification: Option<CIM_DirectorySpecification>,

/// 
    #[serde(rename = "FileSpecification")]
    pub file_specification: Option<CIM_FileSpecification>,
}

impl CIM_DirectorySpecificationFile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            directory_specification: None,
            file_specification: None,
        }
    }


    /// Sets the value of DirectorySpecification
    pub fn set_directory_specification(&mut self, value: CIM_DirectorySpecification) {
        self.directory_specification = Some(value);
    }

    /// Gets the value of DirectorySpecification
    pub fn get_directory_specification(&self) -> Option<&CIM_DirectorySpecification> {
        self.directory_specification.as_ref()
    }

    /// Sets the value of FileSpecification
    pub fn set_file_specification(&mut self, value: CIM_FileSpecification) {
        self.file_specification = Some(value);
    }

    /// Gets the value of FileSpecification
    pub fn get_file_specification(&self) -> Option<&CIM_FileSpecification> {
        self.file_specification.as_ref()
    }
}

