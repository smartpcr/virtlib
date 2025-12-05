// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSParallel_DeviceBytesTransferred struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSParallel_DeviceBytesTransferred {
    #[serde(flatten)]
    pub base: MSParallel,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BoundedEcpReadCount")]
    pub bounded_ecp_read_count: Option<i64>,

/// 
    #[serde(rename = "BoundedEcpWriteCount")]
    pub bounded_ecp_write_count: Option<i64>,

/// 
    #[serde(rename = "ByteReadCount")]
    pub byte_read_count: Option<i64>,

/// 
    #[serde(rename = "ChannelNibbleReadCount")]
    pub channel_nibble_read_count: Option<i64>,

/// 
    #[serde(rename = "Flags1")]
    pub flags1: Option<u32>,

/// 
    #[serde(rename = "Flags2")]
    pub flags2: Option<u32>,

/// 
    #[serde(rename = "HwEcpReadCount")]
    pub hw_ecp_read_count: Option<i64>,

/// 
    #[serde(rename = "HwEcpWriteCount")]
    pub hw_ecp_write_count: Option<i64>,

/// 
    #[serde(rename = "HwEppReadCount")]
    pub hw_epp_read_count: Option<i64>,

/// 
    #[serde(rename = "HwEppWriteCount")]
    pub hw_epp_write_count: Option<i64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NibbleReadCount")]
    pub nibble_read_count: Option<i64>,

/// 
    #[serde(rename = "spare")]
    pub spare: Vec<u32>,

/// 
    #[serde(rename = "SppWriteCount")]
    pub spp_write_count: Option<i64>,

/// 
    #[serde(rename = "SwEcpReadCount")]
    pub sw_ecp_read_count: Option<i64>,

/// 
    #[serde(rename = "SwEcpWriteCount")]
    pub sw_ecp_write_count: Option<i64>,

/// 
    #[serde(rename = "SwEppReadCount")]
    pub sw_epp_read_count: Option<i64>,

/// 
    #[serde(rename = "SwEppWriteCount")]
    pub sw_epp_write_count: Option<i64>,
}

impl MSParallel_DeviceBytesTransferred {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSParallel::new(),
            active: None,
            bounded_ecp_read_count: None,
            bounded_ecp_write_count: None,
            byte_read_count: None,
            channel_nibble_read_count: None,
            flags1: None,
            flags2: None,
            hw_ecp_read_count: None,
            hw_ecp_write_count: None,
            hw_epp_read_count: None,
            hw_epp_write_count: None,
            instance_name: None,
            nibble_read_count: None,
            spare: Vec::new(),
            spp_write_count: None,
            sw_ecp_read_count: None,
            sw_ecp_write_count: None,
            sw_epp_read_count: None,
            sw_epp_write_count: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of BoundedEcpReadCount
    pub fn set_bounded_ecp_read_count(&mut self, value: i64) {
        self.bounded_ecp_read_count = Some(value);
    }

    /// Gets the value of BoundedEcpReadCount
    pub fn get_bounded_ecp_read_count(&self) -> Option<&i64> {
        self.bounded_ecp_read_count.as_ref()
    }

    /// Sets the value of BoundedEcpWriteCount
    pub fn set_bounded_ecp_write_count(&mut self, value: i64) {
        self.bounded_ecp_write_count = Some(value);
    }

    /// Gets the value of BoundedEcpWriteCount
    pub fn get_bounded_ecp_write_count(&self) -> Option<&i64> {
        self.bounded_ecp_write_count.as_ref()
    }

    /// Sets the value of ByteReadCount
    pub fn set_byte_read_count(&mut self, value: i64) {
        self.byte_read_count = Some(value);
    }

    /// Gets the value of ByteReadCount
    pub fn get_byte_read_count(&self) -> Option<&i64> {
        self.byte_read_count.as_ref()
    }

    /// Sets the value of ChannelNibbleReadCount
    pub fn set_channel_nibble_read_count(&mut self, value: i64) {
        self.channel_nibble_read_count = Some(value);
    }

    /// Gets the value of ChannelNibbleReadCount
    pub fn get_channel_nibble_read_count(&self) -> Option<&i64> {
        self.channel_nibble_read_count.as_ref()
    }

    /// Sets the value of Flags1
    pub fn set_flags1(&mut self, value: u32) {
        self.flags1 = Some(value);
    }

    /// Gets the value of Flags1
    pub fn get_flags1(&self) -> Option<&u32> {
        self.flags1.as_ref()
    }

    /// Sets the value of Flags2
    pub fn set_flags2(&mut self, value: u32) {
        self.flags2 = Some(value);
    }

    /// Gets the value of Flags2
    pub fn get_flags2(&self) -> Option<&u32> {
        self.flags2.as_ref()
    }

    /// Sets the value of HwEcpReadCount
    pub fn set_hw_ecp_read_count(&mut self, value: i64) {
        self.hw_ecp_read_count = Some(value);
    }

    /// Gets the value of HwEcpReadCount
    pub fn get_hw_ecp_read_count(&self) -> Option<&i64> {
        self.hw_ecp_read_count.as_ref()
    }

    /// Sets the value of HwEcpWriteCount
    pub fn set_hw_ecp_write_count(&mut self, value: i64) {
        self.hw_ecp_write_count = Some(value);
    }

    /// Gets the value of HwEcpWriteCount
    pub fn get_hw_ecp_write_count(&self) -> Option<&i64> {
        self.hw_ecp_write_count.as_ref()
    }

    /// Sets the value of HwEppReadCount
    pub fn set_hw_epp_read_count(&mut self, value: i64) {
        self.hw_epp_read_count = Some(value);
    }

    /// Gets the value of HwEppReadCount
    pub fn get_hw_epp_read_count(&self) -> Option<&i64> {
        self.hw_epp_read_count.as_ref()
    }

    /// Sets the value of HwEppWriteCount
    pub fn set_hw_epp_write_count(&mut self, value: i64) {
        self.hw_epp_write_count = Some(value);
    }

    /// Gets the value of HwEppWriteCount
    pub fn get_hw_epp_write_count(&self) -> Option<&i64> {
        self.hw_epp_write_count.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NibbleReadCount
    pub fn set_nibble_read_count(&mut self, value: i64) {
        self.nibble_read_count = Some(value);
    }

    /// Gets the value of NibbleReadCount
    pub fn get_nibble_read_count(&self) -> Option<&i64> {
        self.nibble_read_count.as_ref()
    }

    /// Sets the value of spare
    pub fn set_spare(&mut self, value: Vec<u32>) {
        self.spare = value;
    }

    /// Gets the value of spare
    pub fn get_spare(&self) -> &Vec<u32> {
        &self.spare
    }

    /// Sets the value of SppWriteCount
    pub fn set_spp_write_count(&mut self, value: i64) {
        self.spp_write_count = Some(value);
    }

    /// Gets the value of SppWriteCount
    pub fn get_spp_write_count(&self) -> Option<&i64> {
        self.spp_write_count.as_ref()
    }

    /// Sets the value of SwEcpReadCount
    pub fn set_sw_ecp_read_count(&mut self, value: i64) {
        self.sw_ecp_read_count = Some(value);
    }

    /// Gets the value of SwEcpReadCount
    pub fn get_sw_ecp_read_count(&self) -> Option<&i64> {
        self.sw_ecp_read_count.as_ref()
    }

    /// Sets the value of SwEcpWriteCount
    pub fn set_sw_ecp_write_count(&mut self, value: i64) {
        self.sw_ecp_write_count = Some(value);
    }

    /// Gets the value of SwEcpWriteCount
    pub fn get_sw_ecp_write_count(&self) -> Option<&i64> {
        self.sw_ecp_write_count.as_ref()
    }

    /// Sets the value of SwEppReadCount
    pub fn set_sw_epp_read_count(&mut self, value: i64) {
        self.sw_epp_read_count = Some(value);
    }

    /// Gets the value of SwEppReadCount
    pub fn get_sw_epp_read_count(&self) -> Option<&i64> {
        self.sw_epp_read_count.as_ref()
    }

    /// Sets the value of SwEppWriteCount
    pub fn set_sw_epp_write_count(&mut self, value: i64) {
        self.sw_epp_write_count = Some(value);
    }

    /// Gets the value of SwEppWriteCount
    pub fn get_sw_epp_write_count(&self) -> Option<&i64> {
        self.sw_epp_write_count.as_ref()
    }
}

