// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_SMSvcHost4000_SMSvcHost4000 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_SMSvcHost4000_SMSvcHost4000 {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ConnectionsAcceptedovernetpipe")]
    pub connections_acceptedovernetpipe: Option<u32>,

/// 
    #[serde(rename = "ConnectionsAcceptedovernettcp")]
    pub connections_acceptedovernettcp: Option<u32>,

/// 
    #[serde(rename = "ConnectionsDispatchedovernetpipe")]
    pub connections_dispatchedovernetpipe: Option<u32>,

/// 
    #[serde(rename = "ConnectionsDispatchedovernettcp")]
    pub connections_dispatchedovernettcp: Option<u32>,

/// 
    #[serde(rename = "DispatchFailuresovernetpipe")]
    pub dispatch_failuresovernetpipe: Option<u32>,

/// 
    #[serde(rename = "DispatchFailuresovernettcp")]
    pub dispatch_failuresovernettcp: Option<u32>,

/// 
    #[serde(rename = "ProtocolFailuresovernetpipe")]
    pub protocol_failuresovernetpipe: Option<u32>,

/// 
    #[serde(rename = "ProtocolFailuresovernettcp")]
    pub protocol_failuresovernettcp: Option<u32>,

/// 
    #[serde(rename = "RegistrationsActivefornetpipe")]
    pub registrations_activefornetpipe: Option<u32>,

/// 
    #[serde(rename = "RegistrationsActivefornettcp")]
    pub registrations_activefornettcp: Option<u32>,

/// 
    #[serde(rename = "UrisRegisteredfornetpipe")]
    pub uris_registeredfornetpipe: Option<u32>,

/// 
    #[serde(rename = "UrisRegisteredfornettcp")]
    pub uris_registeredfornettcp: Option<u32>,

/// 
    #[serde(rename = "UrisUnregisteredfornetpipe")]
    pub uris_unregisteredfornetpipe: Option<u32>,

/// 
    #[serde(rename = "UrisUnregisteredfornettcp")]
    pub uris_unregisteredfornettcp: Option<u32>,
}

impl Win32_PerfFormattedData_SMSvcHost4000_SMSvcHost4000 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            connections_acceptedovernetpipe: None,
            connections_acceptedovernettcp: None,
            connections_dispatchedovernetpipe: None,
            connections_dispatchedovernettcp: None,
            dispatch_failuresovernetpipe: None,
            dispatch_failuresovernettcp: None,
            protocol_failuresovernetpipe: None,
            protocol_failuresovernettcp: None,
            registrations_activefornetpipe: None,
            registrations_activefornettcp: None,
            uris_registeredfornetpipe: None,
            uris_registeredfornettcp: None,
            uris_unregisteredfornetpipe: None,
            uris_unregisteredfornettcp: None,
        }
    }


    /// Sets the value of ConnectionsAcceptedovernetpipe
    pub fn set_connections_acceptedovernetpipe(&mut self, value: u32) {
        self.connections_acceptedovernetpipe = Some(value);
    }

    /// Gets the value of ConnectionsAcceptedovernetpipe
    pub fn get_connections_acceptedovernetpipe(&self) -> Option<&u32> {
        self.connections_acceptedovernetpipe.as_ref()
    }

    /// Sets the value of ConnectionsAcceptedovernettcp
    pub fn set_connections_acceptedovernettcp(&mut self, value: u32) {
        self.connections_acceptedovernettcp = Some(value);
    }

    /// Gets the value of ConnectionsAcceptedovernettcp
    pub fn get_connections_acceptedovernettcp(&self) -> Option<&u32> {
        self.connections_acceptedovernettcp.as_ref()
    }

    /// Sets the value of ConnectionsDispatchedovernetpipe
    pub fn set_connections_dispatchedovernetpipe(&mut self, value: u32) {
        self.connections_dispatchedovernetpipe = Some(value);
    }

    /// Gets the value of ConnectionsDispatchedovernetpipe
    pub fn get_connections_dispatchedovernetpipe(&self) -> Option<&u32> {
        self.connections_dispatchedovernetpipe.as_ref()
    }

    /// Sets the value of ConnectionsDispatchedovernettcp
    pub fn set_connections_dispatchedovernettcp(&mut self, value: u32) {
        self.connections_dispatchedovernettcp = Some(value);
    }

    /// Gets the value of ConnectionsDispatchedovernettcp
    pub fn get_connections_dispatchedovernettcp(&self) -> Option<&u32> {
        self.connections_dispatchedovernettcp.as_ref()
    }

    /// Sets the value of DispatchFailuresovernetpipe
    pub fn set_dispatch_failuresovernetpipe(&mut self, value: u32) {
        self.dispatch_failuresovernetpipe = Some(value);
    }

    /// Gets the value of DispatchFailuresovernetpipe
    pub fn get_dispatch_failuresovernetpipe(&self) -> Option<&u32> {
        self.dispatch_failuresovernetpipe.as_ref()
    }

    /// Sets the value of DispatchFailuresovernettcp
    pub fn set_dispatch_failuresovernettcp(&mut self, value: u32) {
        self.dispatch_failuresovernettcp = Some(value);
    }

    /// Gets the value of DispatchFailuresovernettcp
    pub fn get_dispatch_failuresovernettcp(&self) -> Option<&u32> {
        self.dispatch_failuresovernettcp.as_ref()
    }

    /// Sets the value of ProtocolFailuresovernetpipe
    pub fn set_protocol_failuresovernetpipe(&mut self, value: u32) {
        self.protocol_failuresovernetpipe = Some(value);
    }

    /// Gets the value of ProtocolFailuresovernetpipe
    pub fn get_protocol_failuresovernetpipe(&self) -> Option<&u32> {
        self.protocol_failuresovernetpipe.as_ref()
    }

    /// Sets the value of ProtocolFailuresovernettcp
    pub fn set_protocol_failuresovernettcp(&mut self, value: u32) {
        self.protocol_failuresovernettcp = Some(value);
    }

    /// Gets the value of ProtocolFailuresovernettcp
    pub fn get_protocol_failuresovernettcp(&self) -> Option<&u32> {
        self.protocol_failuresovernettcp.as_ref()
    }

    /// Sets the value of RegistrationsActivefornetpipe
    pub fn set_registrations_activefornetpipe(&mut self, value: u32) {
        self.registrations_activefornetpipe = Some(value);
    }

    /// Gets the value of RegistrationsActivefornetpipe
    pub fn get_registrations_activefornetpipe(&self) -> Option<&u32> {
        self.registrations_activefornetpipe.as_ref()
    }

    /// Sets the value of RegistrationsActivefornettcp
    pub fn set_registrations_activefornettcp(&mut self, value: u32) {
        self.registrations_activefornettcp = Some(value);
    }

    /// Gets the value of RegistrationsActivefornettcp
    pub fn get_registrations_activefornettcp(&self) -> Option<&u32> {
        self.registrations_activefornettcp.as_ref()
    }

    /// Sets the value of UrisRegisteredfornetpipe
    pub fn set_uris_registeredfornetpipe(&mut self, value: u32) {
        self.uris_registeredfornetpipe = Some(value);
    }

    /// Gets the value of UrisRegisteredfornetpipe
    pub fn get_uris_registeredfornetpipe(&self) -> Option<&u32> {
        self.uris_registeredfornetpipe.as_ref()
    }

    /// Sets the value of UrisRegisteredfornettcp
    pub fn set_uris_registeredfornettcp(&mut self, value: u32) {
        self.uris_registeredfornettcp = Some(value);
    }

    /// Gets the value of UrisRegisteredfornettcp
    pub fn get_uris_registeredfornettcp(&self) -> Option<&u32> {
        self.uris_registeredfornettcp.as_ref()
    }

    /// Sets the value of UrisUnregisteredfornetpipe
    pub fn set_uris_unregisteredfornetpipe(&mut self, value: u32) {
        self.uris_unregisteredfornetpipe = Some(value);
    }

    /// Gets the value of UrisUnregisteredfornetpipe
    pub fn get_uris_unregisteredfornetpipe(&self) -> Option<&u32> {
        self.uris_unregisteredfornetpipe.as_ref()
    }

    /// Sets the value of UrisUnregisteredfornettcp
    pub fn set_uris_unregisteredfornettcp(&mut self, value: u32) {
        self.uris_unregisteredfornettcp = Some(value);
    }

    /// Gets the value of UrisUnregisteredfornettcp
    pub fn get_uris_unregisteredfornettcp(&self) -> Option<&u32> {
        self.uris_unregisteredfornettcp.as_ref()
    }
}

