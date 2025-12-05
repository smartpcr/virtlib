// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterIPsecOffloadV2SettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterIPsecOffloadV2SettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "AhEnabled")]
    pub ah_enabled: Option<bool>,

/// 
    #[serde(rename = "AhEspCombinedEnabled")]
    pub ah_esp_combined_enabled: Option<bool>,

/// 
    #[serde(rename = "AhEspCombinedSupported")]
    pub ah_esp_combined_supported: Option<bool>,

/// 
    #[serde(rename = "AhSupported")]
    pub ah_supported: Option<bool>,

/// 
    #[serde(rename = "AuthenticationAlgorithmsEnabled")]
    pub authentication_algorithms_enabled: Option<u32>,

/// 
    #[serde(rename = "AuthenticationAlgorithmsSupported")]
    pub authentication_algorithms_supported: Option<u32>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "EncryptionAlgorithmsEnabled")]
    pub encryption_algorithms_enabled: Option<u32>,

/// 
    #[serde(rename = "EncryptionAlgorithmsSupported")]
    pub encryption_algorithms_supported: Option<u32>,

/// 
    #[serde(rename = "EspEnabled")]
    pub esp_enabled: Option<bool>,

/// 
    #[serde(rename = "EspSupported")]
    pub esp_supported: Option<bool>,

/// 
    #[serde(rename = "IPv4OptionsEnabled")]
    pub ipv4_options_enabled: Option<bool>,

/// 
    #[serde(rename = "IPv4OptionsSupported")]
    pub ipv4_options_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6Enabled")]
    pub ipv6_enabled: Option<bool>,

/// 
    #[serde(rename = "IPv6NonIPsecExtensionHeadersEnabled")]
    pub ipv6_non_ipsec_extension_headers_enabled: Option<bool>,

/// 
    #[serde(rename = "IPv6NonIPsecExtensionHeadersSupported")]
    pub ipv6_non_ipsec_extension_headers_supported: Option<bool>,

/// 
    #[serde(rename = "IPv6Supported")]
    pub ipv6_supported: Option<bool>,

/// 
    #[serde(rename = "LsoEnabled")]
    pub lso_enabled: Option<bool>,

/// 
    #[serde(rename = "LsoSupported")]
    pub lso_supported: Option<bool>,

/// 
    #[serde(rename = "SaOffloadCapacityEnabled")]
    pub sa_offload_capacity_enabled: Option<u32>,

/// 
    #[serde(rename = "SaOffloadCapacitySupported")]
    pub sa_offload_capacity_supported: Option<u32>,

/// 
    #[serde(rename = "TransportEnabled")]
    pub transport_enabled: Option<bool>,

/// 
    #[serde(rename = "TransportSupported")]
    pub transport_supported: Option<bool>,

/// 
    #[serde(rename = "TunnelEnabled")]
    pub tunnel_enabled: Option<bool>,

/// 
    #[serde(rename = "TunnelSupported")]
    pub tunnel_supported: Option<bool>,

/// 
    #[serde(rename = "UdpEspEnabled")]
    pub udp_esp_enabled: Option<u32>,

/// 
    #[serde(rename = "UdpEspSupported")]
    pub udp_esp_supported: Option<u32>,
}

impl MSFT_NetAdapterIPsecOffloadV2SettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            ah_enabled: None,
            ah_esp_combined_enabled: None,
            ah_esp_combined_supported: None,
            ah_supported: None,
            authentication_algorithms_enabled: None,
            authentication_algorithms_supported: None,
            enabled: None,
            encryption_algorithms_enabled: None,
            encryption_algorithms_supported: None,
            esp_enabled: None,
            esp_supported: None,
            ipv4_options_enabled: None,
            ipv4_options_supported: None,
            ipv6_enabled: None,
            ipv6_non_ipsec_extension_headers_enabled: None,
            ipv6_non_ipsec_extension_headers_supported: None,
            ipv6_supported: None,
            lso_enabled: None,
            lso_supported: None,
            sa_offload_capacity_enabled: None,
            sa_offload_capacity_supported: None,
            transport_enabled: None,
            transport_supported: None,
            tunnel_enabled: None,
            tunnel_supported: None,
            udp_esp_enabled: None,
            udp_esp_supported: None,
        }
    }


    /// Sets the value of AhEnabled
    pub fn set_ah_enabled(&mut self, value: bool) {
        self.ah_enabled = Some(value);
    }

    /// Gets the value of AhEnabled
    pub fn get_ah_enabled(&self) -> Option<&bool> {
        self.ah_enabled.as_ref()
    }

    /// Sets the value of AhEspCombinedEnabled
    pub fn set_ah_esp_combined_enabled(&mut self, value: bool) {
        self.ah_esp_combined_enabled = Some(value);
    }

    /// Gets the value of AhEspCombinedEnabled
    pub fn get_ah_esp_combined_enabled(&self) -> Option<&bool> {
        self.ah_esp_combined_enabled.as_ref()
    }

    /// Sets the value of AhEspCombinedSupported
    pub fn set_ah_esp_combined_supported(&mut self, value: bool) {
        self.ah_esp_combined_supported = Some(value);
    }

    /// Gets the value of AhEspCombinedSupported
    pub fn get_ah_esp_combined_supported(&self) -> Option<&bool> {
        self.ah_esp_combined_supported.as_ref()
    }

    /// Sets the value of AhSupported
    pub fn set_ah_supported(&mut self, value: bool) {
        self.ah_supported = Some(value);
    }

    /// Gets the value of AhSupported
    pub fn get_ah_supported(&self) -> Option<&bool> {
        self.ah_supported.as_ref()
    }

    /// Sets the value of AuthenticationAlgorithmsEnabled
    pub fn set_authentication_algorithms_enabled(&mut self, value: u32) {
        self.authentication_algorithms_enabled = Some(value);
    }

    /// Gets the value of AuthenticationAlgorithmsEnabled
    pub fn get_authentication_algorithms_enabled(&self) -> Option<&u32> {
        self.authentication_algorithms_enabled.as_ref()
    }

    /// Sets the value of AuthenticationAlgorithmsSupported
    pub fn set_authentication_algorithms_supported(&mut self, value: u32) {
        self.authentication_algorithms_supported = Some(value);
    }

    /// Gets the value of AuthenticationAlgorithmsSupported
    pub fn get_authentication_algorithms_supported(&self) -> Option<&u32> {
        self.authentication_algorithms_supported.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of EncryptionAlgorithmsEnabled
    pub fn set_encryption_algorithms_enabled(&mut self, value: u32) {
        self.encryption_algorithms_enabled = Some(value);
    }

    /// Gets the value of EncryptionAlgorithmsEnabled
    pub fn get_encryption_algorithms_enabled(&self) -> Option<&u32> {
        self.encryption_algorithms_enabled.as_ref()
    }

    /// Sets the value of EncryptionAlgorithmsSupported
    pub fn set_encryption_algorithms_supported(&mut self, value: u32) {
        self.encryption_algorithms_supported = Some(value);
    }

    /// Gets the value of EncryptionAlgorithmsSupported
    pub fn get_encryption_algorithms_supported(&self) -> Option<&u32> {
        self.encryption_algorithms_supported.as_ref()
    }

    /// Sets the value of EspEnabled
    pub fn set_esp_enabled(&mut self, value: bool) {
        self.esp_enabled = Some(value);
    }

    /// Gets the value of EspEnabled
    pub fn get_esp_enabled(&self) -> Option<&bool> {
        self.esp_enabled.as_ref()
    }

    /// Sets the value of EspSupported
    pub fn set_esp_supported(&mut self, value: bool) {
        self.esp_supported = Some(value);
    }

    /// Gets the value of EspSupported
    pub fn get_esp_supported(&self) -> Option<&bool> {
        self.esp_supported.as_ref()
    }

    /// Sets the value of IPv4OptionsEnabled
    pub fn set_ipv4_options_enabled(&mut self, value: bool) {
        self.ipv4_options_enabled = Some(value);
    }

    /// Gets the value of IPv4OptionsEnabled
    pub fn get_ipv4_options_enabled(&self) -> Option<&bool> {
        self.ipv4_options_enabled.as_ref()
    }

    /// Sets the value of IPv4OptionsSupported
    pub fn set_ipv4_options_supported(&mut self, value: bool) {
        self.ipv4_options_supported = Some(value);
    }

    /// Gets the value of IPv4OptionsSupported
    pub fn get_ipv4_options_supported(&self) -> Option<&bool> {
        self.ipv4_options_supported.as_ref()
    }

    /// Sets the value of IPv6Enabled
    pub fn set_ipv6_enabled(&mut self, value: bool) {
        self.ipv6_enabled = Some(value);
    }

    /// Gets the value of IPv6Enabled
    pub fn get_ipv6_enabled(&self) -> Option<&bool> {
        self.ipv6_enabled.as_ref()
    }

    /// Sets the value of IPv6NonIPsecExtensionHeadersEnabled
    pub fn set_ipv6_non_ipsec_extension_headers_enabled(&mut self, value: bool) {
        self.ipv6_non_ipsec_extension_headers_enabled = Some(value);
    }

    /// Gets the value of IPv6NonIPsecExtensionHeadersEnabled
    pub fn get_ipv6_non_ipsec_extension_headers_enabled(&self) -> Option<&bool> {
        self.ipv6_non_ipsec_extension_headers_enabled.as_ref()
    }

    /// Sets the value of IPv6NonIPsecExtensionHeadersSupported
    pub fn set_ipv6_non_ipsec_extension_headers_supported(&mut self, value: bool) {
        self.ipv6_non_ipsec_extension_headers_supported = Some(value);
    }

    /// Gets the value of IPv6NonIPsecExtensionHeadersSupported
    pub fn get_ipv6_non_ipsec_extension_headers_supported(&self) -> Option<&bool> {
        self.ipv6_non_ipsec_extension_headers_supported.as_ref()
    }

    /// Sets the value of IPv6Supported
    pub fn set_ipv6_supported(&mut self, value: bool) {
        self.ipv6_supported = Some(value);
    }

    /// Gets the value of IPv6Supported
    pub fn get_ipv6_supported(&self) -> Option<&bool> {
        self.ipv6_supported.as_ref()
    }

    /// Sets the value of LsoEnabled
    pub fn set_lso_enabled(&mut self, value: bool) {
        self.lso_enabled = Some(value);
    }

    /// Gets the value of LsoEnabled
    pub fn get_lso_enabled(&self) -> Option<&bool> {
        self.lso_enabled.as_ref()
    }

    /// Sets the value of LsoSupported
    pub fn set_lso_supported(&mut self, value: bool) {
        self.lso_supported = Some(value);
    }

    /// Gets the value of LsoSupported
    pub fn get_lso_supported(&self) -> Option<&bool> {
        self.lso_supported.as_ref()
    }

    /// Sets the value of SaOffloadCapacityEnabled
    pub fn set_sa_offload_capacity_enabled(&mut self, value: u32) {
        self.sa_offload_capacity_enabled = Some(value);
    }

    /// Gets the value of SaOffloadCapacityEnabled
    pub fn get_sa_offload_capacity_enabled(&self) -> Option<&u32> {
        self.sa_offload_capacity_enabled.as_ref()
    }

    /// Sets the value of SaOffloadCapacitySupported
    pub fn set_sa_offload_capacity_supported(&mut self, value: u32) {
        self.sa_offload_capacity_supported = Some(value);
    }

    /// Gets the value of SaOffloadCapacitySupported
    pub fn get_sa_offload_capacity_supported(&self) -> Option<&u32> {
        self.sa_offload_capacity_supported.as_ref()
    }

    /// Sets the value of TransportEnabled
    pub fn set_transport_enabled(&mut self, value: bool) {
        self.transport_enabled = Some(value);
    }

    /// Gets the value of TransportEnabled
    pub fn get_transport_enabled(&self) -> Option<&bool> {
        self.transport_enabled.as_ref()
    }

    /// Sets the value of TransportSupported
    pub fn set_transport_supported(&mut self, value: bool) {
        self.transport_supported = Some(value);
    }

    /// Gets the value of TransportSupported
    pub fn get_transport_supported(&self) -> Option<&bool> {
        self.transport_supported.as_ref()
    }

    /// Sets the value of TunnelEnabled
    pub fn set_tunnel_enabled(&mut self, value: bool) {
        self.tunnel_enabled = Some(value);
    }

    /// Gets the value of TunnelEnabled
    pub fn get_tunnel_enabled(&self) -> Option<&bool> {
        self.tunnel_enabled.as_ref()
    }

    /// Sets the value of TunnelSupported
    pub fn set_tunnel_supported(&mut self, value: bool) {
        self.tunnel_supported = Some(value);
    }

    /// Gets the value of TunnelSupported
    pub fn get_tunnel_supported(&self) -> Option<&bool> {
        self.tunnel_supported.as_ref()
    }

    /// Sets the value of UdpEspEnabled
    pub fn set_udp_esp_enabled(&mut self, value: u32) {
        self.udp_esp_enabled = Some(value);
    }

    /// Gets the value of UdpEspEnabled
    pub fn get_udp_esp_enabled(&self) -> Option<&u32> {
        self.udp_esp_enabled.as_ref()
    }

    /// Sets the value of UdpEspSupported
    pub fn set_udp_esp_supported(&mut self, value: u32) {
        self.udp_esp_supported = Some(value);
    }

    /// Gets the value of UdpEspSupported
    pub fn get_udp_esp_supported(&self) -> Option<&u32> {
        self.udp_esp_supported.as_ref()
    }

/// 

    /// * `enabled` -  (bool)
    /// * `no_restart` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterIPsecOffloadV2SettingData)
    /// * `return_value` -  (u32)
    pub fn set(&self, enabled: bool, no_restart: bool, pass_thru: bool, cmdlet_output: &mut MSFT_NetAdapterIPsecOffloadV2SettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Enabled".to_string(), value: enabled.into() });
        args.push(MethodParameter { name: "NoRestart".to_string(), value: no_restart.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Set", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `no_restart` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterIPsecOffloadV2SettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, no_restart: bool, pass_thru: bool, cmdlet_output: &mut MSFT_NetAdapterIPsecOffloadV2SettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NoRestart".to_string(), value: no_restart.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Enable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `no_restart` -  (bool)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterIPsecOffloadV2SettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, no_restart: bool, pass_thru: bool, cmdlet_output: &mut MSFT_NetAdapterIPsecOffloadV2SettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NoRestart".to_string(), value: no_restart.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Disable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

