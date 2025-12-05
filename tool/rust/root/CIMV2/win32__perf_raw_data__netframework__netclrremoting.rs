// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NETFramework_NETCLRRemoting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NETFramework_NETCLRRemoting {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Channels")]
    pub channels: Option<u32>,

/// 
    #[serde(rename = "ContextBoundClassesLoaded")]
    pub context_bound_classes_loaded: Option<u32>,

/// 
    #[serde(rename = "ContextBoundObjectsAllocPersec")]
    pub context_bound_objects_alloc_persec: Option<u32>,

/// 
    #[serde(rename = "ContextProxies")]
    pub context_proxies: Option<u32>,

/// 
    #[serde(rename = "Contexts")]
    pub contexts: Option<u32>,

/// 
    #[serde(rename = "RemoteCallsPersec")]
    pub remote_calls_persec: Option<u32>,

/// 
    #[serde(rename = "TotalRemoteCalls")]
    pub total_remote_calls: Option<u32>,
}

impl Win32_PerfRawData_NETFramework_NETCLRRemoting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            channels: None,
            context_bound_classes_loaded: None,
            context_bound_objects_alloc_persec: None,
            context_proxies: None,
            contexts: None,
            remote_calls_persec: None,
            total_remote_calls: None,
        }
    }


    /// Sets the value of Channels
    pub fn set_channels(&mut self, value: u32) {
        self.channels = Some(value);
    }

    /// Gets the value of Channels
    pub fn get_channels(&self) -> Option<&u32> {
        self.channels.as_ref()
    }

    /// Sets the value of ContextBoundClassesLoaded
    pub fn set_context_bound_classes_loaded(&mut self, value: u32) {
        self.context_bound_classes_loaded = Some(value);
    }

    /// Gets the value of ContextBoundClassesLoaded
    pub fn get_context_bound_classes_loaded(&self) -> Option<&u32> {
        self.context_bound_classes_loaded.as_ref()
    }

    /// Sets the value of ContextBoundObjectsAllocPersec
    pub fn set_context_bound_objects_alloc_persec(&mut self, value: u32) {
        self.context_bound_objects_alloc_persec = Some(value);
    }

    /// Gets the value of ContextBoundObjectsAllocPersec
    pub fn get_context_bound_objects_alloc_persec(&self) -> Option<&u32> {
        self.context_bound_objects_alloc_persec.as_ref()
    }

    /// Sets the value of ContextProxies
    pub fn set_context_proxies(&mut self, value: u32) {
        self.context_proxies = Some(value);
    }

    /// Gets the value of ContextProxies
    pub fn get_context_proxies(&self) -> Option<&u32> {
        self.context_proxies.as_ref()
    }

    /// Sets the value of Contexts
    pub fn set_contexts(&mut self, value: u32) {
        self.contexts = Some(value);
    }

    /// Gets the value of Contexts
    pub fn get_contexts(&self) -> Option<&u32> {
        self.contexts.as_ref()
    }

    /// Sets the value of RemoteCallsPersec
    pub fn set_remote_calls_persec(&mut self, value: u32) {
        self.remote_calls_persec = Some(value);
    }

    /// Gets the value of RemoteCallsPersec
    pub fn get_remote_calls_persec(&self) -> Option<&u32> {
        self.remote_calls_persec.as_ref()
    }

    /// Sets the value of TotalRemoteCalls
    pub fn set_total_remote_calls(&mut self, value: u32) {
        self.total_remote_calls = Some(value);
    }

    /// Gets the value of TotalRemoteCalls
    pub fn get_total_remote_calls(&self) -> Option<&u32> {
        self.total_remote_calls.as_ref()
    }
}

