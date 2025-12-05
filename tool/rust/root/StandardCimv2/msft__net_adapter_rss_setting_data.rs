// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterRssSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterRssSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "BaseProcessorGroup")]
    pub base_processor_group: Option<u16>,

/// 
    #[serde(rename = "BaseProcessorNumber")]
    pub base_processor_number: Option<u8>,

/// 
    #[serde(rename = "ClassificationAtDpcSupported")]
    pub classification_at_dpc_supported: Option<bool>,

/// 
    #[serde(rename = "ClassificationAtIsrSupported")]
    pub classification_at_isr_supported: Option<bool>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "HashKeySize")]
    pub hash_key_size: Option<u16>,

/// 
    #[serde(rename = "IndirectionTable")]
    pub indirection_table: Vec<MSFT_NetAdapter_ProcessorNumber>,

/// 
    #[serde(rename = "IndirectionTableEntryCount")]
    pub indirection_table_entry_count: Option<u16>,

/// 
    #[serde(rename = "IPv4HashEnabled")]
    pub ipv4_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "IPv6ExtensionHashEnabled")]
    pub ipv6_extension_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "IPv6HashEnabled")]
    pub ipv6_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "MaxProcessorGroup")]
    pub max_processor_group: Option<u16>,

/// 
    #[serde(rename = "MaxProcessorNumber")]
    pub max_processor_number: Option<u8>,

/// 
    #[serde(rename = "MaxProcessors")]
    pub max_processors: Option<u32>,

/// 
    #[serde(rename = "MsiSupported")]
    pub msi_supported: Option<bool>,

/// 
    #[serde(rename = "MsiXEnabled")]
    pub msi_xenabled: Option<bool>,

/// 
    #[serde(rename = "MsiXSupported")]
    pub msi_xsupported: Option<bool>,

/// 
    #[serde(rename = "NumaNode")]
    pub numa_node: Option<u16>,

/// 
    #[serde(rename = "NumberOfInterruptMessages")]
    pub number_of_interrupt_messages: Option<u32>,

/// 
    #[serde(rename = "NumberOfReceiveQueues")]
    pub number_of_receive_queues: Option<u32>,

/// 
    #[serde(rename = "Profile")]
    pub profile: Option<u32>,

/// 
    #[serde(rename = "RssOnPortsSupported")]
    pub rss_on_ports_supported: Option<bool>,

/// 
    #[serde(rename = "RssProcessorArray")]
    pub rss_processor_array: Vec<MSFT_NetAdapter_RssProcessor>,

/// 
    #[serde(rename = "RssProcessorArraySize")]
    pub rss_processor_array_size: Option<u32>,

/// 
    #[serde(rename = "TcpIPv4HashEnabled")]
    pub tcp_ipv4_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "TcpIPv4HashSupported")]
    pub tcp_ipv4_hash_supported: Option<bool>,

/// 
    #[serde(rename = "TcpIPv6ExtensionHashEnabled")]
    pub tcp_ipv6_extension_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "TcpIPv6ExtensionHashSupported")]
    pub tcp_ipv6_extension_hash_supported: Option<bool>,

/// 
    #[serde(rename = "TcpIPv6HashEnabled")]
    pub tcp_ipv6_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "TcpIPv6HashSupported")]
    pub tcp_ipv6_hash_supported: Option<bool>,

/// 
    #[serde(rename = "ToeplitzHashFunctionEnabled")]
    pub toeplitz_hash_function_enabled: Option<bool>,

/// 
    #[serde(rename = "ToeplitzHashFunctionSupported")]
    pub toeplitz_hash_function_supported: Option<bool>,

/// 
    #[serde(rename = "UdpIPv4HashEnabled")]
    pub udp_ipv4_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "UdpIPv4HashSupported")]
    pub udp_ipv4_hash_supported: Option<bool>,

/// 
    #[serde(rename = "UdpIPv6ExtensionHashEnabled")]
    pub udp_ipv6_extension_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "UdpIPv6ExtensionHashSupported")]
    pub udp_ipv6_extension_hash_supported: Option<bool>,

/// 
    #[serde(rename = "UdpIPv6HashEnabled")]
    pub udp_ipv6_hash_enabled: Option<bool>,

/// 
    #[serde(rename = "UdpIPv6HashSupported")]
    pub udp_ipv6_hash_supported: Option<bool>,
}

impl MSFT_NetAdapterRssSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            base_processor_group: None,
            base_processor_number: None,
            classification_at_dpc_supported: None,
            classification_at_isr_supported: None,
            enabled: None,
            hash_key_size: None,
            indirection_table: Vec::new(),
            indirection_table_entry_count: None,
            ipv4_hash_enabled: None,
            ipv6_extension_hash_enabled: None,
            ipv6_hash_enabled: None,
            max_processor_group: None,
            max_processor_number: None,
            max_processors: None,
            msi_supported: None,
            msi_xenabled: None,
            msi_xsupported: None,
            numa_node: None,
            number_of_interrupt_messages: None,
            number_of_receive_queues: None,
            profile: None,
            rss_on_ports_supported: None,
            rss_processor_array: Vec::new(),
            rss_processor_array_size: None,
            tcp_ipv4_hash_enabled: None,
            tcp_ipv4_hash_supported: None,
            tcp_ipv6_extension_hash_enabled: None,
            tcp_ipv6_extension_hash_supported: None,
            tcp_ipv6_hash_enabled: None,
            tcp_ipv6_hash_supported: None,
            toeplitz_hash_function_enabled: None,
            toeplitz_hash_function_supported: None,
            udp_ipv4_hash_enabled: None,
            udp_ipv4_hash_supported: None,
            udp_ipv6_extension_hash_enabled: None,
            udp_ipv6_extension_hash_supported: None,
            udp_ipv6_hash_enabled: None,
            udp_ipv6_hash_supported: None,
        }
    }


    /// Sets the value of BaseProcessorGroup
    pub fn set_base_processor_group(&mut self, value: u16) {
        self.base_processor_group = Some(value);
    }

    /// Gets the value of BaseProcessorGroup
    pub fn get_base_processor_group(&self) -> Option<&u16> {
        self.base_processor_group.as_ref()
    }

    /// Sets the value of BaseProcessorNumber
    pub fn set_base_processor_number(&mut self, value: u8) {
        self.base_processor_number = Some(value);
    }

    /// Gets the value of BaseProcessorNumber
    pub fn get_base_processor_number(&self) -> Option<&u8> {
        self.base_processor_number.as_ref()
    }

    /// Sets the value of ClassificationAtDpcSupported
    pub fn set_classification_at_dpc_supported(&mut self, value: bool) {
        self.classification_at_dpc_supported = Some(value);
    }

    /// Gets the value of ClassificationAtDpcSupported
    pub fn get_classification_at_dpc_supported(&self) -> Option<&bool> {
        self.classification_at_dpc_supported.as_ref()
    }

    /// Sets the value of ClassificationAtIsrSupported
    pub fn set_classification_at_isr_supported(&mut self, value: bool) {
        self.classification_at_isr_supported = Some(value);
    }

    /// Gets the value of ClassificationAtIsrSupported
    pub fn get_classification_at_isr_supported(&self) -> Option<&bool> {
        self.classification_at_isr_supported.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of HashKeySize
    pub fn set_hash_key_size(&mut self, value: u16) {
        self.hash_key_size = Some(value);
    }

    /// Gets the value of HashKeySize
    pub fn get_hash_key_size(&self) -> Option<&u16> {
        self.hash_key_size.as_ref()
    }

    /// Sets the value of IndirectionTable
    pub fn set_indirection_table(&mut self, value: Vec<MSFT_NetAdapter_ProcessorNumber>) {
        self.indirection_table = value;
    }

    /// Gets the value of IndirectionTable
    pub fn get_indirection_table(&self) -> &Vec<MSFT_NetAdapter_ProcessorNumber> {
        &self.indirection_table
    }

    /// Sets the value of IndirectionTableEntryCount
    pub fn set_indirection_table_entry_count(&mut self, value: u16) {
        self.indirection_table_entry_count = Some(value);
    }

    /// Gets the value of IndirectionTableEntryCount
    pub fn get_indirection_table_entry_count(&self) -> Option<&u16> {
        self.indirection_table_entry_count.as_ref()
    }

    /// Sets the value of IPv4HashEnabled
    pub fn set_ipv4_hash_enabled(&mut self, value: bool) {
        self.ipv4_hash_enabled = Some(value);
    }

    /// Gets the value of IPv4HashEnabled
    pub fn get_ipv4_hash_enabled(&self) -> Option<&bool> {
        self.ipv4_hash_enabled.as_ref()
    }

    /// Sets the value of IPv6ExtensionHashEnabled
    pub fn set_ipv6_extension_hash_enabled(&mut self, value: bool) {
        self.ipv6_extension_hash_enabled = Some(value);
    }

    /// Gets the value of IPv6ExtensionHashEnabled
    pub fn get_ipv6_extension_hash_enabled(&self) -> Option<&bool> {
        self.ipv6_extension_hash_enabled.as_ref()
    }

    /// Sets the value of IPv6HashEnabled
    pub fn set_ipv6_hash_enabled(&mut self, value: bool) {
        self.ipv6_hash_enabled = Some(value);
    }

    /// Gets the value of IPv6HashEnabled
    pub fn get_ipv6_hash_enabled(&self) -> Option<&bool> {
        self.ipv6_hash_enabled.as_ref()
    }

    /// Sets the value of MaxProcessorGroup
    pub fn set_max_processor_group(&mut self, value: u16) {
        self.max_processor_group = Some(value);
    }

    /// Gets the value of MaxProcessorGroup
    pub fn get_max_processor_group(&self) -> Option<&u16> {
        self.max_processor_group.as_ref()
    }

    /// Sets the value of MaxProcessorNumber
    pub fn set_max_processor_number(&mut self, value: u8) {
        self.max_processor_number = Some(value);
    }

    /// Gets the value of MaxProcessorNumber
    pub fn get_max_processor_number(&self) -> Option<&u8> {
        self.max_processor_number.as_ref()
    }

    /// Sets the value of MaxProcessors
    pub fn set_max_processors(&mut self, value: u32) {
        self.max_processors = Some(value);
    }

    /// Gets the value of MaxProcessors
    pub fn get_max_processors(&self) -> Option<&u32> {
        self.max_processors.as_ref()
    }

    /// Sets the value of MsiSupported
    pub fn set_msi_supported(&mut self, value: bool) {
        self.msi_supported = Some(value);
    }

    /// Gets the value of MsiSupported
    pub fn get_msi_supported(&self) -> Option<&bool> {
        self.msi_supported.as_ref()
    }

    /// Sets the value of MsiXEnabled
    pub fn set_msi_xenabled(&mut self, value: bool) {
        self.msi_xenabled = Some(value);
    }

    /// Gets the value of MsiXEnabled
    pub fn get_msi_xenabled(&self) -> Option<&bool> {
        self.msi_xenabled.as_ref()
    }

    /// Sets the value of MsiXSupported
    pub fn set_msi_xsupported(&mut self, value: bool) {
        self.msi_xsupported = Some(value);
    }

    /// Gets the value of MsiXSupported
    pub fn get_msi_xsupported(&self) -> Option<&bool> {
        self.msi_xsupported.as_ref()
    }

    /// Sets the value of NumaNode
    pub fn set_numa_node(&mut self, value: u16) {
        self.numa_node = Some(value);
    }

    /// Gets the value of NumaNode
    pub fn get_numa_node(&self) -> Option<&u16> {
        self.numa_node.as_ref()
    }

    /// Sets the value of NumberOfInterruptMessages
    pub fn set_number_of_interrupt_messages(&mut self, value: u32) {
        self.number_of_interrupt_messages = Some(value);
    }

    /// Gets the value of NumberOfInterruptMessages
    pub fn get_number_of_interrupt_messages(&self) -> Option<&u32> {
        self.number_of_interrupt_messages.as_ref()
    }

    /// Sets the value of NumberOfReceiveQueues
    pub fn set_number_of_receive_queues(&mut self, value: u32) {
        self.number_of_receive_queues = Some(value);
    }

    /// Gets the value of NumberOfReceiveQueues
    pub fn get_number_of_receive_queues(&self) -> Option<&u32> {
        self.number_of_receive_queues.as_ref()
    }

    /// Sets the value of Profile
    pub fn set_profile(&mut self, value: u32) {
        self.profile = Some(value);
    }

    /// Gets the value of Profile
    pub fn get_profile(&self) -> Option<&u32> {
        self.profile.as_ref()
    }

    /// Sets the value of RssOnPortsSupported
    pub fn set_rss_on_ports_supported(&mut self, value: bool) {
        self.rss_on_ports_supported = Some(value);
    }

    /// Gets the value of RssOnPortsSupported
    pub fn get_rss_on_ports_supported(&self) -> Option<&bool> {
        self.rss_on_ports_supported.as_ref()
    }

    /// Sets the value of RssProcessorArray
    pub fn set_rss_processor_array(&mut self, value: Vec<MSFT_NetAdapter_RssProcessor>) {
        self.rss_processor_array = value;
    }

    /// Gets the value of RssProcessorArray
    pub fn get_rss_processor_array(&self) -> &Vec<MSFT_NetAdapter_RssProcessor> {
        &self.rss_processor_array
    }

    /// Sets the value of RssProcessorArraySize
    pub fn set_rss_processor_array_size(&mut self, value: u32) {
        self.rss_processor_array_size = Some(value);
    }

    /// Gets the value of RssProcessorArraySize
    pub fn get_rss_processor_array_size(&self) -> Option<&u32> {
        self.rss_processor_array_size.as_ref()
    }

    /// Sets the value of TcpIPv4HashEnabled
    pub fn set_tcp_ipv4_hash_enabled(&mut self, value: bool) {
        self.tcp_ipv4_hash_enabled = Some(value);
    }

    /// Gets the value of TcpIPv4HashEnabled
    pub fn get_tcp_ipv4_hash_enabled(&self) -> Option<&bool> {
        self.tcp_ipv4_hash_enabled.as_ref()
    }

    /// Sets the value of TcpIPv4HashSupported
    pub fn set_tcp_ipv4_hash_supported(&mut self, value: bool) {
        self.tcp_ipv4_hash_supported = Some(value);
    }

    /// Gets the value of TcpIPv4HashSupported
    pub fn get_tcp_ipv4_hash_supported(&self) -> Option<&bool> {
        self.tcp_ipv4_hash_supported.as_ref()
    }

    /// Sets the value of TcpIPv6ExtensionHashEnabled
    pub fn set_tcp_ipv6_extension_hash_enabled(&mut self, value: bool) {
        self.tcp_ipv6_extension_hash_enabled = Some(value);
    }

    /// Gets the value of TcpIPv6ExtensionHashEnabled
    pub fn get_tcp_ipv6_extension_hash_enabled(&self) -> Option<&bool> {
        self.tcp_ipv6_extension_hash_enabled.as_ref()
    }

    /// Sets the value of TcpIPv6ExtensionHashSupported
    pub fn set_tcp_ipv6_extension_hash_supported(&mut self, value: bool) {
        self.tcp_ipv6_extension_hash_supported = Some(value);
    }

    /// Gets the value of TcpIPv6ExtensionHashSupported
    pub fn get_tcp_ipv6_extension_hash_supported(&self) -> Option<&bool> {
        self.tcp_ipv6_extension_hash_supported.as_ref()
    }

    /// Sets the value of TcpIPv6HashEnabled
    pub fn set_tcp_ipv6_hash_enabled(&mut self, value: bool) {
        self.tcp_ipv6_hash_enabled = Some(value);
    }

    /// Gets the value of TcpIPv6HashEnabled
    pub fn get_tcp_ipv6_hash_enabled(&self) -> Option<&bool> {
        self.tcp_ipv6_hash_enabled.as_ref()
    }

    /// Sets the value of TcpIPv6HashSupported
    pub fn set_tcp_ipv6_hash_supported(&mut self, value: bool) {
        self.tcp_ipv6_hash_supported = Some(value);
    }

    /// Gets the value of TcpIPv6HashSupported
    pub fn get_tcp_ipv6_hash_supported(&self) -> Option<&bool> {
        self.tcp_ipv6_hash_supported.as_ref()
    }

    /// Sets the value of ToeplitzHashFunctionEnabled
    pub fn set_toeplitz_hash_function_enabled(&mut self, value: bool) {
        self.toeplitz_hash_function_enabled = Some(value);
    }

    /// Gets the value of ToeplitzHashFunctionEnabled
    pub fn get_toeplitz_hash_function_enabled(&self) -> Option<&bool> {
        self.toeplitz_hash_function_enabled.as_ref()
    }

    /// Sets the value of ToeplitzHashFunctionSupported
    pub fn set_toeplitz_hash_function_supported(&mut self, value: bool) {
        self.toeplitz_hash_function_supported = Some(value);
    }

    /// Gets the value of ToeplitzHashFunctionSupported
    pub fn get_toeplitz_hash_function_supported(&self) -> Option<&bool> {
        self.toeplitz_hash_function_supported.as_ref()
    }

    /// Sets the value of UdpIPv4HashEnabled
    pub fn set_udp_ipv4_hash_enabled(&mut self, value: bool) {
        self.udp_ipv4_hash_enabled = Some(value);
    }

    /// Gets the value of UdpIPv4HashEnabled
    pub fn get_udp_ipv4_hash_enabled(&self) -> Option<&bool> {
        self.udp_ipv4_hash_enabled.as_ref()
    }

    /// Sets the value of UdpIPv4HashSupported
    pub fn set_udp_ipv4_hash_supported(&mut self, value: bool) {
        self.udp_ipv4_hash_supported = Some(value);
    }

    /// Gets the value of UdpIPv4HashSupported
    pub fn get_udp_ipv4_hash_supported(&self) -> Option<&bool> {
        self.udp_ipv4_hash_supported.as_ref()
    }

    /// Sets the value of UdpIPv6ExtensionHashEnabled
    pub fn set_udp_ipv6_extension_hash_enabled(&mut self, value: bool) {
        self.udp_ipv6_extension_hash_enabled = Some(value);
    }

    /// Gets the value of UdpIPv6ExtensionHashEnabled
    pub fn get_udp_ipv6_extension_hash_enabled(&self) -> Option<&bool> {
        self.udp_ipv6_extension_hash_enabled.as_ref()
    }

    /// Sets the value of UdpIPv6ExtensionHashSupported
    pub fn set_udp_ipv6_extension_hash_supported(&mut self, value: bool) {
        self.udp_ipv6_extension_hash_supported = Some(value);
    }

    /// Gets the value of UdpIPv6ExtensionHashSupported
    pub fn get_udp_ipv6_extension_hash_supported(&self) -> Option<&bool> {
        self.udp_ipv6_extension_hash_supported.as_ref()
    }

    /// Sets the value of UdpIPv6HashEnabled
    pub fn set_udp_ipv6_hash_enabled(&mut self, value: bool) {
        self.udp_ipv6_hash_enabled = Some(value);
    }

    /// Gets the value of UdpIPv6HashEnabled
    pub fn get_udp_ipv6_hash_enabled(&self) -> Option<&bool> {
        self.udp_ipv6_hash_enabled.as_ref()
    }

    /// Sets the value of UdpIPv6HashSupported
    pub fn set_udp_ipv6_hash_supported(&mut self, value: bool) {
        self.udp_ipv6_hash_supported = Some(value);
    }

    /// Gets the value of UdpIPv6HashSupported
    pub fn get_udp_ipv6_hash_supported(&self) -> Option<&bool> {
        self.udp_ipv6_hash_supported.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterRssSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapterRssSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterRssSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapterRssSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

