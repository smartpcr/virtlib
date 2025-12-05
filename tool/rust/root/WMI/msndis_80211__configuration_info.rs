// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_80211_ConfigurationInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_80211_ConfigurationInfo {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "ATIMWindow")]
    pub atimwindow: Option<u32>,

/// 
    #[serde(rename = "BeaconPeriod")]
    pub beacon_period: Option<u32>,

/// 
    #[serde(rename = "ConfigLength")]
    pub config_length: Option<u32>,

/// 
    #[serde(rename = "DSConfig")]
    pub dsconfig: Option<u32>,

/// 
    #[serde(rename = "FHConfig")]
    pub fhconfig: Option<MSNdis_80211_ConfigurationFH>,
}

impl MSNdis_80211_ConfigurationInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            atimwindow: None,
            beacon_period: None,
            config_length: None,
            dsconfig: None,
            fhconfig: None,
        }
    }


    /// Sets the value of ATIMWindow
    pub fn set_atimwindow(&mut self, value: u32) {
        self.atimwindow = Some(value);
    }

    /// Gets the value of ATIMWindow
    pub fn get_atimwindow(&self) -> Option<&u32> {
        self.atimwindow.as_ref()
    }

    /// Sets the value of BeaconPeriod
    pub fn set_beacon_period(&mut self, value: u32) {
        self.beacon_period = Some(value);
    }

    /// Gets the value of BeaconPeriod
    pub fn get_beacon_period(&self) -> Option<&u32> {
        self.beacon_period.as_ref()
    }

    /// Sets the value of ConfigLength
    pub fn set_config_length(&mut self, value: u32) {
        self.config_length = Some(value);
    }

    /// Gets the value of ConfigLength
    pub fn get_config_length(&self) -> Option<&u32> {
        self.config_length.as_ref()
    }

    /// Sets the value of DSConfig
    pub fn set_dsconfig(&mut self, value: u32) {
        self.dsconfig = Some(value);
    }

    /// Gets the value of DSConfig
    pub fn get_dsconfig(&self) -> Option<&u32> {
        self.dsconfig.as_ref()
    }

    /// Sets the value of FHConfig
    pub fn set_fhconfig(&mut self, value: MSNdis_80211_ConfigurationFH) {
        self.fhconfig = Some(value);
    }

    /// Gets the value of FHConfig
    pub fn get_fhconfig(&self) -> Option<&MSNdis_80211_ConfigurationFH> {
        self.fhconfig.as_ref()
    }
}

