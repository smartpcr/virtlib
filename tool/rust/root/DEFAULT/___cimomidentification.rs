// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.DEFAULT
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __CIMOMIdentification struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __CIMOMIdentification {
    #[serde(flatten)]
    pub base: __SystemClass,

/// 
    #[serde(rename = "SetupDateTime")]
    pub setup_date_time: Option<String>,

/// 
    #[serde(rename = "VersionCurrentlyRunning")]
    pub version_currently_running: Option<String>,

/// 
    #[serde(rename = "VersionUsedToCreateDB")]
    pub version_used_to_create_db: Option<String>,

/// 
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,
}

impl __CIMOMIdentification {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __SystemClass::new(),
            setup_date_time: None,
            version_currently_running: None,
            version_used_to_create_db: None,
            working_directory: None,
        }
    }


    /// Sets the value of SetupDateTime
    pub fn set_setup_date_time(&mut self, value: String) {
        self.setup_date_time = Some(value);
    }

    /// Gets the value of SetupDateTime
    pub fn get_setup_date_time(&self) -> Option<&String> {
        self.setup_date_time.as_ref()
    }

    /// Sets the value of VersionCurrentlyRunning
    pub fn set_version_currently_running(&mut self, value: String) {
        self.version_currently_running = Some(value);
    }

    /// Gets the value of VersionCurrentlyRunning
    pub fn get_version_currently_running(&self) -> Option<&String> {
        self.version_currently_running.as_ref()
    }

    /// Sets the value of VersionUsedToCreateDB
    pub fn set_version_used_to_create_db(&mut self, value: String) {
        self.version_used_to_create_db = Some(value);
    }

    /// Gets the value of VersionUsedToCreateDB
    pub fn get_version_used_to_create_db(&self) -> Option<&String> {
        self.version_used_to_create_db.as_ref()
    }

    /// Sets the value of WorkingDirectory
    pub fn set_working_directory(&mut self, value: String) {
        self.working_directory = Some(value);
    }

    /// Gets the value of WorkingDirectory
    pub fn get_working_directory(&self) -> Option<&String> {
        self.working_directory.as_ref()
    }
}

