// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerEventDetail struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerEventDetail {

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u32>,

/// 
    #[serde(rename = "Level")]
    pub level: Option<u16>,

/// 
    #[serde(rename = "Log")]
    pub log: Option<String>,

/// 
    #[serde(rename = "QueryFileId")]
    pub query_file_id: Option<i32>,

/// 
    #[serde(rename = "RecordId")]
    pub record_id: Option<u32>,

/// 
    #[serde(rename = "Source")]
    pub source: Option<String>,

/// 
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<u64>,
}

impl MSFT_ServerEventDetail {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            description: None,
            id: None,
            level: None,
            log: None,
            query_file_id: None,
            record_id: None,
            source: None,
            timestamp: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u32) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u32> {
        self.id.as_ref()
    }

    /// Sets the value of Level
    pub fn set_level(&mut self, value: u16) {
        self.level = Some(value);
    }

    /// Gets the value of Level
    pub fn get_level(&self) -> Option<&u16> {
        self.level.as_ref()
    }

    /// Sets the value of Log
    pub fn set_log(&mut self, value: String) {
        self.log = Some(value);
    }

    /// Gets the value of Log
    pub fn get_log(&self) -> Option<&String> {
        self.log.as_ref()
    }

    /// Sets the value of QueryFileId
    pub fn set_query_file_id(&mut self, value: i32) {
        self.query_file_id = Some(value);
    }

    /// Gets the value of QueryFileId
    pub fn get_query_file_id(&self) -> Option<&i32> {
        self.query_file_id.as_ref()
    }

    /// Sets the value of RecordId
    pub fn set_record_id(&mut self, value: u32) {
        self.record_id = Some(value);
    }

    /// Gets the value of RecordId
    pub fn get_record_id(&self) -> Option<&u32> {
        self.record_id.as_ref()
    }

    /// Sets the value of Source
    pub fn set_source(&mut self, value: String) {
        self.source = Some(value);
    }

    /// Gets the value of Source
    pub fn get_source(&self) -> Option<&String> {
        self.source.as_ref()
    }

    /// Sets the value of Timestamp
    pub fn set_timestamp(&mut self, value: u64) {
        self.timestamp = Some(value);
    }

    /// Gets the value of Timestamp
    pub fn get_timestamp(&self) -> Option<&u64> {
        self.timestamp.as_ref()
    }
}

