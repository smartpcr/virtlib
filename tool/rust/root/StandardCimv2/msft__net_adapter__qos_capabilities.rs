// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_QosCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_QosCapabilities {

/// 
    #[serde(rename = "CeeDcbxSupported")]
    pub cee_dcbx_supported: Option<bool>,

/// 
    #[serde(rename = "IeeeDcbxSupported")]
    pub ieee_dcbx_supported: Option<bool>,

/// 
    #[serde(rename = "MacSecBypassSupported")]
    pub mac_sec_bypass_supported: Option<bool>,

/// 
    #[serde(rename = "NumberOfEtsCapableTrafficClasses")]
    pub number_of_ets_capable_traffic_classes: Option<u8>,

/// 
    #[serde(rename = "NumberOfPfcEnabledTrafficClasses")]
    pub number_of_pfc_enabled_traffic_classes: Option<u8>,

/// 
    #[serde(rename = "NumberOfTrafficClasses")]
    pub number_of_traffic_classes: Option<u8>,
}

impl MSFT_NetAdapter_QosCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cee_dcbx_supported: None,
            ieee_dcbx_supported: None,
            mac_sec_bypass_supported: None,
            number_of_ets_capable_traffic_classes: None,
            number_of_pfc_enabled_traffic_classes: None,
            number_of_traffic_classes: None,
        }
    }


    /// Sets the value of CeeDcbxSupported
    pub fn set_cee_dcbx_supported(&mut self, value: bool) {
        self.cee_dcbx_supported = Some(value);
    }

    /// Gets the value of CeeDcbxSupported
    pub fn get_cee_dcbx_supported(&self) -> Option<&bool> {
        self.cee_dcbx_supported.as_ref()
    }

    /// Sets the value of IeeeDcbxSupported
    pub fn set_ieee_dcbx_supported(&mut self, value: bool) {
        self.ieee_dcbx_supported = Some(value);
    }

    /// Gets the value of IeeeDcbxSupported
    pub fn get_ieee_dcbx_supported(&self) -> Option<&bool> {
        self.ieee_dcbx_supported.as_ref()
    }

    /// Sets the value of MacSecBypassSupported
    pub fn set_mac_sec_bypass_supported(&mut self, value: bool) {
        self.mac_sec_bypass_supported = Some(value);
    }

    /// Gets the value of MacSecBypassSupported
    pub fn get_mac_sec_bypass_supported(&self) -> Option<&bool> {
        self.mac_sec_bypass_supported.as_ref()
    }

    /// Sets the value of NumberOfEtsCapableTrafficClasses
    pub fn set_number_of_ets_capable_traffic_classes(&mut self, value: u8) {
        self.number_of_ets_capable_traffic_classes = Some(value);
    }

    /// Gets the value of NumberOfEtsCapableTrafficClasses
    pub fn get_number_of_ets_capable_traffic_classes(&self) -> Option<&u8> {
        self.number_of_ets_capable_traffic_classes.as_ref()
    }

    /// Sets the value of NumberOfPfcEnabledTrafficClasses
    pub fn set_number_of_pfc_enabled_traffic_classes(&mut self, value: u8) {
        self.number_of_pfc_enabled_traffic_classes = Some(value);
    }

    /// Gets the value of NumberOfPfcEnabledTrafficClasses
    pub fn get_number_of_pfc_enabled_traffic_classes(&self) -> Option<&u8> {
        self.number_of_pfc_enabled_traffic_classes.as_ref()
    }

    /// Sets the value of NumberOfTrafficClasses
    pub fn set_number_of_traffic_classes(&mut self, value: u8) {
        self.number_of_traffic_classes = Some(value);
    }

    /// Gets the value of NumberOfTrafficClasses
    pub fn get_number_of_traffic_classes(&self) -> Option<&u8> {
        self.number_of_traffic_classes.as_ref()
    }
}

