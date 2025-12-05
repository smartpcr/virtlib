// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterVmqSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterVmqSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "AnyVlanSupported")]
    pub any_vlan_supported: Option<bool>,

/// 
    #[serde(rename = "BaseProcessorGroup")]
    pub base_processor_group: Option<u16>,

/// 
    #[serde(rename = "BaseProcessorNumber")]
    pub base_processor_number: Option<u8>,

/// 
    #[serde(rename = "DynamicProcessorAffinityChangeSupported")]
    pub dynamic_processor_affinity_change_supported: Option<bool>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "InterruptVectorCoalescingSupported")]
    pub interrupt_vector_coalescing_supported: Option<bool>,

/// 
    #[serde(rename = "LookaheadSplitSupported")]
    pub lookahead_split_supported: Option<bool>,

/// 
    #[serde(rename = "MaxLookaheadSplitSize")]
    pub max_lookahead_split_size: Option<u32>,

/// 
    #[serde(rename = "MaxProcessorNumber")]
    pub max_processor_number: Option<u8>,

/// 
    #[serde(rename = "MaxProcessors")]
    pub max_processors: Option<u32>,

/// 
    #[serde(rename = "MinLookaheadSplitSize")]
    pub min_lookahead_split_size: Option<u32>,

/// 
    #[serde(rename = "NumaNode")]
    pub numa_node: Option<u16>,

/// 
    #[serde(rename = "NumberOfReceiveQueues")]
    pub number_of_receive_queues: Option<u32>,

/// 
    #[serde(rename = "NumMacAddressesPerPort")]
    pub num_mac_addresses_per_port: Option<u32>,

/// 
    #[serde(rename = "NumVlansPerPort")]
    pub num_vlans_per_port: Option<u32>,

/// 
    #[serde(rename = "TotalNumberOfMacAddresses")]
    pub total_number_of_mac_addresses: Option<u32>,

/// 
    #[serde(rename = "VlanFilteringSupported")]
    pub vlan_filtering_supported: Option<bool>,
}

impl MSFT_NetAdapterVmqSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            any_vlan_supported: None,
            base_processor_group: None,
            base_processor_number: None,
            dynamic_processor_affinity_change_supported: None,
            enabled: None,
            interrupt_vector_coalescing_supported: None,
            lookahead_split_supported: None,
            max_lookahead_split_size: None,
            max_processor_number: None,
            max_processors: None,
            min_lookahead_split_size: None,
            numa_node: None,
            number_of_receive_queues: None,
            num_mac_addresses_per_port: None,
            num_vlans_per_port: None,
            total_number_of_mac_addresses: None,
            vlan_filtering_supported: None,
        }
    }


    /// Sets the value of AnyVlanSupported
    pub fn set_any_vlan_supported(&mut self, value: bool) {
        self.any_vlan_supported = Some(value);
    }

    /// Gets the value of AnyVlanSupported
    pub fn get_any_vlan_supported(&self) -> Option<&bool> {
        self.any_vlan_supported.as_ref()
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

    /// Sets the value of DynamicProcessorAffinityChangeSupported
    pub fn set_dynamic_processor_affinity_change_supported(&mut self, value: bool) {
        self.dynamic_processor_affinity_change_supported = Some(value);
    }

    /// Gets the value of DynamicProcessorAffinityChangeSupported
    pub fn get_dynamic_processor_affinity_change_supported(&self) -> Option<&bool> {
        self.dynamic_processor_affinity_change_supported.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of InterruptVectorCoalescingSupported
    pub fn set_interrupt_vector_coalescing_supported(&mut self, value: bool) {
        self.interrupt_vector_coalescing_supported = Some(value);
    }

    /// Gets the value of InterruptVectorCoalescingSupported
    pub fn get_interrupt_vector_coalescing_supported(&self) -> Option<&bool> {
        self.interrupt_vector_coalescing_supported.as_ref()
    }

    /// Sets the value of LookaheadSplitSupported
    pub fn set_lookahead_split_supported(&mut self, value: bool) {
        self.lookahead_split_supported = Some(value);
    }

    /// Gets the value of LookaheadSplitSupported
    pub fn get_lookahead_split_supported(&self) -> Option<&bool> {
        self.lookahead_split_supported.as_ref()
    }

    /// Sets the value of MaxLookaheadSplitSize
    pub fn set_max_lookahead_split_size(&mut self, value: u32) {
        self.max_lookahead_split_size = Some(value);
    }

    /// Gets the value of MaxLookaheadSplitSize
    pub fn get_max_lookahead_split_size(&self) -> Option<&u32> {
        self.max_lookahead_split_size.as_ref()
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

    /// Sets the value of MinLookaheadSplitSize
    pub fn set_min_lookahead_split_size(&mut self, value: u32) {
        self.min_lookahead_split_size = Some(value);
    }

    /// Gets the value of MinLookaheadSplitSize
    pub fn get_min_lookahead_split_size(&self) -> Option<&u32> {
        self.min_lookahead_split_size.as_ref()
    }

    /// Sets the value of NumaNode
    pub fn set_numa_node(&mut self, value: u16) {
        self.numa_node = Some(value);
    }

    /// Gets the value of NumaNode
    pub fn get_numa_node(&self) -> Option<&u16> {
        self.numa_node.as_ref()
    }

    /// Sets the value of NumberOfReceiveQueues
    pub fn set_number_of_receive_queues(&mut self, value: u32) {
        self.number_of_receive_queues = Some(value);
    }

    /// Gets the value of NumberOfReceiveQueues
    pub fn get_number_of_receive_queues(&self) -> Option<&u32> {
        self.number_of_receive_queues.as_ref()
    }

    /// Sets the value of NumMacAddressesPerPort
    pub fn set_num_mac_addresses_per_port(&mut self, value: u32) {
        self.num_mac_addresses_per_port = Some(value);
    }

    /// Gets the value of NumMacAddressesPerPort
    pub fn get_num_mac_addresses_per_port(&self) -> Option<&u32> {
        self.num_mac_addresses_per_port.as_ref()
    }

    /// Sets the value of NumVlansPerPort
    pub fn set_num_vlans_per_port(&mut self, value: u32) {
        self.num_vlans_per_port = Some(value);
    }

    /// Gets the value of NumVlansPerPort
    pub fn get_num_vlans_per_port(&self) -> Option<&u32> {
        self.num_vlans_per_port.as_ref()
    }

    /// Sets the value of TotalNumberOfMacAddresses
    pub fn set_total_number_of_mac_addresses(&mut self, value: u32) {
        self.total_number_of_mac_addresses = Some(value);
    }

    /// Gets the value of TotalNumberOfMacAddresses
    pub fn get_total_number_of_mac_addresses(&self) -> Option<&u32> {
        self.total_number_of_mac_addresses.as_ref()
    }

    /// Sets the value of VlanFilteringSupported
    pub fn set_vlan_filtering_supported(&mut self, value: bool) {
        self.vlan_filtering_supported = Some(value);
    }

    /// Gets the value of VlanFilteringSupported
    pub fn get_vlan_filtering_supported(&self) -> Option<&bool> {
        self.vlan_filtering_supported.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterVmqSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapterVmqSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterVmqSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapterVmqSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

