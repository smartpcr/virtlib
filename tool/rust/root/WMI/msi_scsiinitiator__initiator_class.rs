// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSIInitiator_InitiatorClass struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSIInitiator_InitiatorClass {

/// 
    #[serde(rename = "InitiatorName")]
    pub initiator_name: Option<String>,
}

impl MSiSCSIInitiator_InitiatorClass {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_name: None,
        }
    }


    /// Sets the value of InitiatorName
    pub fn set_initiator_name(&mut self, value: String) {
        self.initiator_name = Some(value);
    }

    /// Gets the value of InitiatorName
    pub fn get_initiator_name(&self) -> Option<&String> {
        self.initiator_name.as_ref()
    }
}

