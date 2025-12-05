// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_RequestTimeStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_RequestTimeStatistics {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "AverageProcessingTime")]
    pub average_processing_time: Option<u32>,

/// 
    #[serde(rename = "CID")]
    pub cid: Option<u16>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Name of the iSCSI Target
    #[serde(rename = "iSCSIName")]
    pub i_scsiname: Option<String>,

/// 
    #[serde(rename = "MaximumProcessingTime")]
    pub maximum_processing_time: Option<u32>,

/// 
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,

/// 
    #[serde(rename = "USID")]
    pub usid: Option<u64>,
}

impl MSiSCSI_RequestTimeStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active: None,
            average_processing_time: None,
            cid: None,
            instance_name: None,
            i_scsiname: None,
            maximum_processing_time: None,
            unique_adapter_id: None,
            usid: None,
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

    /// Sets the value of AverageProcessingTime
    pub fn set_average_processing_time(&mut self, value: u32) {
        self.average_processing_time = Some(value);
    }

    /// Gets the value of AverageProcessingTime
    pub fn get_average_processing_time(&self) -> Option<&u32> {
        self.average_processing_time.as_ref()
    }

    /// Sets the value of CID
    pub fn set_cid(&mut self, value: u16) {
        self.cid = Some(value);
    }

    /// Gets the value of CID
    pub fn get_cid(&self) -> Option<&u16> {
        self.cid.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of iSCSIName
    pub fn set_i_scsiname(&mut self, value: String) {
        self.i_scsiname = Some(value);
    }

    /// Gets the value of iSCSIName
    pub fn get_i_scsiname(&self) -> Option<&String> {
        self.i_scsiname.as_ref()
    }

    /// Sets the value of MaximumProcessingTime
    pub fn set_maximum_processing_time(&mut self, value: u32) {
        self.maximum_processing_time = Some(value);
    }

    /// Gets the value of MaximumProcessingTime
    pub fn get_maximum_processing_time(&self) -> Option<&u32> {
        self.maximum_processing_time.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }

    /// Sets the value of USID
    pub fn set_usid(&mut self, value: u64) {
        self.usid = Some(value);
    }

    /// Gets the value of USID
    pub fn get_usid(&self) -> Option<&u64> {
        self.usid.as_ref()
    }
}

