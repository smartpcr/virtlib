// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterRdmaSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterRdmaSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// 
    #[serde(rename = "ETS")]
    pub ets: Option<u32>,

/// 
    #[serde(rename = "MaxCompletionQueueCount")]
    pub max_completion_queue_count: Option<u32>,

/// 
    #[serde(rename = "MaxInboundReadLimit")]
    pub max_inbound_read_limit: Option<u32>,

/// 
    #[serde(rename = "MaxMemoryRegionCount")]
    pub max_memory_region_count: Option<u32>,

/// 
    #[serde(rename = "MaxMemoryWindowCount")]
    pub max_memory_window_count: Option<u32>,

/// 
    #[serde(rename = "MaxOutboundReadLimit")]
    pub max_outbound_read_limit: Option<u32>,

/// 
    #[serde(rename = "MaxProtectionDomainCount")]
    pub max_protection_domain_count: Option<u32>,

/// 
    #[serde(rename = "MaxQueuePairCount")]
    pub max_queue_pair_count: Option<u32>,

/// 
    #[serde(rename = "MaxSharedReceiveQueueCount")]
    pub max_shared_receive_queue_count: Option<u32>,

/// 
    #[serde(rename = "OperationalState")]
    pub operational_state: Option<bool>,

/// 
    #[serde(rename = "PFC")]
    pub pfc: Option<u32>,

/// 
    #[serde(rename = "RdmaAdapterInfo")]
    pub rdma_adapter_info: Option<MSFT_NetAdapter_RdmaAdapterInfo>,

/// 
    #[serde(rename = "RdmaMissingCounterInfo")]
    pub rdma_missing_counter_info: Option<MSFT_NetAdapter_RdmaMissingCounterInfo>,
}

impl MSFT_NetAdapterRdmaSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            enabled: None,
            ets: None,
            max_completion_queue_count: None,
            max_inbound_read_limit: None,
            max_memory_region_count: None,
            max_memory_window_count: None,
            max_outbound_read_limit: None,
            max_protection_domain_count: None,
            max_queue_pair_count: None,
            max_shared_receive_queue_count: None,
            operational_state: None,
            pfc: None,
            rdma_adapter_info: None,
            rdma_missing_counter_info: None,
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

    /// Sets the value of ETS
    pub fn set_ets(&mut self, value: u32) {
        self.ets = Some(value);
    }

    /// Gets the value of ETS
    pub fn get_ets(&self) -> Option<&u32> {
        self.ets.as_ref()
    }

    /// Sets the value of MaxCompletionQueueCount
    pub fn set_max_completion_queue_count(&mut self, value: u32) {
        self.max_completion_queue_count = Some(value);
    }

    /// Gets the value of MaxCompletionQueueCount
    pub fn get_max_completion_queue_count(&self) -> Option<&u32> {
        self.max_completion_queue_count.as_ref()
    }

    /// Sets the value of MaxInboundReadLimit
    pub fn set_max_inbound_read_limit(&mut self, value: u32) {
        self.max_inbound_read_limit = Some(value);
    }

    /// Gets the value of MaxInboundReadLimit
    pub fn get_max_inbound_read_limit(&self) -> Option<&u32> {
        self.max_inbound_read_limit.as_ref()
    }

    /// Sets the value of MaxMemoryRegionCount
    pub fn set_max_memory_region_count(&mut self, value: u32) {
        self.max_memory_region_count = Some(value);
    }

    /// Gets the value of MaxMemoryRegionCount
    pub fn get_max_memory_region_count(&self) -> Option<&u32> {
        self.max_memory_region_count.as_ref()
    }

    /// Sets the value of MaxMemoryWindowCount
    pub fn set_max_memory_window_count(&mut self, value: u32) {
        self.max_memory_window_count = Some(value);
    }

    /// Gets the value of MaxMemoryWindowCount
    pub fn get_max_memory_window_count(&self) -> Option<&u32> {
        self.max_memory_window_count.as_ref()
    }

    /// Sets the value of MaxOutboundReadLimit
    pub fn set_max_outbound_read_limit(&mut self, value: u32) {
        self.max_outbound_read_limit = Some(value);
    }

    /// Gets the value of MaxOutboundReadLimit
    pub fn get_max_outbound_read_limit(&self) -> Option<&u32> {
        self.max_outbound_read_limit.as_ref()
    }

    /// Sets the value of MaxProtectionDomainCount
    pub fn set_max_protection_domain_count(&mut self, value: u32) {
        self.max_protection_domain_count = Some(value);
    }

    /// Gets the value of MaxProtectionDomainCount
    pub fn get_max_protection_domain_count(&self) -> Option<&u32> {
        self.max_protection_domain_count.as_ref()
    }

    /// Sets the value of MaxQueuePairCount
    pub fn set_max_queue_pair_count(&mut self, value: u32) {
        self.max_queue_pair_count = Some(value);
    }

    /// Gets the value of MaxQueuePairCount
    pub fn get_max_queue_pair_count(&self) -> Option<&u32> {
        self.max_queue_pair_count.as_ref()
    }

    /// Sets the value of MaxSharedReceiveQueueCount
    pub fn set_max_shared_receive_queue_count(&mut self, value: u32) {
        self.max_shared_receive_queue_count = Some(value);
    }

    /// Gets the value of MaxSharedReceiveQueueCount
    pub fn get_max_shared_receive_queue_count(&self) -> Option<&u32> {
        self.max_shared_receive_queue_count.as_ref()
    }

    /// Sets the value of OperationalState
    pub fn set_operational_state(&mut self, value: bool) {
        self.operational_state = Some(value);
    }

    /// Gets the value of OperationalState
    pub fn get_operational_state(&self) -> Option<&bool> {
        self.operational_state.as_ref()
    }

    /// Sets the value of PFC
    pub fn set_pfc(&mut self, value: u32) {
        self.pfc = Some(value);
    }

    /// Gets the value of PFC
    pub fn get_pfc(&self) -> Option<&u32> {
        self.pfc.as_ref()
    }

    /// Sets the value of RdmaAdapterInfo
    pub fn set_rdma_adapter_info(&mut self, value: MSFT_NetAdapter_RdmaAdapterInfo) {
        self.rdma_adapter_info = Some(value);
    }

    /// Gets the value of RdmaAdapterInfo
    pub fn get_rdma_adapter_info(&self) -> Option<&MSFT_NetAdapter_RdmaAdapterInfo> {
        self.rdma_adapter_info.as_ref()
    }

    /// Sets the value of RdmaMissingCounterInfo
    pub fn set_rdma_missing_counter_info(&mut self, value: MSFT_NetAdapter_RdmaMissingCounterInfo) {
        self.rdma_missing_counter_info = Some(value);
    }

    /// Gets the value of RdmaMissingCounterInfo
    pub fn get_rdma_missing_counter_info(&self) -> Option<&MSFT_NetAdapter_RdmaMissingCounterInfo> {
        self.rdma_missing_counter_info.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterRdmaSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapterRdmaSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterRdmaSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapterRdmaSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

