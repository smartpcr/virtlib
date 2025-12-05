// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterQosSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterQosSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "CurrentCapabilities")]
    pub current_capabilities: Option<MSFT_NetAdapter_QosCapabilities>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "HardwareCapabilities")]
    pub hardware_capabilities: Option<MSFT_NetAdapter_QosCapabilities>,

/// 
    #[serde(rename = "OperationalSettings")]
    pub operational_settings: Option<MSFT_NetAdapter_QosSettings>,

/// 
    #[serde(rename = "RemoteSettings")]
    pub remote_settings: Option<MSFT_NetAdapter_QosSettings>,
}

impl MSFT_NetAdapterQosSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            current_capabilities: None,
            enabled: None,
            hardware_capabilities: None,
            operational_settings: None,
            remote_settings: None,
        }
    }


    /// Sets the value of CurrentCapabilities
    pub fn set_current_capabilities(&mut self, value: MSFT_NetAdapter_QosCapabilities) {
        self.current_capabilities = Some(value);
    }

    /// Gets the value of CurrentCapabilities
    pub fn get_current_capabilities(&self) -> Option<&MSFT_NetAdapter_QosCapabilities> {
        self.current_capabilities.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of HardwareCapabilities
    pub fn set_hardware_capabilities(&mut self, value: MSFT_NetAdapter_QosCapabilities) {
        self.hardware_capabilities = Some(value);
    }

    /// Gets the value of HardwareCapabilities
    pub fn get_hardware_capabilities(&self) -> Option<&MSFT_NetAdapter_QosCapabilities> {
        self.hardware_capabilities.as_ref()
    }

    /// Sets the value of OperationalSettings
    pub fn set_operational_settings(&mut self, value: MSFT_NetAdapter_QosSettings) {
        self.operational_settings = Some(value);
    }

    /// Gets the value of OperationalSettings
    pub fn get_operational_settings(&self) -> Option<&MSFT_NetAdapter_QosSettings> {
        self.operational_settings.as_ref()
    }

    /// Sets the value of RemoteSettings
    pub fn set_remote_settings(&mut self, value: MSFT_NetAdapter_QosSettings) {
        self.remote_settings = Some(value);
    }

    /// Gets the value of RemoteSettings
    pub fn get_remote_settings(&self) -> Option<&MSFT_NetAdapter_QosSettings> {
        self.remote_settings.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterQosSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapterQosSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterQosSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapterQosSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

