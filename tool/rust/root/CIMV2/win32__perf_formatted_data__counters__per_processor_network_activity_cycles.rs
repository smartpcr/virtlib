// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_PerProcessorNetworkActivityCycles struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_PerProcessorNetworkActivityCycles {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BuildScatterGatherCyclesPersec")]
    pub build_scatter_gather_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "InterruptCyclesPersec")]
    pub interrupt_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "InterruptDPCCyclesPersec")]
    pub interrupt_dpccycles_persec: Option<u64>,

/// 
    #[serde(rename = "InterruptDPCLatencyCyclesPersec")]
    pub interrupt_dpclatency_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "MiniportReturnPacketCyclesPersec")]
    pub miniport_return_packet_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "MiniportRSSIndirectionTableChangeCycles")]
    pub miniport_rssindirection_table_change_cycles: Option<u64>,

/// 
    #[serde(rename = "MiniportSendCyclesPersec")]
    pub miniport_send_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "NDISReceiveIndicationCyclesPersec")]
    pub ndisreceive_indication_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "NDISReturnPacketCyclesPersec")]
    pub ndisreturn_packet_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "NDISSendCompleteCyclesPersec")]
    pub ndissend_complete_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "NDISSendCyclesPersec")]
    pub ndissend_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "StackReceiveIndicationCyclesPersec")]
    pub stack_receive_indication_cycles_persec: Option<u64>,

/// 
    #[serde(rename = "StackSendCompleteCyclesPersec")]
    pub stack_send_complete_cycles_persec: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_PerProcessorNetworkActivityCycles {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            build_scatter_gather_cycles_persec: None,
            interrupt_cycles_persec: None,
            interrupt_dpccycles_persec: None,
            interrupt_dpclatency_cycles_persec: None,
            miniport_return_packet_cycles_persec: None,
            miniport_rssindirection_table_change_cycles: None,
            miniport_send_cycles_persec: None,
            ndisreceive_indication_cycles_persec: None,
            ndisreturn_packet_cycles_persec: None,
            ndissend_complete_cycles_persec: None,
            ndissend_cycles_persec: None,
            stack_receive_indication_cycles_persec: None,
            stack_send_complete_cycles_persec: None,
        }
    }


    /// Sets the value of BuildScatterGatherCyclesPersec
    pub fn set_build_scatter_gather_cycles_persec(&mut self, value: u64) {
        self.build_scatter_gather_cycles_persec = Some(value);
    }

    /// Gets the value of BuildScatterGatherCyclesPersec
    pub fn get_build_scatter_gather_cycles_persec(&self) -> Option<&u64> {
        self.build_scatter_gather_cycles_persec.as_ref()
    }

    /// Sets the value of InterruptCyclesPersec
    pub fn set_interrupt_cycles_persec(&mut self, value: u64) {
        self.interrupt_cycles_persec = Some(value);
    }

    /// Gets the value of InterruptCyclesPersec
    pub fn get_interrupt_cycles_persec(&self) -> Option<&u64> {
        self.interrupt_cycles_persec.as_ref()
    }

    /// Sets the value of InterruptDPCCyclesPersec
    pub fn set_interrupt_dpccycles_persec(&mut self, value: u64) {
        self.interrupt_dpccycles_persec = Some(value);
    }

    /// Gets the value of InterruptDPCCyclesPersec
    pub fn get_interrupt_dpccycles_persec(&self) -> Option<&u64> {
        self.interrupt_dpccycles_persec.as_ref()
    }

    /// Sets the value of InterruptDPCLatencyCyclesPersec
    pub fn set_interrupt_dpclatency_cycles_persec(&mut self, value: u64) {
        self.interrupt_dpclatency_cycles_persec = Some(value);
    }

    /// Gets the value of InterruptDPCLatencyCyclesPersec
    pub fn get_interrupt_dpclatency_cycles_persec(&self) -> Option<&u64> {
        self.interrupt_dpclatency_cycles_persec.as_ref()
    }

    /// Sets the value of MiniportReturnPacketCyclesPersec
    pub fn set_miniport_return_packet_cycles_persec(&mut self, value: u64) {
        self.miniport_return_packet_cycles_persec = Some(value);
    }

    /// Gets the value of MiniportReturnPacketCyclesPersec
    pub fn get_miniport_return_packet_cycles_persec(&self) -> Option<&u64> {
        self.miniport_return_packet_cycles_persec.as_ref()
    }

    /// Sets the value of MiniportRSSIndirectionTableChangeCycles
    pub fn set_miniport_rssindirection_table_change_cycles(&mut self, value: u64) {
        self.miniport_rssindirection_table_change_cycles = Some(value);
    }

    /// Gets the value of MiniportRSSIndirectionTableChangeCycles
    pub fn get_miniport_rssindirection_table_change_cycles(&self) -> Option<&u64> {
        self.miniport_rssindirection_table_change_cycles.as_ref()
    }

    /// Sets the value of MiniportSendCyclesPersec
    pub fn set_miniport_send_cycles_persec(&mut self, value: u64) {
        self.miniport_send_cycles_persec = Some(value);
    }

    /// Gets the value of MiniportSendCyclesPersec
    pub fn get_miniport_send_cycles_persec(&self) -> Option<&u64> {
        self.miniport_send_cycles_persec.as_ref()
    }

    /// Sets the value of NDISReceiveIndicationCyclesPersec
    pub fn set_ndisreceive_indication_cycles_persec(&mut self, value: u64) {
        self.ndisreceive_indication_cycles_persec = Some(value);
    }

    /// Gets the value of NDISReceiveIndicationCyclesPersec
    pub fn get_ndisreceive_indication_cycles_persec(&self) -> Option<&u64> {
        self.ndisreceive_indication_cycles_persec.as_ref()
    }

    /// Sets the value of NDISReturnPacketCyclesPersec
    pub fn set_ndisreturn_packet_cycles_persec(&mut self, value: u64) {
        self.ndisreturn_packet_cycles_persec = Some(value);
    }

    /// Gets the value of NDISReturnPacketCyclesPersec
    pub fn get_ndisreturn_packet_cycles_persec(&self) -> Option<&u64> {
        self.ndisreturn_packet_cycles_persec.as_ref()
    }

    /// Sets the value of NDISSendCompleteCyclesPersec
    pub fn set_ndissend_complete_cycles_persec(&mut self, value: u64) {
        self.ndissend_complete_cycles_persec = Some(value);
    }

    /// Gets the value of NDISSendCompleteCyclesPersec
    pub fn get_ndissend_complete_cycles_persec(&self) -> Option<&u64> {
        self.ndissend_complete_cycles_persec.as_ref()
    }

    /// Sets the value of NDISSendCyclesPersec
    pub fn set_ndissend_cycles_persec(&mut self, value: u64) {
        self.ndissend_cycles_persec = Some(value);
    }

    /// Gets the value of NDISSendCyclesPersec
    pub fn get_ndissend_cycles_persec(&self) -> Option<&u64> {
        self.ndissend_cycles_persec.as_ref()
    }

    /// Sets the value of StackReceiveIndicationCyclesPersec
    pub fn set_stack_receive_indication_cycles_persec(&mut self, value: u64) {
        self.stack_receive_indication_cycles_persec = Some(value);
    }

    /// Gets the value of StackReceiveIndicationCyclesPersec
    pub fn get_stack_receive_indication_cycles_persec(&self) -> Option<&u64> {
        self.stack_receive_indication_cycles_persec.as_ref()
    }

    /// Sets the value of StackSendCompleteCyclesPersec
    pub fn set_stack_send_complete_cycles_persec(&mut self, value: u64) {
        self.stack_send_complete_cycles_persec = Some(value);
    }

    /// Gets the value of StackSendCompleteCyclesPersec
    pub fn get_stack_send_complete_cycles_persec(&self) -> Option<&u64> {
        self.stack_send_complete_cycles_persec.as_ref()
    }
}

