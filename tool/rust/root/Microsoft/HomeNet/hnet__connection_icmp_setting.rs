// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.HomeNet
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// HNet_ConnectionIcmpSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HNet_ConnectionIcmpSetting {

/// 
    #[serde(rename = "Connection")]
    pub connection: Option<HNet_Connection>,

/// 
    #[serde(rename = "IcmpSettings")]
    pub icmp_settings: Option<HNet_FwIcmpSettings>,
}

impl HNet_ConnectionIcmpSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection: None,
            icmp_settings: None,
        }
    }


    /// Sets the value of Connection
    pub fn set_connection(&mut self, value: HNet_Connection) {
        self.connection = Some(value);
    }

    /// Gets the value of Connection
    pub fn get_connection(&self) -> Option<&HNet_Connection> {
        self.connection.as_ref()
    }

    /// Sets the value of IcmpSettings
    pub fn set_icmp_settings(&mut self, value: HNet_FwIcmpSettings) {
        self.icmp_settings = Some(value);
    }

    /// Gets the value of IcmpSettings
    pub fn get_icmp_settings(&self) -> Option<&HNet_FwIcmpSettings> {
        self.icmp_settings.as_ref()
    }
}

