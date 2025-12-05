// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_NetworkDirectAdapterCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_NetworkDirectAdapterCapabilities {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "MaxCqCount")]
    pub max_cq_count: Option<u32>,

/// 
    #[serde(rename = "MaxInboundReadLimit")]
    pub max_inbound_read_limit: Option<u32>,

/// 
    #[serde(rename = "MaxMrCount")]
    pub max_mr_count: Option<u32>,

/// 
    #[serde(rename = "MaxMwCount")]
    pub max_mw_count: Option<u32>,

/// 
    #[serde(rename = "MaxOutboundReadLimit")]
    pub max_outbound_read_limit: Option<u32>,

/// 
    #[serde(rename = "MaxPdCount")]
    pub max_pd_count: Option<u32>,

/// 
    #[serde(rename = "MaxQpCount")]
    pub max_qp_count: Option<u32>,

/// 
    #[serde(rename = "MaxSrqCount")]
    pub max_srq_count: Option<u32>,

/// 
    #[serde(rename = "MissingCounterMask")]
    pub missing_counter_mask: Option<u64>,

/// 
    #[serde(rename = "NdAdapterInfo")]
    pub nd_adapter_info: Option<MSNdis_NetworkDirectAdapterInfo>,
}

impl MSNdis_NetworkDirectAdapterCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            max_cq_count: None,
            max_inbound_read_limit: None,
            max_mr_count: None,
            max_mw_count: None,
            max_outbound_read_limit: None,
            max_pd_count: None,
            max_qp_count: None,
            max_srq_count: None,
            missing_counter_mask: None,
            nd_adapter_info: None,
        }
    }


    /// Sets the value of MaxCqCount
    pub fn set_max_cq_count(&mut self, value: u32) {
        self.max_cq_count = Some(value);
    }

    /// Gets the value of MaxCqCount
    pub fn get_max_cq_count(&self) -> Option<&u32> {
        self.max_cq_count.as_ref()
    }

    /// Sets the value of MaxInboundReadLimit
    pub fn set_max_inbound_read_limit(&mut self, value: u32) {
        self.max_inbound_read_limit = Some(value);
    }

    /// Gets the value of MaxInboundReadLimit
    pub fn get_max_inbound_read_limit(&self) -> Option<&u32> {
        self.max_inbound_read_limit.as_ref()
    }

    /// Sets the value of MaxMrCount
    pub fn set_max_mr_count(&mut self, value: u32) {
        self.max_mr_count = Some(value);
    }

    /// Gets the value of MaxMrCount
    pub fn get_max_mr_count(&self) -> Option<&u32> {
        self.max_mr_count.as_ref()
    }

    /// Sets the value of MaxMwCount
    pub fn set_max_mw_count(&mut self, value: u32) {
        self.max_mw_count = Some(value);
    }

    /// Gets the value of MaxMwCount
    pub fn get_max_mw_count(&self) -> Option<&u32> {
        self.max_mw_count.as_ref()
    }

    /// Sets the value of MaxOutboundReadLimit
    pub fn set_max_outbound_read_limit(&mut self, value: u32) {
        self.max_outbound_read_limit = Some(value);
    }

    /// Gets the value of MaxOutboundReadLimit
    pub fn get_max_outbound_read_limit(&self) -> Option<&u32> {
        self.max_outbound_read_limit.as_ref()
    }

    /// Sets the value of MaxPdCount
    pub fn set_max_pd_count(&mut self, value: u32) {
        self.max_pd_count = Some(value);
    }

    /// Gets the value of MaxPdCount
    pub fn get_max_pd_count(&self) -> Option<&u32> {
        self.max_pd_count.as_ref()
    }

    /// Sets the value of MaxQpCount
    pub fn set_max_qp_count(&mut self, value: u32) {
        self.max_qp_count = Some(value);
    }

    /// Gets the value of MaxQpCount
    pub fn get_max_qp_count(&self) -> Option<&u32> {
        self.max_qp_count.as_ref()
    }

    /// Sets the value of MaxSrqCount
    pub fn set_max_srq_count(&mut self, value: u32) {
        self.max_srq_count = Some(value);
    }

    /// Gets the value of MaxSrqCount
    pub fn get_max_srq_count(&self) -> Option<&u32> {
        self.max_srq_count.as_ref()
    }

    /// Sets the value of MissingCounterMask
    pub fn set_missing_counter_mask(&mut self, value: u64) {
        self.missing_counter_mask = Some(value);
    }

    /// Gets the value of MissingCounterMask
    pub fn get_missing_counter_mask(&self) -> Option<&u64> {
        self.missing_counter_mask.as_ref()
    }

    /// Sets the value of NdAdapterInfo
    pub fn set_nd_adapter_info(&mut self, value: MSNdis_NetworkDirectAdapterInfo) {
        self.nd_adapter_info = Some(value);
    }

    /// Gets the value of NdAdapterInfo
    pub fn get_nd_adapter_info(&self) -> Option<&MSNdis_NetworkDirectAdapterInfo> {
        self.nd_adapter_info.as_ref()
    }
}

