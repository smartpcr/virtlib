// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Header_LastDroppedTimes_TypeGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Header_LastDroppedTimes_TypeGroup {
    #[serde(flatten)]
    pub base: EventTraceEvent,

/// 
    #[serde(rename = "Padding")]
    pub padding: Option<u32>,

/// 
    #[serde(rename = "TimeStamp")]
    pub time_stamp: Vec<u64>,

/// 
    #[serde(rename = "TimeStampCount")]
    pub time_stamp_count: Option<u32>,
}

impl Header_LastDroppedTimes_TypeGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTraceEvent::new(),
            padding: None,
            time_stamp: Vec::new(),
            time_stamp_count: None,
        }
    }


    /// Sets the value of Padding
    pub fn set_padding(&mut self, value: u32) {
        self.padding = Some(value);
    }

    /// Gets the value of Padding
    pub fn get_padding(&self) -> Option<&u32> {
        self.padding.as_ref()
    }

    /// Sets the value of TimeStamp
    pub fn set_time_stamp(&mut self, value: Vec<u64>) {
        self.time_stamp = value;
    }

    /// Gets the value of TimeStamp
    pub fn get_time_stamp(&self) -> &Vec<u64> {
        &self.time_stamp
    }

    /// Sets the value of TimeStampCount
    pub fn set_time_stamp_count(&mut self, value: u32) {
        self.time_stamp_count = Some(value);
    }

    /// Gets the value of TimeStampCount
    pub fn get_time_stamp_count(&self) -> Option<&u32> {
        self.time_stamp_count.as_ref()
    }
}

