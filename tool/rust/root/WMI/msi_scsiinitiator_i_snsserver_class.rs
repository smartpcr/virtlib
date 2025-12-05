// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_iSNSServerClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_iSNSServerClass {

/// 
    #[serde(rename = "iSNSServerAddress")]
    pub i_snsserver_address: Option<String>,
}

impl MSiSCSIInitiator_iSNSServerClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_snsserver_address: None,
        }
    }


    /// Sets the value of iSNSServerAddress
    pub fn set_i_snsserver_address(&mut self, value: String) {
        self.i_snsserver_address = Some(value);
    }

    /// Gets the value of iSNSServerAddress
    pub fn get_i_snsserver_address(&self) -> Option<&String> {
        self.i_snsserver_address.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn refresh(&self) -> Result<(), WmiError> {
        self.invoke_method("Refresh", &[])

    }

}

