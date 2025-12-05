// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ProtocolBinding struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ProtocolBinding {

/// 
    #[serde(rename = "Antecedent")]
    pub antecedent: Option<Win32_NetworkProtocol>,

/// 
    #[serde(rename = "Dependent")]
    pub dependent: Option<Win32_SystemDriver>,

/// 
    #[serde(rename = "Device")]
    pub device: Option<Win32_NetworkAdapter>,
}

impl Win32_ProtocolBinding {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            antecedent: None,
            dependent: None,
            device: None,
        }
    }


    /// Sets the value of Antecedent
    pub fn set_antecedent(&mut self, value: Win32_NetworkProtocol) {
        self.antecedent = Some(value);
    }

    /// Gets the value of Antecedent
    pub fn get_antecedent(&self) -> Option<&Win32_NetworkProtocol> {
        self.antecedent.as_ref()
    }

    /// Sets the value of Dependent
    pub fn set_dependent(&mut self, value: Win32_SystemDriver) {
        self.dependent = Some(value);
    }

    /// Gets the value of Dependent
    pub fn get_dependent(&self) -> Option<&Win32_SystemDriver> {
        self.dependent.as_ref()
    }

    /// Sets the value of Device
    pub fn set_device(&mut self, value: Win32_NetworkAdapter) {
        self.device = Some(value);
    }

    /// Gets the value of Device
    pub fn get_device(&self) -> Option<&Win32_NetworkAdapter> {
        self.device.as_ref()
    }
}

