// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.SDDC.Management
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SDDC_VirtualSwitch struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SDDC_VirtualSwitch {

/// 
    #[serde(rename = "AllowManagementOS")]
    pub allow_management_os: Option<bool>,

/// 
    #[serde(rename = "BandwidthReservationMode")]
    pub bandwidth_reservation_mode: Option<u32>,

/// 
    #[serde(rename = "Extensions")]
    pub extensions: Vec<String>,

/// 
    #[serde(rename = "HostName")]
    pub host_name: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "IovSupportReasons")]
    pub iov_support_reasons: Vec<String>,

/// 
    #[serde(rename = "IsIovCapable")]
    pub is_iov_capable: Option<bool>,

/// 
    #[serde(rename = "IsIovEnabled")]
    pub is_iov_enabled: Option<bool>,

/// 
    #[serde(rename = "IsTeamingEnabled")]
    pub is_teaming_enabled: Option<bool>,

/// 
    #[serde(rename = "LoadBalancingAlgorithm")]
    pub load_balancing_algorithm: Option<u32>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "NetAdapterDescriptions")]
    pub net_adapter_descriptions: Vec<String>,

/// 
    #[serde(rename = "Notes")]
    pub notes: Option<String>,

/// 
    #[serde(rename = "SwitchType")]
    pub switch_type: Option<u16>,

/// 
    #[serde(rename = "TeamingMode")]
    pub teaming_mode: Option<u32>,
}

impl SDDC_VirtualSwitch {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_management_os: None,
            bandwidth_reservation_mode: None,
            extensions: Vec::new(),
            host_name: None,
            id: None,
            iov_support_reasons: Vec::new(),
            is_iov_capable: None,
            is_iov_enabled: None,
            is_teaming_enabled: None,
            load_balancing_algorithm: None,
            name: None,
            net_adapter_descriptions: Vec::new(),
            notes: None,
            switch_type: None,
            teaming_mode: None,
        }
    }


    /// Sets the value of AllowManagementOS
    pub fn set_allow_management_os(&mut self, value: bool) {
        self.allow_management_os = Some(value);
    }

    /// Gets the value of AllowManagementOS
    pub fn get_allow_management_os(&self) -> Option<&bool> {
        self.allow_management_os.as_ref()
    }

    /// Sets the value of BandwidthReservationMode
    pub fn set_bandwidth_reservation_mode(&mut self, value: u32) {
        self.bandwidth_reservation_mode = Some(value);
    }

    /// Gets the value of BandwidthReservationMode
    pub fn get_bandwidth_reservation_mode(&self) -> Option<&u32> {
        self.bandwidth_reservation_mode.as_ref()
    }

    /// Sets the value of Extensions
    pub fn set_extensions(&mut self, value: Vec<String>) {
        self.extensions = value;
    }

    /// Gets the value of Extensions
    pub fn get_extensions(&self) -> &Vec<String> {
        &self.extensions
    }

    /// Sets the value of HostName
    pub fn set_host_name(&mut self, value: String) {
        self.host_name = Some(value);
    }

    /// Gets the value of HostName
    pub fn get_host_name(&self) -> Option<&String> {
        self.host_name.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of IovSupportReasons
    pub fn set_iov_support_reasons(&mut self, value: Vec<String>) {
        self.iov_support_reasons = value;
    }

    /// Gets the value of IovSupportReasons
    pub fn get_iov_support_reasons(&self) -> &Vec<String> {
        &self.iov_support_reasons
    }

    /// Sets the value of IsIovCapable
    pub fn set_is_iov_capable(&mut self, value: bool) {
        self.is_iov_capable = Some(value);
    }

    /// Gets the value of IsIovCapable
    pub fn get_is_iov_capable(&self) -> Option<&bool> {
        self.is_iov_capable.as_ref()
    }

    /// Sets the value of IsIovEnabled
    pub fn set_is_iov_enabled(&mut self, value: bool) {
        self.is_iov_enabled = Some(value);
    }

    /// Gets the value of IsIovEnabled
    pub fn get_is_iov_enabled(&self) -> Option<&bool> {
        self.is_iov_enabled.as_ref()
    }

    /// Sets the value of IsTeamingEnabled
    pub fn set_is_teaming_enabled(&mut self, value: bool) {
        self.is_teaming_enabled = Some(value);
    }

    /// Gets the value of IsTeamingEnabled
    pub fn get_is_teaming_enabled(&self) -> Option<&bool> {
        self.is_teaming_enabled.as_ref()
    }

    /// Sets the value of LoadBalancingAlgorithm
    pub fn set_load_balancing_algorithm(&mut self, value: u32) {
        self.load_balancing_algorithm = Some(value);
    }

    /// Gets the value of LoadBalancingAlgorithm
    pub fn get_load_balancing_algorithm(&self) -> Option<&u32> {
        self.load_balancing_algorithm.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of NetAdapterDescriptions
    pub fn set_net_adapter_descriptions(&mut self, value: Vec<String>) {
        self.net_adapter_descriptions = value;
    }

    /// Gets the value of NetAdapterDescriptions
    pub fn get_net_adapter_descriptions(&self) -> &Vec<String> {
        &self.net_adapter_descriptions
    }

    /// Sets the value of Notes
    pub fn set_notes(&mut self, value: String) {
        self.notes = Some(value);
    }

    /// Gets the value of Notes
    pub fn get_notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    /// Sets the value of SwitchType
    pub fn set_switch_type(&mut self, value: u16) {
        self.switch_type = Some(value);
    }

    /// Gets the value of SwitchType
    pub fn get_switch_type(&self) -> Option<&u16> {
        self.switch_type.as_ref()
    }

    /// Sets the value of TeamingMode
    pub fn set_teaming_mode(&mut self, value: u32) {
        self.teaming_mode = Some(value);
    }

    /// Gets the value of TeamingMode
    pub fn get_teaming_mode(&self) -> Option<&u32> {
        self.teaming_mode.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn refresh(&self) -> Result<(), WmiError> {
        self.invoke_method("Refresh", &[])

    }

}

