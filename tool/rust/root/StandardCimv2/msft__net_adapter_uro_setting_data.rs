// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterUroSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterUroSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "FailureReason")]
    pub failure_reason: Option<u32>,

/// 
    #[serde(rename = "Operational")]
    pub operational: Option<bool>,

/// 
    #[serde(rename = "UroHardwareCapabilities")]
    pub uro_hardware_capabilities: Option<MSFT_NetAdapterUroCapabilities>,
}

impl MSFT_NetAdapterUroSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            enabled: None,
            failure_reason: None,
            operational: None,
            uro_hardware_capabilities: None,
        }
    }


    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of FailureReason
    pub fn set_failure_reason(&mut self, value: u32) {
        self.failure_reason = Some(value);
    }

    /// Gets the value of FailureReason
    pub fn get_failure_reason(&self) -> Option<&u32> {
        self.failure_reason.as_ref()
    }

    /// Sets the value of Operational
    pub fn set_operational(&mut self, value: bool) {
        self.operational = Some(value);
    }

    /// Gets the value of Operational
    pub fn get_operational(&self) -> Option<&bool> {
        self.operational.as_ref()
    }

    /// Sets the value of UroHardwareCapabilities
    pub fn set_uro_hardware_capabilities(&mut self, value: MSFT_NetAdapterUroCapabilities) {
        self.uro_hardware_capabilities = Some(value);
    }

    /// Gets the value of UroHardwareCapabilities
    pub fn get_uro_hardware_capabilities(&self) -> Option<&MSFT_NetAdapterUroCapabilities> {
        self.uro_hardware_capabilities.as_ref()
    }

/// 

    /// * `enable` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterUroSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, enable: bool, cmdlet_output: &mut MSFT_NetAdapterUroSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Enable".to_string(), value: enable.into() });

        let result = self.invoke_method("Enable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `disable` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterUroSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, disable: bool, cmdlet_output: &mut MSFT_NetAdapterUroSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Disable".to_string(), value: disable.into() });

        let result = self.invoke_method("Disable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

