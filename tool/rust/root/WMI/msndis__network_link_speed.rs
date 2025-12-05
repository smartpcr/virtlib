// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_NetworkLinkSpeed struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_NetworkLinkSpeed {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Inbound")]
    pub inbound: Option<u32>,

/// 
    #[serde(rename = "Outbound")]
    pub outbound: Option<u32>,
}

impl MSNdis_NetworkLinkSpeed {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            inbound: None,
            outbound: None,
        }
    }


    /// Sets the value of Inbound
    pub fn set_inbound(&mut self, value: u32) {
        self.inbound = Some(value);
    }

    /// Gets the value of Inbound
    pub fn get_inbound(&self) -> Option<&u32> {
        self.inbound.as_ref()
    }

    /// Sets the value of Outbound
    pub fn set_outbound(&mut self, value: u32) {
        self.outbound = Some(value);
    }

    /// Gets the value of Outbound
    pub fn get_outbound(&self) -> Option<&u32> {
        self.outbound.as_ref()
    }
}

