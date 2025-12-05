// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_InstalledProgramFramework struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_InstalledProgramFramework {

/// 
    #[serde(rename = "FrameworkName")]
    pub framework_name: Option<String>,

/// 
    #[serde(rename = "FrameworkPublisher")]
    pub framework_publisher: Option<String>,

/// 
    #[serde(rename = "FrameworkVersion")]
    pub framework_version: Option<String>,

/// 
    #[serde(rename = "FrameworkVersionActual")]
    pub framework_version_actual: Option<String>,

/// 
    #[serde(rename = "IsPrivate")]
    pub is_private: Option<bool>,

/// 
    #[serde(rename = "ProgramId")]
    pub program_id: Option<String>,
}

impl Win32_InstalledProgramFramework {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            framework_name: None,
            framework_publisher: None,
            framework_version: None,
            framework_version_actual: None,
            is_private: None,
            program_id: None,
        }
    }


    /// Sets the value of FrameworkName
    pub fn set_framework_name(&mut self, value: String) {
        self.framework_name = Some(value);
    }

    /// Gets the value of FrameworkName
    pub fn get_framework_name(&self) -> Option<&String> {
        self.framework_name.as_ref()
    }

    /// Sets the value of FrameworkPublisher
    pub fn set_framework_publisher(&mut self, value: String) {
        self.framework_publisher = Some(value);
    }

    /// Gets the value of FrameworkPublisher
    pub fn get_framework_publisher(&self) -> Option<&String> {
        self.framework_publisher.as_ref()
    }

    /// Sets the value of FrameworkVersion
    pub fn set_framework_version(&mut self, value: String) {
        self.framework_version = Some(value);
    }

    /// Gets the value of FrameworkVersion
    pub fn get_framework_version(&self) -> Option<&String> {
        self.framework_version.as_ref()
    }

    /// Sets the value of FrameworkVersionActual
    pub fn set_framework_version_actual(&mut self, value: String) {
        self.framework_version_actual = Some(value);
    }

    /// Gets the value of FrameworkVersionActual
    pub fn get_framework_version_actual(&self) -> Option<&String> {
        self.framework_version_actual.as_ref()
    }

    /// Sets the value of IsPrivate
    pub fn set_is_private(&mut self, value: bool) {
        self.is_private = Some(value);
    }

    /// Gets the value of IsPrivate
    pub fn get_is_private(&self) -> Option<&bool> {
        self.is_private.as_ref()
    }

    /// Sets the value of ProgramId
    pub fn set_program_id(&mut self, value: String) {
        self.program_id = Some(value);
    }

    /// Gets the value of ProgramId
    pub fn get_program_id(&self) -> Option<&String> {
        self.program_id.as_ref()
    }
}

