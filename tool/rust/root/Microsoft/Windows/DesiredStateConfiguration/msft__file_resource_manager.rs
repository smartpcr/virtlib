// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileResourceManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileResourceManager {
    #[serde(flatten)]
    pub base: OMI_ResourceModuleManager,

/// 
    #[serde(rename = "Credential")]
    pub credential: Option<MSFT_Credential>,

/// 
    #[serde(rename = "SourcePath")]
    pub source_path: Option<String>,
}

impl MSFT_FileResourceManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: OMI_ResourceModuleManager::new(),
            credential: None,
            source_path: None,
        }
    }


    /// Sets the value of Credential
    pub fn set_credential(&mut self, value: MSFT_Credential) {
        self.credential = Some(value);
    }

    /// Gets the value of Credential
    pub fn get_credential(&self) -> Option<&MSFT_Credential> {
        self.credential.as_ref()
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

