// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEConnectionWinINetSettings struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEConnectionWinINetSettings {

/// 
    #[serde(rename = "connectionName")]
    pub connection_name: Option<String>,

/// 
    #[serde(rename = "internetPerConnOptionListData")]
    pub internet_per_conn_option_list_data: Vec<u8>,

/// 
    #[serde(rename = "internetPerConnOptionListDataSize")]
    pub internet_per_conn_option_list_data_size: Option<u32>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<i32>,
}

impl RSOP_IEConnectionWinINetSettings {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            connection_name: None,
            internet_per_conn_option_list_data: Vec::new(),
            internet_per_conn_option_list_data_size: None,
            rsop_id: None,
            rsop_precedence: None,
        }
    }


    /// Sets the value of connectionName
    pub fn set_connection_name(&mut self, value: String) {
        self.connection_name = Some(value);
    }

    /// Gets the value of connectionName
    pub fn get_connection_name(&self) -> Option<&String> {
        self.connection_name.as_ref()
    }

    /// Sets the value of internetPerConnOptionListData
    pub fn set_internet_per_conn_option_list_data(&mut self, value: Vec<u8>) {
        self.internet_per_conn_option_list_data = value;
    }

    /// Gets the value of internetPerConnOptionListData
    pub fn get_internet_per_conn_option_list_data(&self) -> &Vec<u8> {
        &self.internet_per_conn_option_list_data
    }

    /// Sets the value of internetPerConnOptionListDataSize
    pub fn set_internet_per_conn_option_list_data_size(&mut self, value: u32) {
        self.internet_per_conn_option_list_data_size = Some(value);
    }

    /// Gets the value of internetPerConnOptionListDataSize
    pub fn get_internet_per_conn_option_list_data_size(&self) -> Option<&u32> {
        self.internet_per_conn_option_list_data_size.as_ref()
    }

    /// Sets the value of rsopID
    pub fn set_rsop_id(&mut self, value: String) {
        self.rsop_id = Some(value);
    }

    /// Gets the value of rsopID
    pub fn get_rsop_id(&self) -> Option<&String> {
        self.rsop_id.as_ref()
    }

    /// Sets the value of rsopPrecedence
    pub fn set_rsop_precedence(&mut self, value: i32) {
        self.rsop_precedence = Some(value);
    }

    /// Gets the value of rsopPrecedence
    pub fn get_rsop_precedence(&self) -> Option<&i32> {
        self.rsop_precedence.as_ref()
    }
}

