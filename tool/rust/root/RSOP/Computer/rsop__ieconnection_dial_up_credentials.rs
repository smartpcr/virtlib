// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.RSOP.Computer
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RSOP_IEConnectionDialUpCredentials struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RSOP_IEConnectionDialUpCredentials {

/// 
    #[serde(rename = "callbackID")]
    pub callback_id: Option<u32>,

/// 
    #[serde(rename = "callbackNumber")]
    pub callback_number: Option<String>,

/// 
    #[serde(rename = "connectionName")]
    pub connection_name: Option<String>,

/// 
    #[serde(rename = "domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "entryName")]
    pub entry_name: Option<String>,

/// 
    #[serde(rename = "password")]
    pub password: Option<String>,

/// 
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,

/// 
    #[serde(rename = "rasDialParamsData")]
    pub ras_dial_params_data: Vec<u8>,

/// 
    #[serde(rename = "rasDialParamsDataSize")]
    pub ras_dial_params_data_size: Option<u32>,

/// 
    #[serde(rename = "rsopID")]
    pub rsop_id: Option<String>,

/// 
    #[serde(rename = "rsopPrecedence")]
    pub rsop_precedence: Option<i32>,

/// 
    #[serde(rename = "subEntry")]
    pub sub_entry: Option<u32>,

/// 
    #[serde(rename = "userName")]
    pub user_name: Option<String>,

/// 
    #[serde(rename = "windowsVersion")]
    pub windows_version: Option<u32>,
}

impl RSOP_IEConnectionDialUpCredentials {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            callback_id: None,
            callback_number: None,
            connection_name: None,
            domain: None,
            entry_name: None,
            password: None,
            phone_number: None,
            ras_dial_params_data: Vec::new(),
            ras_dial_params_data_size: None,
            rsop_id: None,
            rsop_precedence: None,
            sub_entry: None,
            user_name: None,
            windows_version: None,
        }
    }


    /// Sets the value of callbackID
    pub fn set_callback_id(&mut self, value: u32) {
        self.callback_id = Some(value);
    }

    /// Gets the value of callbackID
    pub fn get_callback_id(&self) -> Option<&u32> {
        self.callback_id.as_ref()
    }

    /// Sets the value of callbackNumber
    pub fn set_callback_number(&mut self, value: String) {
        self.callback_number = Some(value);
    }

    /// Gets the value of callbackNumber
    pub fn get_callback_number(&self) -> Option<&String> {
        self.callback_number.as_ref()
    }

    /// Sets the value of connectionName
    pub fn set_connection_name(&mut self, value: String) {
        self.connection_name = Some(value);
    }

    /// Gets the value of connectionName
    pub fn get_connection_name(&self) -> Option<&String> {
        self.connection_name.as_ref()
    }

    /// Sets the value of domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of entryName
    pub fn set_entry_name(&mut self, value: String) {
        self.entry_name = Some(value);
    }

    /// Gets the value of entryName
    pub fn get_entry_name(&self) -> Option<&String> {
        self.entry_name.as_ref()
    }

    /// Sets the value of password
    pub fn set_password(&mut self, value: String) {
        self.password = Some(value);
    }

    /// Gets the value of password
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }

    /// Sets the value of phoneNumber
    pub fn set_phone_number(&mut self, value: String) {
        self.phone_number = Some(value);
    }

    /// Gets the value of phoneNumber
    pub fn get_phone_number(&self) -> Option<&String> {
        self.phone_number.as_ref()
    }

    /// Sets the value of rasDialParamsData
    pub fn set_ras_dial_params_data(&mut self, value: Vec<u8>) {
        self.ras_dial_params_data = value;
    }

    /// Gets the value of rasDialParamsData
    pub fn get_ras_dial_params_data(&self) -> &Vec<u8> {
        &self.ras_dial_params_data
    }

    /// Sets the value of rasDialParamsDataSize
    pub fn set_ras_dial_params_data_size(&mut self, value: u32) {
        self.ras_dial_params_data_size = Some(value);
    }

    /// Gets the value of rasDialParamsDataSize
    pub fn get_ras_dial_params_data_size(&self) -> Option<&u32> {
        self.ras_dial_params_data_size.as_ref()
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

    /// Sets the value of subEntry
    pub fn set_sub_entry(&mut self, value: u32) {
        self.sub_entry = Some(value);
    }

    /// Gets the value of subEntry
    pub fn get_sub_entry(&self) -> Option<&u32> {
        self.sub_entry.as_ref()
    }

    /// Sets the value of userName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of userName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    /// Sets the value of windowsVersion
    pub fn set_windows_version(&mut self, value: u32) {
        self.windows_version = Some(value);
    }

    /// Gets the value of windowsVersion
    pub fn get_windows_version(&self) -> Option<&u32> {
        self.windows_version.as_ref()
    }
}

