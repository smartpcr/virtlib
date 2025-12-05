// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AcpiControlStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AcpiControlStatus {

/// 
    #[serde(rename = "Control")]
    pub control: Option<AcpiGenAddr>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<AcpiGenAddr>,
}

impl AcpiControlStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            control: None,
            status: None,
        }
    }


    /// Sets the value of Control
    pub fn set_control(&mut self, value: AcpiGenAddr) {
        self.control = Some(value);
    }

    /// Gets the value of Control
    pub fn get_control(&self) -> Option<&AcpiGenAddr> {
        self.control.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: AcpiGenAddr) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&AcpiGenAddr> {
        self.status.as_ref()
    }
}

