// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ServerSession struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ServerSession {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "ActiveTime")]
    pub active_time: Option<u32>,

/// 
    #[serde(rename = "ClientType")]
    pub client_type: Option<String>,

/// 
    #[serde(rename = "ComputerName")]
    pub computer_name: Option<String>,

/// 
    #[serde(rename = "IdleTime")]
    pub idle_time: Option<u32>,

/// 
    #[serde(rename = "ResourcesOpened")]
    pub resources_opened: Option<u32>,

/// 
    #[serde(rename = "SessionType")]
    pub session_type: Option<u32>,

/// 
    #[serde(rename = "TransportName")]
    pub transport_name: Option<String>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,
}

impl Win32_ServerSession {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            active_time: None,
            client_type: None,
            computer_name: None,
            idle_time: None,
            resources_opened: None,
            session_type: None,
            transport_name: None,
            user_name: None,
        }
    }


    /// Sets the value of ActiveTime
    pub fn set_active_time(&mut self, value: u32) {
        self.active_time = Some(value);
    }

    /// Gets the value of ActiveTime
    pub fn get_active_time(&self) -> Option<&u32> {
        self.active_time.as_ref()
    }

    /// Sets the value of ClientType
    pub fn set_client_type(&mut self, value: String) {
        self.client_type = Some(value);
    }

    /// Gets the value of ClientType
    pub fn get_client_type(&self) -> Option<&String> {
        self.client_type.as_ref()
    }

    /// Sets the value of ComputerName
    pub fn set_computer_name(&mut self, value: String) {
        self.computer_name = Some(value);
    }

    /// Gets the value of ComputerName
    pub fn get_computer_name(&self) -> Option<&String> {
        self.computer_name.as_ref()
    }

    /// Sets the value of IdleTime
    pub fn set_idle_time(&mut self, value: u32) {
        self.idle_time = Some(value);
    }

    /// Gets the value of IdleTime
    pub fn get_idle_time(&self) -> Option<&u32> {
        self.idle_time.as_ref()
    }

    /// Sets the value of ResourcesOpened
    pub fn set_resources_opened(&mut self, value: u32) {
        self.resources_opened = Some(value);
    }

    /// Gets the value of ResourcesOpened
    pub fn get_resources_opened(&self) -> Option<&u32> {
        self.resources_opened.as_ref()
    }

    /// Sets the value of SessionType
    pub fn set_session_type(&mut self, value: u32) {
        self.session_type = Some(value);
    }

    /// Gets the value of SessionType
    pub fn get_session_type(&self) -> Option<&u32> {
        self.session_type.as_ref()
    }

    /// Sets the value of TransportName
    pub fn set_transport_name(&mut self, value: String) {
        self.transport_name = Some(value);
    }

    /// Gets the value of TransportName
    pub fn get_transport_name(&self) -> Option<&String> {
        self.transport_name.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }
}

