// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_PerProcessorNetworkInterfaceCardActivity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_PerProcessorNetworkInterfaceCardActivity {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "BuildScatterGatherListCallsPersec")]
    pub build_scatter_gather_list_calls_persec: Option<u64>,

/// 
    #[serde(rename = "DPCsDeferredPersec")]
    pub dpcs_deferred_persec: Option<u64>,

/// 
    #[serde(rename = "DPCsQueuedonOtherCPUsPersec")]
    pub dpcs_queuedon_other_cpus_persec: Option<u64>,

/// 
    #[serde(rename = "DPCsQueuedPersec")]
    pub dpcs_queued_persec: Option<u64>,

/// 
    #[serde(rename = "InterruptsPersec")]
    pub interrupts_persec: Option<u64>,

/// 
    #[serde(rename = "LowResourceReceivedPacketsPersec")]
    pub low_resource_received_packets_persec: Option<u64>,

/// 
    #[serde(rename = "LowResourceReceiveIndicationsPersec")]
    pub low_resource_receive_indications_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsCoalescedPersec")]
    pub packets_coalesced_persec: Option<u64>,

/// 
    #[serde(rename = "ReceivedPacketsPersec")]
    pub received_packets_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiveIndicationsPersec")]
    pub receive_indications_persec: Option<u64>,

/// 
    #[serde(rename = "ReturnedPacketsPersec")]
    pub returned_packets_persec: Option<u64>,

/// 
    #[serde(rename = "ReturnPacketCallsPersec")]
    pub return_packet_calls_persec: Option<u64>,

/// 
    #[serde(rename = "RSSIndirectionTableChangeCallsPersec")]
    pub rssindirection_table_change_calls_persec: Option<u64>,

/// 
    #[serde(rename = "SendCompleteCallsPersec")]
    pub send_complete_calls_persec: Option<u64>,

/// 
    #[serde(rename = "SendRequestCallsPersec")]
    pub send_request_calls_persec: Option<u64>,

/// 
    #[serde(rename = "SentCompletePacketsPersec")]
    pub sent_complete_packets_persec: Option<u64>,

/// 
    #[serde(rename = "SentPacketsPersec")]
    pub sent_packets_persec: Option<u64>,

/// 
    #[serde(rename = "TcpOffloadReceivebytesPersec")]
    pub tcp_offload_receivebytes_persec: Option<u64>,

/// 
    #[serde(rename = "TcpOffloadReceiveIndicationsPersec")]
    pub tcp_offload_receive_indications_persec: Option<u64>,

/// 
    #[serde(rename = "TcpOffloadSendbytesPersec")]
    pub tcp_offload_sendbytes_persec: Option<u64>,

/// 
    #[serde(rename = "TcpOffloadSendRequestCallsPersec")]
    pub tcp_offload_send_request_calls_persec: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_PerProcessorNetworkInterfaceCardActivity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            build_scatter_gather_list_calls_persec: None,
            dpcs_deferred_persec: None,
            dpcs_queuedon_other_cpus_persec: None,
            dpcs_queued_persec: None,
            interrupts_persec: None,
            low_resource_received_packets_persec: None,
            low_resource_receive_indications_persec: None,
            packets_coalesced_persec: None,
            received_packets_persec: None,
            receive_indications_persec: None,
            returned_packets_persec: None,
            return_packet_calls_persec: None,
            rssindirection_table_change_calls_persec: None,
            send_complete_calls_persec: None,
            send_request_calls_persec: None,
            sent_complete_packets_persec: None,
            sent_packets_persec: None,
            tcp_offload_receivebytes_persec: None,
            tcp_offload_receive_indications_persec: None,
            tcp_offload_sendbytes_persec: None,
            tcp_offload_send_request_calls_persec: None,
        }
    }


    /// Sets the value of BuildScatterGatherListCallsPersec
    pub fn set_build_scatter_gather_list_calls_persec(&mut self, value: u64) {
        self.build_scatter_gather_list_calls_persec = Some(value);
    }

    /// Gets the value of BuildScatterGatherListCallsPersec
    pub fn get_build_scatter_gather_list_calls_persec(&self) -> Option<&u64> {
        self.build_scatter_gather_list_calls_persec.as_ref()
    }

    /// Sets the value of DPCsDeferredPersec
    pub fn set_dpcs_deferred_persec(&mut self, value: u64) {
        self.dpcs_deferred_persec = Some(value);
    }

    /// Gets the value of DPCsDeferredPersec
    pub fn get_dpcs_deferred_persec(&self) -> Option<&u64> {
        self.dpcs_deferred_persec.as_ref()
    }

    /// Sets the value of DPCsQueuedonOtherCPUsPersec
    pub fn set_dpcs_queuedon_other_cpus_persec(&mut self, value: u64) {
        self.dpcs_queuedon_other_cpus_persec = Some(value);
    }

    /// Gets the value of DPCsQueuedonOtherCPUsPersec
    pub fn get_dpcs_queuedon_other_cpus_persec(&self) -> Option<&u64> {
        self.dpcs_queuedon_other_cpus_persec.as_ref()
    }

    /// Sets the value of DPCsQueuedPersec
    pub fn set_dpcs_queued_persec(&mut self, value: u64) {
        self.dpcs_queued_persec = Some(value);
    }

    /// Gets the value of DPCsQueuedPersec
    pub fn get_dpcs_queued_persec(&self) -> Option<&u64> {
        self.dpcs_queued_persec.as_ref()
    }

    /// Sets the value of InterruptsPersec
    pub fn set_interrupts_persec(&mut self, value: u64) {
        self.interrupts_persec = Some(value);
    }

    /// Gets the value of InterruptsPersec
    pub fn get_interrupts_persec(&self) -> Option<&u64> {
        self.interrupts_persec.as_ref()
    }

    /// Sets the value of LowResourceReceivedPacketsPersec
    pub fn set_low_resource_received_packets_persec(&mut self, value: u64) {
        self.low_resource_received_packets_persec = Some(value);
    }

    /// Gets the value of LowResourceReceivedPacketsPersec
    pub fn get_low_resource_received_packets_persec(&self) -> Option<&u64> {
        self.low_resource_received_packets_persec.as_ref()
    }

    /// Sets the value of LowResourceReceiveIndicationsPersec
    pub fn set_low_resource_receive_indications_persec(&mut self, value: u64) {
        self.low_resource_receive_indications_persec = Some(value);
    }

    /// Gets the value of LowResourceReceiveIndicationsPersec
    pub fn get_low_resource_receive_indications_persec(&self) -> Option<&u64> {
        self.low_resource_receive_indications_persec.as_ref()
    }

    /// Sets the value of PacketsCoalescedPersec
    pub fn set_packets_coalesced_persec(&mut self, value: u64) {
        self.packets_coalesced_persec = Some(value);
    }

    /// Gets the value of PacketsCoalescedPersec
    pub fn get_packets_coalesced_persec(&self) -> Option<&u64> {
        self.packets_coalesced_persec.as_ref()
    }

    /// Sets the value of ReceivedPacketsPersec
    pub fn set_received_packets_persec(&mut self, value: u64) {
        self.received_packets_persec = Some(value);
    }

    /// Gets the value of ReceivedPacketsPersec
    pub fn get_received_packets_persec(&self) -> Option<&u64> {
        self.received_packets_persec.as_ref()
    }

    /// Sets the value of ReceiveIndicationsPersec
    pub fn set_receive_indications_persec(&mut self, value: u64) {
        self.receive_indications_persec = Some(value);
    }

    /// Gets the value of ReceiveIndicationsPersec
    pub fn get_receive_indications_persec(&self) -> Option<&u64> {
        self.receive_indications_persec.as_ref()
    }

    /// Sets the value of ReturnedPacketsPersec
    pub fn set_returned_packets_persec(&mut self, value: u64) {
        self.returned_packets_persec = Some(value);
    }

    /// Gets the value of ReturnedPacketsPersec
    pub fn get_returned_packets_persec(&self) -> Option<&u64> {
        self.returned_packets_persec.as_ref()
    }

    /// Sets the value of ReturnPacketCallsPersec
    pub fn set_return_packet_calls_persec(&mut self, value: u64) {
        self.return_packet_calls_persec = Some(value);
    }

    /// Gets the value of ReturnPacketCallsPersec
    pub fn get_return_packet_calls_persec(&self) -> Option<&u64> {
        self.return_packet_calls_persec.as_ref()
    }

    /// Sets the value of RSSIndirectionTableChangeCallsPersec
    pub fn set_rssindirection_table_change_calls_persec(&mut self, value: u64) {
        self.rssindirection_table_change_calls_persec = Some(value);
    }

    /// Gets the value of RSSIndirectionTableChangeCallsPersec
    pub fn get_rssindirection_table_change_calls_persec(&self) -> Option<&u64> {
        self.rssindirection_table_change_calls_persec.as_ref()
    }

    /// Sets the value of SendCompleteCallsPersec
    pub fn set_send_complete_calls_persec(&mut self, value: u64) {
        self.send_complete_calls_persec = Some(value);
    }

    /// Gets the value of SendCompleteCallsPersec
    pub fn get_send_complete_calls_persec(&self) -> Option<&u64> {
        self.send_complete_calls_persec.as_ref()
    }

    /// Sets the value of SendRequestCallsPersec
    pub fn set_send_request_calls_persec(&mut self, value: u64) {
        self.send_request_calls_persec = Some(value);
    }

    /// Gets the value of SendRequestCallsPersec
    pub fn get_send_request_calls_persec(&self) -> Option<&u64> {
        self.send_request_calls_persec.as_ref()
    }

    /// Sets the value of SentCompletePacketsPersec
    pub fn set_sent_complete_packets_persec(&mut self, value: u64) {
        self.sent_complete_packets_persec = Some(value);
    }

    /// Gets the value of SentCompletePacketsPersec
    pub fn get_sent_complete_packets_persec(&self) -> Option<&u64> {
        self.sent_complete_packets_persec.as_ref()
    }

    /// Sets the value of SentPacketsPersec
    pub fn set_sent_packets_persec(&mut self, value: u64) {
        self.sent_packets_persec = Some(value);
    }

    /// Gets the value of SentPacketsPersec
    pub fn get_sent_packets_persec(&self) -> Option<&u64> {
        self.sent_packets_persec.as_ref()
    }

    /// Sets the value of TcpOffloadReceivebytesPersec
    pub fn set_tcp_offload_receivebytes_persec(&mut self, value: u64) {
        self.tcp_offload_receivebytes_persec = Some(value);
    }

    /// Gets the value of TcpOffloadReceivebytesPersec
    pub fn get_tcp_offload_receivebytes_persec(&self) -> Option<&u64> {
        self.tcp_offload_receivebytes_persec.as_ref()
    }

    /// Sets the value of TcpOffloadReceiveIndicationsPersec
    pub fn set_tcp_offload_receive_indications_persec(&mut self, value: u64) {
        self.tcp_offload_receive_indications_persec = Some(value);
    }

    /// Gets the value of TcpOffloadReceiveIndicationsPersec
    pub fn get_tcp_offload_receive_indications_persec(&self) -> Option<&u64> {
        self.tcp_offload_receive_indications_persec.as_ref()
    }

    /// Sets the value of TcpOffloadSendbytesPersec
    pub fn set_tcp_offload_sendbytes_persec(&mut self, value: u64) {
        self.tcp_offload_sendbytes_persec = Some(value);
    }

    /// Gets the value of TcpOffloadSendbytesPersec
    pub fn get_tcp_offload_sendbytes_persec(&self) -> Option<&u64> {
        self.tcp_offload_sendbytes_persec.as_ref()
    }

    /// Sets the value of TcpOffloadSendRequestCallsPersec
    pub fn set_tcp_offload_send_request_calls_persec(&mut self, value: u64) {
        self.tcp_offload_send_request_calls_persec = Some(value);
    }

    /// Gets the value of TcpOffloadSendRequestCallsPersec
    pub fn get_tcp_offload_send_request_calls_persec(&self) -> Option<&u64> {
        self.tcp_offload_send_request_calls_persec.as_ref()
    }
}

