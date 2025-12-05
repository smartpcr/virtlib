// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterChecksumOffloadSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterChecksumOffloadSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "ChecksumOffloadHardwareCapabilities")]
    pub checksum_offload_hardware_capabilities: Option<MSFT_NetAdapterChecksumOffloadCapabilities>,

/// 
    #[serde(rename = "IpIPv4Enabled")]
    pub ip_ipv4_enabled: Option<u32>,

/// 
    #[serde(rename = "TcpIPv4Enabled")]
    pub tcp_ipv4_enabled: Option<u32>,

/// 
    #[serde(rename = "TcpIPv6Enabled")]
    pub tcp_ipv6_enabled: Option<u32>,

/// 
    #[serde(rename = "UdpIPv4Enabled")]
    pub udp_ipv4_enabled: Option<u32>,

/// 
    #[serde(rename = "UdpIPv6Enabled")]
    pub udp_ipv6_enabled: Option<u32>,
}

impl MSFT_NetAdapterChecksumOffloadSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            checksum_offload_hardware_capabilities: None,
            ip_ipv4_enabled: None,
            tcp_ipv4_enabled: None,
            tcp_ipv6_enabled: None,
            udp_ipv4_enabled: None,
            udp_ipv6_enabled: None,
        }
    }


    /// Sets the value of ChecksumOffloadHardwareCapabilities
    pub fn set_checksum_offload_hardware_capabilities(&mut self, value: MSFT_NetAdapterChecksumOffloadCapabilities) {
        self.checksum_offload_hardware_capabilities = Some(value);
    }

    /// Gets the value of ChecksumOffloadHardwareCapabilities
    pub fn get_checksum_offload_hardware_capabilities(&self) -> Option<&MSFT_NetAdapterChecksumOffloadCapabilities> {
        self.checksum_offload_hardware_capabilities.as_ref()
    }

    /// Sets the value of IpIPv4Enabled
    pub fn set_ip_ipv4_enabled(&mut self, value: u32) {
        self.ip_ipv4_enabled = Some(value);
    }

    /// Gets the value of IpIPv4Enabled
    pub fn get_ip_ipv4_enabled(&self) -> Option<&u32> {
        self.ip_ipv4_enabled.as_ref()
    }

    /// Sets the value of TcpIPv4Enabled
    pub fn set_tcp_ipv4_enabled(&mut self, value: u32) {
        self.tcp_ipv4_enabled = Some(value);
    }

    /// Gets the value of TcpIPv4Enabled
    pub fn get_tcp_ipv4_enabled(&self) -> Option<&u32> {
        self.tcp_ipv4_enabled.as_ref()
    }

    /// Sets the value of TcpIPv6Enabled
    pub fn set_tcp_ipv6_enabled(&mut self, value: u32) {
        self.tcp_ipv6_enabled = Some(value);
    }

    /// Gets the value of TcpIPv6Enabled
    pub fn get_tcp_ipv6_enabled(&self) -> Option<&u32> {
        self.tcp_ipv6_enabled.as_ref()
    }

    /// Sets the value of UdpIPv4Enabled
    pub fn set_udp_ipv4_enabled(&mut self, value: u32) {
        self.udp_ipv4_enabled = Some(value);
    }

    /// Gets the value of UdpIPv4Enabled
    pub fn get_udp_ipv4_enabled(&self) -> Option<&u32> {
        self.udp_ipv4_enabled.as_ref()
    }

    /// Sets the value of UdpIPv6Enabled
    pub fn set_udp_ipv6_enabled(&mut self, value: u32) {
        self.udp_ipv6_enabled = Some(value);
    }

    /// Gets the value of UdpIPv6Enabled
    pub fn get_udp_ipv6_enabled(&self) -> Option<&u32> {
        self.udp_ipv6_enabled.as_ref()
    }

/// 

    /// * `ip_ipv4` -  (bool)
    /// * `rx_tx_control` -  (u32)
    /// * `tcp_ipv4` -  (bool)
    /// * `tcp_ipv6` -  (bool)
    /// * `udp_ipv4` -  (bool)
    /// * `udp_ipv6` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterChecksumOffloadSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, ip_ipv4: bool, tcp_ipv4: bool, tcp_ipv6: bool, udp_ipv4: bool, udp_ipv6: bool, rx_tx_control: u32, cmdlet_output: &mut MSFT_NetAdapterChecksumOffloadSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IpIPv4".to_string(), value: ip_ipv4.into() });
        args.push(MethodParameter { name: "TcpIPv4".to_string(), value: tcp_ipv4.into() });
        args.push(MethodParameter { name: "TcpIPv6".to_string(), value: tcp_ipv6.into() });
        args.push(MethodParameter { name: "UdpIPv4".to_string(), value: udp_ipv4.into() });
        args.push(MethodParameter { name: "UdpIPv6".to_string(), value: udp_ipv6.into() });
        args.push(MethodParameter { name: "RxTxControl".to_string(), value: rx_tx_control.into() });

        let result = self.invoke_method("Enable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `ip_ipv4` -  (bool)
    /// * `rx_tx_control` -  (u32)
    /// * `tcp_ipv4` -  (bool)
    /// * `tcp_ipv6` -  (bool)
    /// * `udp_ipv4` -  (bool)
    /// * `udp_ipv6` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetAdapterChecksumOffloadSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, ip_ipv4: bool, tcp_ipv4: bool, tcp_ipv6: bool, udp_ipv4: bool, udp_ipv6: bool, rx_tx_control: u32, cmdlet_output: &mut MSFT_NetAdapterChecksumOffloadSettingData) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IpIPv4".to_string(), value: ip_ipv4.into() });
        args.push(MethodParameter { name: "TcpIPv4".to_string(), value: tcp_ipv4.into() });
        args.push(MethodParameter { name: "TcpIPv6".to_string(), value: tcp_ipv6.into() });
        args.push(MethodParameter { name: "UdpIPv4".to_string(), value: udp_ipv4.into() });
        args.push(MethodParameter { name: "UdpIPv6".to_string(), value: udp_ipv6.into() });
        args.push(MethodParameter { name: "RxTxControl".to_string(), value: rx_tx_control.into() });

        let result = self.invoke_method("Disable", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

