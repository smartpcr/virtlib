// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PingStatus struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PingStatus {

/// 
    #[serde(rename = "Address")]
    pub address: Option<String>,

/// 
    #[serde(rename = "BufferSize")]
    pub buffer_size: Option<u32>,

/// 
    #[serde(rename = "NoFragmentation")]
    pub no_fragmentation: Option<bool>,

/// 
    #[serde(rename = "PrimaryAddressResolutionStatus")]
    pub primary_address_resolution_status: Option<u32>,

/// 
    #[serde(rename = "ProtocolAddress")]
    pub protocol_address: Option<String>,

/// 
    #[serde(rename = "ProtocolAddressResolved")]
    pub protocol_address_resolved: Option<String>,

/// 
    #[serde(rename = "RecordRoute")]
    pub record_route: Option<u32>,

/// 
    #[serde(rename = "ReplyInconsistency")]
    pub reply_inconsistency: Option<bool>,

/// 
    #[serde(rename = "ReplySize")]
    pub reply_size: Option<u32>,

/// 
    #[serde(rename = "ResolveAddressNames")]
    pub resolve_address_names: Option<bool>,

/// 
    #[serde(rename = "ResponseTime")]
    pub response_time: Option<u32>,

/// 
    #[serde(rename = "ResponseTimeToLive")]
    pub response_time_to_live: Option<u32>,

/// 
    #[serde(rename = "RouteRecord")]
    pub route_record: Vec<String>,

/// 
    #[serde(rename = "RouteRecordResolved")]
    pub route_record_resolved: Vec<String>,

/// 
    #[serde(rename = "SourceRoute")]
    pub source_route: Option<String>,

/// 
    #[serde(rename = "SourceRouteType")]
    pub source_route_type: Option<u32>,

/// 
    #[serde(rename = "StatusCode")]
    pub status_code: Option<u32>,

/// 
    #[serde(rename = "Timeout")]
    pub timeout: Option<u32>,

/// 
    #[serde(rename = "TimeStampRecord")]
    pub time_stamp_record: Vec<u32>,

/// 
    #[serde(rename = "TimeStampRecordAddress")]
    pub time_stamp_record_address: Vec<String>,

/// 
    #[serde(rename = "TimeStampRecordAddressResolved")]
    pub time_stamp_record_address_resolved: Vec<String>,

/// 
    #[serde(rename = "TimestampRoute")]
    pub timestamp_route: Option<u32>,

/// 
    #[serde(rename = "TimeToLive")]
    pub time_to_live: Option<u32>,

/// 
    #[serde(rename = "TypeofService")]
    pub typeof_service: Option<u32>,
}

impl Win32_PingStatus {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            address: None,
            buffer_size: None,
            no_fragmentation: None,
            primary_address_resolution_status: None,
            protocol_address: None,
            protocol_address_resolved: None,
            record_route: None,
            reply_inconsistency: None,
            reply_size: None,
            resolve_address_names: None,
            response_time: None,
            response_time_to_live: None,
            route_record: Vec::new(),
            route_record_resolved: Vec::new(),
            source_route: None,
            source_route_type: None,
            status_code: None,
            timeout: None,
            time_stamp_record: Vec::new(),
            time_stamp_record_address: Vec::new(),
            time_stamp_record_address_resolved: Vec::new(),
            timestamp_route: None,
            time_to_live: None,
            typeof_service: None,
        }
    }


    /// Sets the value of Address
    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    /// Gets the value of Address
    pub fn get_address(&self) -> Option<&String> {
        self.address.as_ref()
    }

    /// Sets the value of BufferSize
    pub fn set_buffer_size(&mut self, value: u32) {
        self.buffer_size = Some(value);
    }

    /// Gets the value of BufferSize
    pub fn get_buffer_size(&self) -> Option<&u32> {
        self.buffer_size.as_ref()
    }

    /// Sets the value of NoFragmentation
    pub fn set_no_fragmentation(&mut self, value: bool) {
        self.no_fragmentation = Some(value);
    }

    /// Gets the value of NoFragmentation
    pub fn get_no_fragmentation(&self) -> Option<&bool> {
        self.no_fragmentation.as_ref()
    }

    /// Sets the value of PrimaryAddressResolutionStatus
    pub fn set_primary_address_resolution_status(&mut self, value: u32) {
        self.primary_address_resolution_status = Some(value);
    }

    /// Gets the value of PrimaryAddressResolutionStatus
    pub fn get_primary_address_resolution_status(&self) -> Option<&u32> {
        self.primary_address_resolution_status.as_ref()
    }

    /// Sets the value of ProtocolAddress
    pub fn set_protocol_address(&mut self, value: String) {
        self.protocol_address = Some(value);
    }

    /// Gets the value of ProtocolAddress
    pub fn get_protocol_address(&self) -> Option<&String> {
        self.protocol_address.as_ref()
    }

    /// Sets the value of ProtocolAddressResolved
    pub fn set_protocol_address_resolved(&mut self, value: String) {
        self.protocol_address_resolved = Some(value);
    }

    /// Gets the value of ProtocolAddressResolved
    pub fn get_protocol_address_resolved(&self) -> Option<&String> {
        self.protocol_address_resolved.as_ref()
    }

    /// Sets the value of RecordRoute
    pub fn set_record_route(&mut self, value: u32) {
        self.record_route = Some(value);
    }

    /// Gets the value of RecordRoute
    pub fn get_record_route(&self) -> Option<&u32> {
        self.record_route.as_ref()
    }

    /// Sets the value of ReplyInconsistency
    pub fn set_reply_inconsistency(&mut self, value: bool) {
        self.reply_inconsistency = Some(value);
    }

    /// Gets the value of ReplyInconsistency
    pub fn get_reply_inconsistency(&self) -> Option<&bool> {
        self.reply_inconsistency.as_ref()
    }

    /// Sets the value of ReplySize
    pub fn set_reply_size(&mut self, value: u32) {
        self.reply_size = Some(value);
    }

    /// Gets the value of ReplySize
    pub fn get_reply_size(&self) -> Option<&u32> {
        self.reply_size.as_ref()
    }

    /// Sets the value of ResolveAddressNames
    pub fn set_resolve_address_names(&mut self, value: bool) {
        self.resolve_address_names = Some(value);
    }

    /// Gets the value of ResolveAddressNames
    pub fn get_resolve_address_names(&self) -> Option<&bool> {
        self.resolve_address_names.as_ref()
    }

    /// Sets the value of ResponseTime
    pub fn set_response_time(&mut self, value: u32) {
        self.response_time = Some(value);
    }

    /// Gets the value of ResponseTime
    pub fn get_response_time(&self) -> Option<&u32> {
        self.response_time.as_ref()
    }

    /// Sets the value of ResponseTimeToLive
    pub fn set_response_time_to_live(&mut self, value: u32) {
        self.response_time_to_live = Some(value);
    }

    /// Gets the value of ResponseTimeToLive
    pub fn get_response_time_to_live(&self) -> Option<&u32> {
        self.response_time_to_live.as_ref()
    }

    /// Sets the value of RouteRecord
    pub fn set_route_record(&mut self, value: Vec<String>) {
        self.route_record = value;
    }

    /// Gets the value of RouteRecord
    pub fn get_route_record(&self) -> &Vec<String> {
        &self.route_record
    }

    /// Sets the value of RouteRecordResolved
    pub fn set_route_record_resolved(&mut self, value: Vec<String>) {
        self.route_record_resolved = value;
    }

    /// Gets the value of RouteRecordResolved
    pub fn get_route_record_resolved(&self) -> &Vec<String> {
        &self.route_record_resolved
    }

    /// Sets the value of SourceRoute
    pub fn set_source_route(&mut self, value: String) {
        self.source_route = Some(value);
    }

    /// Gets the value of SourceRoute
    pub fn get_source_route(&self) -> Option<&String> {
        self.source_route.as_ref()
    }

    /// Sets the value of SourceRouteType
    pub fn set_source_route_type(&mut self, value: u32) {
        self.source_route_type = Some(value);
    }

    /// Gets the value of SourceRouteType
    pub fn get_source_route_type(&self) -> Option<&u32> {
        self.source_route_type.as_ref()
    }

    /// Sets the value of StatusCode
    pub fn set_status_code(&mut self, value: u32) {
        self.status_code = Some(value);
    }

    /// Gets the value of StatusCode
    pub fn get_status_code(&self) -> Option<&u32> {
        self.status_code.as_ref()
    }

    /// Sets the value of Timeout
    pub fn set_timeout(&mut self, value: u32) {
        self.timeout = Some(value);
    }

    /// Gets the value of Timeout
    pub fn get_timeout(&self) -> Option<&u32> {
        self.timeout.as_ref()
    }

    /// Sets the value of TimeStampRecord
    pub fn set_time_stamp_record(&mut self, value: Vec<u32>) {
        self.time_stamp_record = value;
    }

    /// Gets the value of TimeStampRecord
    pub fn get_time_stamp_record(&self) -> &Vec<u32> {
        &self.time_stamp_record
    }

    /// Sets the value of TimeStampRecordAddress
    pub fn set_time_stamp_record_address(&mut self, value: Vec<String>) {
        self.time_stamp_record_address = value;
    }

    /// Gets the value of TimeStampRecordAddress
    pub fn get_time_stamp_record_address(&self) -> &Vec<String> {
        &self.time_stamp_record_address
    }

    /// Sets the value of TimeStampRecordAddressResolved
    pub fn set_time_stamp_record_address_resolved(&mut self, value: Vec<String>) {
        self.time_stamp_record_address_resolved = value;
    }

    /// Gets the value of TimeStampRecordAddressResolved
    pub fn get_time_stamp_record_address_resolved(&self) -> &Vec<String> {
        &self.time_stamp_record_address_resolved
    }

    /// Sets the value of TimestampRoute
    pub fn set_timestamp_route(&mut self, value: u32) {
        self.timestamp_route = Some(value);
    }

    /// Gets the value of TimestampRoute
    pub fn get_timestamp_route(&self) -> Option<&u32> {
        self.timestamp_route.as_ref()
    }

    /// Sets the value of TimeToLive
    pub fn set_time_to_live(&mut self, value: u32) {
        self.time_to_live = Some(value);
    }

    /// Gets the value of TimeToLive
    pub fn get_time_to_live(&self) -> Option<&u32> {
        self.time_to_live.as_ref()
    }

    /// Sets the value of TypeofService
    pub fn set_typeof_service(&mut self, value: u32) {
        self.typeof_service = Some(value);
    }

    /// Gets the value of TypeofService
    pub fn get_typeof_service(&self) -> Option<&u32> {
        self.typeof_service.as_ref()
    }
}

