// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiTcpLargeSendOffloadV2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiTcpLargeSendOffloadV2 {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "WmiIPv4")]
    pub wmi_ipv4: Option<MSNdis_WmiTcpLargeSendOffloadV2_IPv4>,

/// 
    #[serde(rename = "WmiIPv6")]
    pub wmi_ipv6: Option<MSNdis_WmiTcpLargeSendOffloadV2_IPv6>,
}

impl MSNdis_WmiTcpLargeSendOffloadV2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            wmi_ipv4: None,
            wmi_ipv6: None,
        }
    }


    /// Sets the value of WmiIPv4
    pub fn set_wmi_ipv4(&mut self, value: MSNdis_WmiTcpLargeSendOffloadV2_IPv4) {
        self.wmi_ipv4 = Some(value);
    }

    /// Gets the value of WmiIPv4
    pub fn get_wmi_ipv4(&self) -> Option<&MSNdis_WmiTcpLargeSendOffloadV2_IPv4> {
        self.wmi_ipv4.as_ref()
    }

    /// Sets the value of WmiIPv6
    pub fn set_wmi_ipv6(&mut self, value: MSNdis_WmiTcpLargeSendOffloadV2_IPv6) {
        self.wmi_ipv6 = Some(value);
    }

    /// Gets the value of WmiIPv6
    pub fn get_wmi_ipv6(&self) -> Option<&MSNdis_WmiTcpLargeSendOffloadV2_IPv6> {
        self.wmi_ipv6.as_ref()
    }
}

