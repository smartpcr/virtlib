// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DOUsage struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DOUsage {
    #[serde(flatten)]
    pub base: MSFT_DOBaseStatus,

/// 
    #[serde(rename = "LinkBps")]
    pub link_bps: Option<u32>,

/// 
    #[serde(rename = "LinkUsageBps")]
    pub link_usage_bps: Option<u32>,

/// 
    #[serde(rename = "MonthlyGroupBytes")]
    pub monthly_group_bytes: Option<u64>,

/// 
    #[serde(rename = "MonthlyInternetBytes")]
    pub monthly_internet_bytes: Option<u64>,

/// 
    #[serde(rename = "MonthlyLanBytes")]
    pub monthly_lan_bytes: Option<u64>,

/// 
    #[serde(rename = "MonthlyLinkLocalBytes")]
    pub monthly_link_local_bytes: Option<u64>,
}

impl MSFT_DOUsage {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_DOBaseStatus::new(),
            link_bps: None,
            link_usage_bps: None,
            monthly_group_bytes: None,
            monthly_internet_bytes: None,
            monthly_lan_bytes: None,
            monthly_link_local_bytes: None,
        }
    }


    /// Sets the value of LinkBps
    pub fn set_link_bps(&mut self, value: u32) {
        self.link_bps = Some(value);
    }

    /// Gets the value of LinkBps
    pub fn get_link_bps(&self) -> Option<&u32> {
        self.link_bps.as_ref()
    }

    /// Sets the value of LinkUsageBps
    pub fn set_link_usage_bps(&mut self, value: u32) {
        self.link_usage_bps = Some(value);
    }

    /// Gets the value of LinkUsageBps
    pub fn get_link_usage_bps(&self) -> Option<&u32> {
        self.link_usage_bps.as_ref()
    }

    /// Sets the value of MonthlyGroupBytes
    pub fn set_monthly_group_bytes(&mut self, value: u64) {
        self.monthly_group_bytes = Some(value);
    }

    /// Gets the value of MonthlyGroupBytes
    pub fn get_monthly_group_bytes(&self) -> Option<&u64> {
        self.monthly_group_bytes.as_ref()
    }

    /// Sets the value of MonthlyInternetBytes
    pub fn set_monthly_internet_bytes(&mut self, value: u64) {
        self.monthly_internet_bytes = Some(value);
    }

    /// Gets the value of MonthlyInternetBytes
    pub fn get_monthly_internet_bytes(&self) -> Option<&u64> {
        self.monthly_internet_bytes.as_ref()
    }

    /// Sets the value of MonthlyLanBytes
    pub fn set_monthly_lan_bytes(&mut self, value: u64) {
        self.monthly_lan_bytes = Some(value);
    }

    /// Gets the value of MonthlyLanBytes
    pub fn get_monthly_lan_bytes(&self) -> Option<&u64> {
        self.monthly_lan_bytes.as_ref()
    }

    /// Sets the value of MonthlyLinkLocalBytes
    pub fn set_monthly_link_local_bytes(&mut self, value: u64) {
        self.monthly_link_local_bytes = Some(value);
    }

    /// Gets the value of MonthlyLinkLocalBytes
    pub fn get_monthly_link_local_bytes(&self) -> Option<&u64> {
        self.monthly_link_local_bytes.as_ref()
    }
}

