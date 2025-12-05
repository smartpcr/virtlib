// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetOffloadGlobalSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetOffloadGlobalSetting {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "Chimney")]
    pub chimney: Option<u8>,

/// 
    #[serde(rename = "NetworkDirect")]
    pub network_direct: Option<u8>,

/// 
    #[serde(rename = "NetworkDirectAcrossIPSubnets")]
    pub network_direct_across_ipsubnets: Option<u8>,

/// 
    #[serde(rename = "PacketCoalescingFilter")]
    pub packet_coalescing_filter: Option<u8>,

/// 
    #[serde(rename = "ReceiveSegmentCoalescing")]
    pub receive_segment_coalescing: Option<u8>,

/// 
    #[serde(rename = "ReceiveSideScaling")]
    pub receive_side_scaling: Option<u8>,

/// 
    #[serde(rename = "TaskOffload")]
    pub task_offload: Option<u8>,
}

impl MSFT_NetOffloadGlobalSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            chimney: None,
            network_direct: None,
            network_direct_across_ipsubnets: None,
            packet_coalescing_filter: None,
            receive_segment_coalescing: None,
            receive_side_scaling: None,
            task_offload: None,
        }
    }


    /// Sets the value of Chimney
    pub fn set_chimney(&mut self, value: u8) {
        self.chimney = Some(value);
    }

    /// Gets the value of Chimney
    pub fn get_chimney(&self) -> Option<&u8> {
        self.chimney.as_ref()
    }

    /// Sets the value of NetworkDirect
    pub fn set_network_direct(&mut self, value: u8) {
        self.network_direct = Some(value);
    }

    /// Gets the value of NetworkDirect
    pub fn get_network_direct(&self) -> Option<&u8> {
        self.network_direct.as_ref()
    }

    /// Sets the value of NetworkDirectAcrossIPSubnets
    pub fn set_network_direct_across_ipsubnets(&mut self, value: u8) {
        self.network_direct_across_ipsubnets = Some(value);
    }

    /// Gets the value of NetworkDirectAcrossIPSubnets
    pub fn get_network_direct_across_ipsubnets(&self) -> Option<&u8> {
        self.network_direct_across_ipsubnets.as_ref()
    }

    /// Sets the value of PacketCoalescingFilter
    pub fn set_packet_coalescing_filter(&mut self, value: u8) {
        self.packet_coalescing_filter = Some(value);
    }

    /// Gets the value of PacketCoalescingFilter
    pub fn get_packet_coalescing_filter(&self) -> Option<&u8> {
        self.packet_coalescing_filter.as_ref()
    }

    /// Sets the value of ReceiveSegmentCoalescing
    pub fn set_receive_segment_coalescing(&mut self, value: u8) {
        self.receive_segment_coalescing = Some(value);
    }

    /// Gets the value of ReceiveSegmentCoalescing
    pub fn get_receive_segment_coalescing(&self) -> Option<&u8> {
        self.receive_segment_coalescing.as_ref()
    }

    /// Sets the value of ReceiveSideScaling
    pub fn set_receive_side_scaling(&mut self, value: u8) {
        self.receive_side_scaling = Some(value);
    }

    /// Gets the value of ReceiveSideScaling
    pub fn get_receive_side_scaling(&self) -> Option<&u8> {
        self.receive_side_scaling.as_ref()
    }

    /// Sets the value of TaskOffload
    pub fn set_task_offload(&mut self, value: u8) {
        self.task_offload = Some(value);
    }

    /// Gets the value of TaskOffload
    pub fn get_task_offload(&self) -> Option<&u8> {
        self.task_offload.as_ref()
    }
}

