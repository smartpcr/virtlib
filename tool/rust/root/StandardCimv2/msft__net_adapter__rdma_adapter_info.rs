// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_RdmaAdapterInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_RdmaAdapterInfo {

/// 
    #[serde(rename = "DeviceId")]
    pub device_id: Option<u32>,

/// 
    #[serde(rename = "FRMRPageCount")]
    pub frmrpage_count: Option<u32>,

/// 
    #[serde(rename = "InOrderDMA")]
    pub in_order_dma: Option<bool>,

/// 
    #[serde(rename = "LargeRequestThreshold")]
    pub large_request_threshold: Option<u32>,

/// 
    #[serde(rename = "MajorVersionNumber")]
    pub major_version_number: Option<u16>,

/// 
    #[serde(rename = "MaxCalleeData")]
    pub max_callee_data: Option<u32>,

/// 
    #[serde(rename = "MaxCallerData")]
    pub max_caller_data: Option<u32>,

/// 
    #[serde(rename = "MaxCompletionQueueDepth")]
    pub max_completion_queue_depth: Option<u32>,

/// 
    #[serde(rename = "MaxInboundReadLimit")]
    pub max_inbound_read_limit: Option<u32>,

/// 
    #[serde(rename = "MaxInitiatorQueueDepth")]
    pub max_initiator_queue_depth: Option<u32>,

/// 
    #[serde(rename = "MaxInitiatorRequestSge")]
    pub max_initiator_request_sge: Option<u32>,

/// 
    #[serde(rename = "MaxInlineDataSize")]
    pub max_inline_data_size: Option<u32>,

/// 
    #[serde(rename = "MaxOutboundReadLimit")]
    pub max_outbound_read_limit: Option<u32>,

/// 
    #[serde(rename = "MaxReadRequestSge")]
    pub max_read_request_sge: Option<u32>,

/// 
    #[serde(rename = "MaxReceiveQueueDepth")]
    pub max_receive_queue_depth: Option<u32>,

/// 
    #[serde(rename = "MaxReceiveRequestSge")]
    pub max_receive_request_sge: Option<u32>,

/// 
    #[serde(rename = "MaxRegistrationSize")]
    pub max_registration_size: Option<u64>,

/// 
    #[serde(rename = "MaxSharedReceiveQueueDepth")]
    pub max_shared_receive_queue_depth: Option<u32>,

/// 
    #[serde(rename = "MaxTransferLength")]
    pub max_transfer_length: Option<u32>,

/// 
    #[serde(rename = "MaxWindowSize")]
    pub max_window_size: Option<u64>,

/// 
    #[serde(rename = "MinorVersionNumber")]
    pub minor_version_number: Option<u16>,

/// 
    #[serde(rename = "RdmaReadSinkFlagNotRequired")]
    pub rdma_read_sink_flag_not_required: Option<bool>,

/// 
    #[serde(rename = "RdmaTechnology")]
    pub rdma_technology: Option<u32>,

/// 
    #[serde(rename = "SupportsCompletionQueueInterruptModeration")]
    pub supports_completion_queue_interrupt_moderation: Option<bool>,

/// 
    #[serde(rename = "SupportsCompletionQueueResize")]
    pub supports_completion_queue_resize: Option<bool>,

/// 
    #[serde(rename = "SupportsLoopbackConnections")]
    pub supports_loopback_connections: Option<bool>,

/// 
    #[serde(rename = "SupportsMultiEngine")]
    pub supports_multi_engine: Option<bool>,

/// 
    #[serde(rename = "VendorId")]
    pub vendor_id: Option<u32>,
}

impl MSFT_NetAdapter_RdmaAdapterInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            device_id: None,
            frmrpage_count: None,
            in_order_dma: None,
            large_request_threshold: None,
            major_version_number: None,
            max_callee_data: None,
            max_caller_data: None,
            max_completion_queue_depth: None,
            max_inbound_read_limit: None,
            max_initiator_queue_depth: None,
            max_initiator_request_sge: None,
            max_inline_data_size: None,
            max_outbound_read_limit: None,
            max_read_request_sge: None,
            max_receive_queue_depth: None,
            max_receive_request_sge: None,
            max_registration_size: None,
            max_shared_receive_queue_depth: None,
            max_transfer_length: None,
            max_window_size: None,
            minor_version_number: None,
            rdma_read_sink_flag_not_required: None,
            rdma_technology: None,
            supports_completion_queue_interrupt_moderation: None,
            supports_completion_queue_resize: None,
            supports_loopback_connections: None,
            supports_multi_engine: None,
            vendor_id: None,
        }
    }


    /// Sets the value of DeviceId
    pub fn set_device_id(&mut self, value: u32) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceId
    pub fn get_device_id(&self) -> Option<&u32> {
        self.device_id.as_ref()
    }

    /// Sets the value of FRMRPageCount
    pub fn set_frmrpage_count(&mut self, value: u32) {
        self.frmrpage_count = Some(value);
    }

    /// Gets the value of FRMRPageCount
    pub fn get_frmrpage_count(&self) -> Option<&u32> {
        self.frmrpage_count.as_ref()
    }

    /// Sets the value of InOrderDMA
    pub fn set_in_order_dma(&mut self, value: bool) {
        self.in_order_dma = Some(value);
    }

    /// Gets the value of InOrderDMA
    pub fn get_in_order_dma(&self) -> Option<&bool> {
        self.in_order_dma.as_ref()
    }

    /// Sets the value of LargeRequestThreshold
    pub fn set_large_request_threshold(&mut self, value: u32) {
        self.large_request_threshold = Some(value);
    }

    /// Gets the value of LargeRequestThreshold
    pub fn get_large_request_threshold(&self) -> Option<&u32> {
        self.large_request_threshold.as_ref()
    }

    /// Sets the value of MajorVersionNumber
    pub fn set_major_version_number(&mut self, value: u16) {
        self.major_version_number = Some(value);
    }

    /// Gets the value of MajorVersionNumber
    pub fn get_major_version_number(&self) -> Option<&u16> {
        self.major_version_number.as_ref()
    }

    /// Sets the value of MaxCalleeData
    pub fn set_max_callee_data(&mut self, value: u32) {
        self.max_callee_data = Some(value);
    }

    /// Gets the value of MaxCalleeData
    pub fn get_max_callee_data(&self) -> Option<&u32> {
        self.max_callee_data.as_ref()
    }

    /// Sets the value of MaxCallerData
    pub fn set_max_caller_data(&mut self, value: u32) {
        self.max_caller_data = Some(value);
    }

    /// Gets the value of MaxCallerData
    pub fn get_max_caller_data(&self) -> Option<&u32> {
        self.max_caller_data.as_ref()
    }

    /// Sets the value of MaxCompletionQueueDepth
    pub fn set_max_completion_queue_depth(&mut self, value: u32) {
        self.max_completion_queue_depth = Some(value);
    }

    /// Gets the value of MaxCompletionQueueDepth
    pub fn get_max_completion_queue_depth(&self) -> Option<&u32> {
        self.max_completion_queue_depth.as_ref()
    }

    /// Sets the value of MaxInboundReadLimit
    pub fn set_max_inbound_read_limit(&mut self, value: u32) {
        self.max_inbound_read_limit = Some(value);
    }

    /// Gets the value of MaxInboundReadLimit
    pub fn get_max_inbound_read_limit(&self) -> Option<&u32> {
        self.max_inbound_read_limit.as_ref()
    }

    /// Sets the value of MaxInitiatorQueueDepth
    pub fn set_max_initiator_queue_depth(&mut self, value: u32) {
        self.max_initiator_queue_depth = Some(value);
    }

    /// Gets the value of MaxInitiatorQueueDepth
    pub fn get_max_initiator_queue_depth(&self) -> Option<&u32> {
        self.max_initiator_queue_depth.as_ref()
    }

    /// Sets the value of MaxInitiatorRequestSge
    pub fn set_max_initiator_request_sge(&mut self, value: u32) {
        self.max_initiator_request_sge = Some(value);
    }

    /// Gets the value of MaxInitiatorRequestSge
    pub fn get_max_initiator_request_sge(&self) -> Option<&u32> {
        self.max_initiator_request_sge.as_ref()
    }

    /// Sets the value of MaxInlineDataSize
    pub fn set_max_inline_data_size(&mut self, value: u32) {
        self.max_inline_data_size = Some(value);
    }

    /// Gets the value of MaxInlineDataSize
    pub fn get_max_inline_data_size(&self) -> Option<&u32> {
        self.max_inline_data_size.as_ref()
    }

    /// Sets the value of MaxOutboundReadLimit
    pub fn set_max_outbound_read_limit(&mut self, value: u32) {
        self.max_outbound_read_limit = Some(value);
    }

    /// Gets the value of MaxOutboundReadLimit
    pub fn get_max_outbound_read_limit(&self) -> Option<&u32> {
        self.max_outbound_read_limit.as_ref()
    }

    /// Sets the value of MaxReadRequestSge
    pub fn set_max_read_request_sge(&mut self, value: u32) {
        self.max_read_request_sge = Some(value);
    }

    /// Gets the value of MaxReadRequestSge
    pub fn get_max_read_request_sge(&self) -> Option<&u32> {
        self.max_read_request_sge.as_ref()
    }

    /// Sets the value of MaxReceiveQueueDepth
    pub fn set_max_receive_queue_depth(&mut self, value: u32) {
        self.max_receive_queue_depth = Some(value);
    }

    /// Gets the value of MaxReceiveQueueDepth
    pub fn get_max_receive_queue_depth(&self) -> Option<&u32> {
        self.max_receive_queue_depth.as_ref()
    }

    /// Sets the value of MaxReceiveRequestSge
    pub fn set_max_receive_request_sge(&mut self, value: u32) {
        self.max_receive_request_sge = Some(value);
    }

    /// Gets the value of MaxReceiveRequestSge
    pub fn get_max_receive_request_sge(&self) -> Option<&u32> {
        self.max_receive_request_sge.as_ref()
    }

    /// Sets the value of MaxRegistrationSize
    pub fn set_max_registration_size(&mut self, value: u64) {
        self.max_registration_size = Some(value);
    }

    /// Gets the value of MaxRegistrationSize
    pub fn get_max_registration_size(&self) -> Option<&u64> {
        self.max_registration_size.as_ref()
    }

    /// Sets the value of MaxSharedReceiveQueueDepth
    pub fn set_max_shared_receive_queue_depth(&mut self, value: u32) {
        self.max_shared_receive_queue_depth = Some(value);
    }

    /// Gets the value of MaxSharedReceiveQueueDepth
    pub fn get_max_shared_receive_queue_depth(&self) -> Option<&u32> {
        self.max_shared_receive_queue_depth.as_ref()
    }

    /// Sets the value of MaxTransferLength
    pub fn set_max_transfer_length(&mut self, value: u32) {
        self.max_transfer_length = Some(value);
    }

    /// Gets the value of MaxTransferLength
    pub fn get_max_transfer_length(&self) -> Option<&u32> {
        self.max_transfer_length.as_ref()
    }

    /// Sets the value of MaxWindowSize
    pub fn set_max_window_size(&mut self, value: u64) {
        self.max_window_size = Some(value);
    }

    /// Gets the value of MaxWindowSize
    pub fn get_max_window_size(&self) -> Option<&u64> {
        self.max_window_size.as_ref()
    }

    /// Sets the value of MinorVersionNumber
    pub fn set_minor_version_number(&mut self, value: u16) {
        self.minor_version_number = Some(value);
    }

    /// Gets the value of MinorVersionNumber
    pub fn get_minor_version_number(&self) -> Option<&u16> {
        self.minor_version_number.as_ref()
    }

    /// Sets the value of RdmaReadSinkFlagNotRequired
    pub fn set_rdma_read_sink_flag_not_required(&mut self, value: bool) {
        self.rdma_read_sink_flag_not_required = Some(value);
    }

    /// Gets the value of RdmaReadSinkFlagNotRequired
    pub fn get_rdma_read_sink_flag_not_required(&self) -> Option<&bool> {
        self.rdma_read_sink_flag_not_required.as_ref()
    }

    /// Sets the value of RdmaTechnology
    pub fn set_rdma_technology(&mut self, value: u32) {
        self.rdma_technology = Some(value);
    }

    /// Gets the value of RdmaTechnology
    pub fn get_rdma_technology(&self) -> Option<&u32> {
        self.rdma_technology.as_ref()
    }

    /// Sets the value of SupportsCompletionQueueInterruptModeration
    pub fn set_supports_completion_queue_interrupt_moderation(&mut self, value: bool) {
        self.supports_completion_queue_interrupt_moderation = Some(value);
    }

    /// Gets the value of SupportsCompletionQueueInterruptModeration
    pub fn get_supports_completion_queue_interrupt_moderation(&self) -> Option<&bool> {
        self.supports_completion_queue_interrupt_moderation.as_ref()
    }

    /// Sets the value of SupportsCompletionQueueResize
    pub fn set_supports_completion_queue_resize(&mut self, value: bool) {
        self.supports_completion_queue_resize = Some(value);
    }

    /// Gets the value of SupportsCompletionQueueResize
    pub fn get_supports_completion_queue_resize(&self) -> Option<&bool> {
        self.supports_completion_queue_resize.as_ref()
    }

    /// Sets the value of SupportsLoopbackConnections
    pub fn set_supports_loopback_connections(&mut self, value: bool) {
        self.supports_loopback_connections = Some(value);
    }

    /// Gets the value of SupportsLoopbackConnections
    pub fn get_supports_loopback_connections(&self) -> Option<&bool> {
        self.supports_loopback_connections.as_ref()
    }

    /// Sets the value of SupportsMultiEngine
    pub fn set_supports_multi_engine(&mut self, value: bool) {
        self.supports_multi_engine = Some(value);
    }

    /// Gets the value of SupportsMultiEngine
    pub fn get_supports_multi_engine(&self) -> Option<&bool> {
        self.supports_multi_engine.as_ref()
    }

    /// Sets the value of VendorId
    pub fn set_vendor_id(&mut self, value: u32) {
        self.vendor_id = Some(value);
    }

    /// Gets the value of VendorId
    pub fn get_vendor_id(&self) -> Option<&u32> {
        self.vendor_id.as_ref()
    }
}

