// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_NetworkDirectAdapterInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_NetworkDirectAdapterInfo {
    #[serde(flatten)]
    pub base: MSNdis,

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
    #[serde(rename = "MaxCalleeData")]
    pub max_callee_data: Option<u32>,

/// 
    #[serde(rename = "MaxCallerData")]
    pub max_caller_data: Option<u32>,

/// 
    #[serde(rename = "MaxCqDepth")]
    pub max_cq_depth: Option<u32>,

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
    #[serde(rename = "MaxSrqDepth")]
    pub max_srq_depth: Option<u32>,

/// 
    #[serde(rename = "MaxTransferLength")]
    pub max_transfer_length: Option<u32>,

/// 
    #[serde(rename = "MaxWindowSize")]
    pub max_window_size: Option<u64>,

/// 
    #[serde(rename = "SupportsCQResize")]
    pub supports_cqresize: Option<bool>,

/// 
    #[serde(rename = "SupportsLoopbackConnections")]
    pub supports_loopback_connections: Option<bool>,

/// 
    #[serde(rename = "VendorId")]
    pub vendor_id: Option<u32>,

/// 
    #[serde(rename = "Version")]
    pub version: Option<MSNdis_NetworkDirectVersion>,
}

impl MSNdis_NetworkDirectAdapterInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            device_id: None,
            frmrpage_count: None,
            in_order_dma: None,
            large_request_threshold: None,
            max_callee_data: None,
            max_caller_data: None,
            max_cq_depth: None,
            max_inbound_read_limit: None,
            max_initiator_queue_depth: None,
            max_initiator_request_sge: None,
            max_inline_data_size: None,
            max_outbound_read_limit: None,
            max_read_request_sge: None,
            max_receive_queue_depth: None,
            max_receive_request_sge: None,
            max_registration_size: None,
            max_srq_depth: None,
            max_transfer_length: None,
            max_window_size: None,
            supports_cqresize: None,
            supports_loopback_connections: None,
            vendor_id: None,
            version: None,
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

    /// Sets the value of MaxCqDepth
    pub fn set_max_cq_depth(&mut self, value: u32) {
        self.max_cq_depth = Some(value);
    }

    /// Gets the value of MaxCqDepth
    pub fn get_max_cq_depth(&self) -> Option<&u32> {
        self.max_cq_depth.as_ref()
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

    /// Sets the value of MaxSrqDepth
    pub fn set_max_srq_depth(&mut self, value: u32) {
        self.max_srq_depth = Some(value);
    }

    /// Gets the value of MaxSrqDepth
    pub fn get_max_srq_depth(&self) -> Option<&u32> {
        self.max_srq_depth.as_ref()
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

    /// Sets the value of SupportsCQResize
    pub fn set_supports_cqresize(&mut self, value: bool) {
        self.supports_cqresize = Some(value);
    }

    /// Gets the value of SupportsCQResize
    pub fn get_supports_cqresize(&self) -> Option<&bool> {
        self.supports_cqresize.as_ref()
    }

    /// Sets the value of SupportsLoopbackConnections
    pub fn set_supports_loopback_connections(&mut self, value: bool) {
        self.supports_loopback_connections = Some(value);
    }

    /// Gets the value of SupportsLoopbackConnections
    pub fn get_supports_loopback_connections(&self) -> Option<&bool> {
        self.supports_loopback_connections.as_ref()
    }

    /// Sets the value of VendorId
    pub fn set_vendor_id(&mut self, value: u32) {
        self.vendor_id = Some(value);
    }

    /// Gets the value of VendorId
    pub fn get_vendor_id(&self) -> Option<&u32> {
        self.vendor_id.as_ref()
    }

    /// Sets the value of Version
    pub fn set_version(&mut self, value: MSNdis_NetworkDirectVersion) {
        self.version = Some(value);
    }

    /// Gets the value of Version
    pub fn get_version(&self) -> Option<&MSNdis_NetworkDirectVersion> {
        self.version.as_ref()
    }
}

