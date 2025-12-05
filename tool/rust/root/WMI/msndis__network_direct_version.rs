// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_NetworkDirectVersion struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_NetworkDirectVersion {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "MajorVersionNumber")]
    pub major_version_number: Option<u16>,

/// 
    #[serde(rename = "MinorVersionNumber")]
    pub minor_version_number: Option<u16>,
}

impl MSNdis_NetworkDirectVersion {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            major_version_number: None,
            minor_version_number: None,
        }
    }


    /// Sets the value of MajorVersionNumber
    pub fn set_major_version_number(&mut self, value: u16) {
        self.major_version_number = Some(value);
    }

    /// Gets the value of MajorVersionNumber
    pub fn get_major_version_number(&self) -> Option<&u16> {
        self.major_version_number.as_ref()
    }

    /// Sets the value of MinorVersionNumber
    pub fn set_minor_version_number(&mut self, value: u16) {
        self.minor_version_number = Some(value);
    }

    /// Gets the value of MinorVersionNumber
    pub fn get_minor_version_number(&self) -> Option<&u16> {
        self.minor_version_number.as_ref()
    }
}

