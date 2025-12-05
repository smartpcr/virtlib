// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterRscSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterRscSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "IPv4Enabled")]
    pub ipv4_enabled: Option<bool>,

/// 
    #[serde(rename = "IPv4FailureReason")]
    pub ipv4_failure_reason: Option<u32>,

/// 
    #[serde(rename = "IPv4OperationalState")]
    pub ipv4_operational_state: Option<bool>,

/// 
    #[serde(rename = "IPv6Enabled")]
    pub ipv6_enabled: Option<bool>,

/// 
    #[serde(rename = "IPv6FailureReason")]
    pub ipv6_failure_reason: Option<u32>,

/// 
    #[serde(rename = "IPv6OperationalState")]
    pub ipv6_operational_state: Option<bool>,

/// 
    #[serde(rename = "RscHardwareCapabilities")]
    pub rsc_hardware_capabilities: Option<MSFT_NetAdapterRscCapabilities>,
}

impl MSFT_NetAdapterRscSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            ipv4_enabled: None,
            ipv4_failure_reason: None,
            ipv4_operational_state: None,
            ipv6_enabled: None,
            ipv6_failure_reason: None,
            ipv6_operational_state: None,
            rsc_hardware_capabilities: None,
        }
    }


    /// Sets the value of IPv4Enabled
    pub fn set_ipv4_enabled(&mut self, value: bool) {
        self.ipv4_enabled = Some(value);
    }

    /// Gets the value of IPv4Enabled
    pub fn get_ipv4_enabled(&self) -> Option<&bool> {
        self.ipv4_enabled.as_ref()
    }

    /// Sets the value of IPv4FailureReason
    pub fn set_ipv4_failure_reason(&mut self, value: u32) {
        self.ipv4_failure_reason = Some(value);
    }

    /// Gets the value of IPv4FailureReason
    pub fn get_ipv4_failure_reason(&self) -> Option<&u32> {
        self.ipv4_failure_reason.as_ref()
    }

    /// Sets the value of IPv4OperationalState
    pub fn set_ipv4_operational_state(&mut self, value: bool) {
        self.ipv4_operational_state = Some(value);
    }

    /// Gets the value of IPv4OperationalState
    pub fn get_ipv4_operational_state(&self) -> Option<&bool> {
        self.ipv4_operational_state.as_ref()
    }

    /// Sets the value of IPv6Enabled
    pub fn set_ipv6_enabled(&mut self, value: bool) {
        self.ipv6_enabled = Some(value);
    }

    /// Gets the value of IPv6Enabled
    pub fn get_ipv6_enabled(&self) -> Option<&bool> {
        self.ipv6_enabled.as_ref()
    }

    /// Sets the value of IPv6FailureReason
    pub fn set_ipv6_failure_reason(&mut self, value: u32) {
        self.ipv6_failure_reason = Some(value);
    }

    /// Gets the value of IPv6FailureReason
    pub fn get_ipv6_failure_reason(&self) -> Option<&u32> {
        self.ipv6_failure_reason.as_ref()
    }

    /// Sets the value of IPv6OperationalState
    pub fn set_ipv6_operational_state(&mut self, value: bool) {
        self.ipv6_operational_state = Some(value);
    }

    /// Gets the value of IPv6OperationalState
    pub fn get_ipv6_operational_state(&self) -> Option<&bool> {
        self.ipv6_operational_state.as_ref()
    }

    /// Sets the value of RscHardwareCapabilities
    pub fn set_rsc_hardware_capabilities(&mut self, value: MSFT_NetAdapterRscCapabilities) {
        self.rsc_hardware_capabilities = Some(value);
    }

    /// Gets the value of RscHardwareCapabilities
    pub fn get_rsc_hardware_capabilities(&self) -> Option<&MSFT_NetAdapterRscCapabilities> {
        self.rsc_hardware_capabilities.as_ref()
    }

/// 

    /// * `ipv4` -  (bool)
    /// * `ipv6` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterRscSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, ipv4: bool, ipv6: bool, cmdlet_output: &mut MSFT_NetAdapterRscSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPv4".to_string(), value: ipv4.into() });
        args.push(MethodParameter { name: "IPv6".to_string(), value: ipv6.into() });

        let result = self.invoke_method("Enable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `ipv4` -  (bool)
    /// * `ipv6` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterRscSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, ipv4: bool, ipv6: bool, cmdlet_output: &mut MSFT_NetAdapterRscSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPv4".to_string(), value: ipv4.into() });
        args.push(MethodParameter { name: "IPv6".to_string(), value: ipv6.into() });

        let result = self.invoke_method("Disable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

