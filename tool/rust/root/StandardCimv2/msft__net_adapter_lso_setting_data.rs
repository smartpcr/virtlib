// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterLsoSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterLsoSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "IPv4Enabled")]
    pub ipv4_enabled: Option<bool>,

/// 
    #[serde(rename = "IPv6Enabled")]
    pub ipv6_enabled: Option<bool>,

/// 
    #[serde(rename = "LargeSendOffloadV1HardwareCapabilities")]
    pub large_send_offload_v1_hardware_capabilities: Option<MSFT_NetAdapterLargeSendOffloadV1Capabilities>,

/// 
    #[serde(rename = "LargeSendOffloadV2HardwareCapabilities")]
    pub large_send_offload_v2_hardware_capabilities: Option<MSFT_NetAdapterLargeSendOffloadV2Capabilities>,

/// 
    #[serde(rename = "MaximumLsoVersionSupported")]
    pub maximum_lso_version_supported: Option<u32>,

/// 
    #[serde(rename = "V1IPv4Enabled")]
    pub v1_ipv4_enabled: Option<bool>,
}

impl MSFT_NetAdapterLsoSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            ipv4_enabled: None,
            ipv6_enabled: None,
            large_send_offload_v1_hardware_capabilities: None,
            large_send_offload_v2_hardware_capabilities: None,
            maximum_lso_version_supported: None,
            v1_ipv4_enabled: None,
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

    /// Sets the value of IPv6Enabled
    pub fn set_ipv6_enabled(&mut self, value: bool) {
        self.ipv6_enabled = Some(value);
    }

    /// Gets the value of IPv6Enabled
    pub fn get_ipv6_enabled(&self) -> Option<&bool> {
        self.ipv6_enabled.as_ref()
    }

    /// Sets the value of LargeSendOffloadV1HardwareCapabilities
    pub fn set_large_send_offload_v1_hardware_capabilities(&mut self, value: MSFT_NetAdapterLargeSendOffloadV1Capabilities) {
        self.large_send_offload_v1_hardware_capabilities = Some(value);
    }

    /// Gets the value of LargeSendOffloadV1HardwareCapabilities
    pub fn get_large_send_offload_v1_hardware_capabilities(&self) -> Option<&MSFT_NetAdapterLargeSendOffloadV1Capabilities> {
        self.large_send_offload_v1_hardware_capabilities.as_ref()
    }

    /// Sets the value of LargeSendOffloadV2HardwareCapabilities
    pub fn set_large_send_offload_v2_hardware_capabilities(&mut self, value: MSFT_NetAdapterLargeSendOffloadV2Capabilities) {
        self.large_send_offload_v2_hardware_capabilities = Some(value);
    }

    /// Gets the value of LargeSendOffloadV2HardwareCapabilities
    pub fn get_large_send_offload_v2_hardware_capabilities(&self) -> Option<&MSFT_NetAdapterLargeSendOffloadV2Capabilities> {
        self.large_send_offload_v2_hardware_capabilities.as_ref()
    }

    /// Sets the value of MaximumLsoVersionSupported
    pub fn set_maximum_lso_version_supported(&mut self, value: u32) {
        self.maximum_lso_version_supported = Some(value);
    }

    /// Gets the value of MaximumLsoVersionSupported
    pub fn get_maximum_lso_version_supported(&self) -> Option<&u32> {
        self.maximum_lso_version_supported.as_ref()
    }

    /// Sets the value of V1IPv4Enabled
    pub fn set_v1_ipv4_enabled(&mut self, value: bool) {
        self.v1_ipv4_enabled = Some(value);
    }

    /// Gets the value of V1IPv4Enabled
    pub fn get_v1_ipv4_enabled(&self) -> Option<&bool> {
        self.v1_ipv4_enabled.as_ref()
    }

/// 

    /// * `ipv4` -  (bool)
    /// * `ipv6` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterLsoSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, ipv4: bool, ipv6: bool, cmdlet_output: &mut MSFT_NetAdapterLsoSettingData) -> Result<(), WmiError> {
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

    /// * `cmdlet_output` -  (MSFT_NetAdapterLsoSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, ipv4: bool, ipv6: bool, cmdlet_output: &mut MSFT_NetAdapterLsoSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPv4".to_string(), value: ipv4.into() });
        args.push(MethodParameter { name: "IPv6".to_string(), value: ipv6.into() });

        let result = self.invoke_method("Disable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

