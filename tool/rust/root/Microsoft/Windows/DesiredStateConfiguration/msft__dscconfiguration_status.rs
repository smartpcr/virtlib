// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DSCConfigurationStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DSCConfigurationStatus {

/// 
    #[serde(rename = "DurationInSeconds")]
    pub duration_in_seconds: Option<u32>,

/// 
    #[serde(rename = "Error")]
    pub error: Option<String>,

/// 
    #[serde(rename = "HostName")]
    pub host_name: Option<String>,

/// 
    #[serde(rename = "IPV4Addresses")]
    pub ipv4_addresses: Vec<String>,

/// 
    #[serde(rename = "IPV6Addresses")]
    pub ipv6_addresses: Vec<String>,

/// 
    #[serde(rename = "JobID")]
    pub job_id: Option<String>,

/// 
    #[serde(rename = "LCMVersion")]
    pub lcmversion: Option<String>,

/// 
    #[serde(rename = "Locale")]
    pub locale: Option<String>,

/// 
    #[serde(rename = "MACAddresses")]
    pub macaddresses: Vec<String>,

/// 
    #[serde(rename = "MetaConfiguration")]
    pub meta_configuration: Option<MSFT_DSCMetaConfiguration>,

/// 
    #[serde(rename = "MetaData")]
    pub meta_data: Option<String>,

/// 
    #[serde(rename = "Mode")]
    pub mode: Option<String>,

/// 
    #[serde(rename = "NumberOfResources")]
    pub number_of_resources: Option<u32>,

/// 
    #[serde(rename = "RebootRequested")]
    pub reboot_requested: Option<bool>,

/// 
    #[serde(rename = "ResourcesChanged")]
    pub resources_changed: Vec<MSFT_ResourceChanged>,

/// 
    #[serde(rename = "ResourcesInDesiredState")]
    pub resources_in_desired_state: Vec<MSFT_ResourceInDesiredState>,

/// 
    #[serde(rename = "ResourcesNotInDesiredState")]
    pub resources_not_in_desired_state: Vec<MSFT_ResourceNotInDesiredState>,

/// 
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl MSFT_DSCConfigurationStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            duration_in_seconds: None,
            error: None,
            host_name: None,
            ipv4_addresses: Vec::new(),
            ipv6_addresses: Vec::new(),
            job_id: None,
            lcmversion: None,
            locale: None,
            macaddresses: Vec::new(),
            meta_configuration: None,
            meta_data: None,
            mode: None,
            number_of_resources: None,
            reboot_requested: None,
            resources_changed: Vec::new(),
            resources_in_desired_state: Vec::new(),
            resources_not_in_desired_state: Vec::new(),
            start_date: None,
            status: None,
            type: None,
        }
    }


    /// Sets the value of DurationInSeconds
    pub fn set_duration_in_seconds(&mut self, value: u32) {
        self.duration_in_seconds = Some(value);
    }

    /// Gets the value of DurationInSeconds
    pub fn get_duration_in_seconds(&self) -> Option<&u32> {
        self.duration_in_seconds.as_ref()
    }

    /// Sets the value of Error
    pub fn set_error(&mut self, value: String) {
        self.error = Some(value);
    }

    /// Gets the value of Error
    pub fn get_error(&self) -> Option<&String> {
        self.error.as_ref()
    }

    /// Sets the value of HostName
    pub fn set_host_name(&mut self, value: String) {
        self.host_name = Some(value);
    }

    /// Gets the value of HostName
    pub fn get_host_name(&self) -> Option<&String> {
        self.host_name.as_ref()
    }

    /// Sets the value of IPV4Addresses
    pub fn set_ipv4_addresses(&mut self, value: Vec<String>) {
        self.ipv4_addresses = value;
    }

    /// Gets the value of IPV4Addresses
    pub fn get_ipv4_addresses(&self) -> &Vec<String> {
        &self.ipv4_addresses
    }

    /// Sets the value of IPV6Addresses
    pub fn set_ipv6_addresses(&mut self, value: Vec<String>) {
        self.ipv6_addresses = value;
    }

    /// Gets the value of IPV6Addresses
    pub fn get_ipv6_addresses(&self) -> &Vec<String> {
        &self.ipv6_addresses
    }

    /// Sets the value of JobID
    pub fn set_job_id(&mut self, value: String) {
        self.job_id = Some(value);
    }

    /// Gets the value of JobID
    pub fn get_job_id(&self) -> Option<&String> {
        self.job_id.as_ref()
    }

    /// Sets the value of LCMVersion
    pub fn set_lcmversion(&mut self, value: String) {
        self.lcmversion = Some(value);
    }

    /// Gets the value of LCMVersion
    pub fn get_lcmversion(&self) -> Option<&String> {
        self.lcmversion.as_ref()
    }

    /// Sets the value of Locale
    pub fn set_locale(&mut self, value: String) {
        self.locale = Some(value);
    }

    /// Gets the value of Locale
    pub fn get_locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }

    /// Sets the value of MACAddresses
    pub fn set_macaddresses(&mut self, value: Vec<String>) {
        self.macaddresses = value;
    }

    /// Gets the value of MACAddresses
    pub fn get_macaddresses(&self) -> &Vec<String> {
        &self.macaddresses
    }

    /// Sets the value of MetaConfiguration
    pub fn set_meta_configuration(&mut self, value: MSFT_DSCMetaConfiguration) {
        self.meta_configuration = Some(value);
    }

    /// Gets the value of MetaConfiguration
    pub fn get_meta_configuration(&self) -> Option<&MSFT_DSCMetaConfiguration> {
        self.meta_configuration.as_ref()
    }

    /// Sets the value of MetaData
    pub fn set_meta_data(&mut self, value: String) {
        self.meta_data = Some(value);
    }

    /// Gets the value of MetaData
    pub fn get_meta_data(&self) -> Option<&String> {
        self.meta_data.as_ref()
    }

    /// Sets the value of Mode
    pub fn set_mode(&mut self, value: String) {
        self.mode = Some(value);
    }

    /// Gets the value of Mode
    pub fn get_mode(&self) -> Option<&String> {
        self.mode.as_ref()
    }

    /// Sets the value of NumberOfResources
    pub fn set_number_of_resources(&mut self, value: u32) {
        self.number_of_resources = Some(value);
    }

    /// Gets the value of NumberOfResources
    pub fn get_number_of_resources(&self) -> Option<&u32> {
        self.number_of_resources.as_ref()
    }

    /// Sets the value of RebootRequested
    pub fn set_reboot_requested(&mut self, value: bool) {
        self.reboot_requested = Some(value);
    }

    /// Gets the value of RebootRequested
    pub fn get_reboot_requested(&self) -> Option<&bool> {
        self.reboot_requested.as_ref()
    }

    /// Sets the value of ResourcesChanged
    pub fn set_resources_changed(&mut self, value: Vec<MSFT_ResourceChanged>) {
        self.resources_changed = value;
    }

    /// Gets the value of ResourcesChanged
    pub fn get_resources_changed(&self) -> &Vec<MSFT_ResourceChanged> {
        &self.resources_changed
    }

    /// Sets the value of ResourcesInDesiredState
    pub fn set_resources_in_desired_state(&mut self, value: Vec<MSFT_ResourceInDesiredState>) {
        self.resources_in_desired_state = value;
    }

    /// Gets the value of ResourcesInDesiredState
    pub fn get_resources_in_desired_state(&self) -> &Vec<MSFT_ResourceInDesiredState> {
        &self.resources_in_desired_state
    }

    /// Sets the value of ResourcesNotInDesiredState
    pub fn set_resources_not_in_desired_state(&mut self, value: Vec<MSFT_ResourceNotInDesiredState>) {
        self.resources_not_in_desired_state = value;
    }

    /// Gets the value of ResourcesNotInDesiredState
    pub fn get_resources_not_in_desired_state(&self) -> &Vec<MSFT_ResourceNotInDesiredState> {
        &self.resources_not_in_desired_state
    }

    /// Sets the value of StartDate
    pub fn set_start_date(&mut self, value: String) {
        self.start_date = Some(value);
    }

    /// Gets the value of StartDate
    pub fn get_start_date(&self) -> Option<&String> {
        self.start_date.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: String) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }
}

