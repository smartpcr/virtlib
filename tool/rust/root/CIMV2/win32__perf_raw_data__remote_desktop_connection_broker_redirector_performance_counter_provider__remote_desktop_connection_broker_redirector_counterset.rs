// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_RemoteDesktopConnectionBrokerRedirectorPerformanceCounterProvider_RemoteDesktopConnectionBrokerRedirectorCounterset struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_RemoteDesktopConnectionBrokerRedirectorPerformanceCounterProvider_RemoteDesktopConnectionBrokerRedirectorCounterset {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "Connectiontime")]
    pub connectiontime: Option<u64>,

/// 
    #[serde(rename = "Contextacquisitionwaittime")]
    pub contextacquisitionwaittime: Option<u64>,

/// 
    #[serde(rename = "RPCContext")]
    pub rpccontext: Option<u64>,

/// 
    #[serde(rename = "ThreadswaitingforRPCContext")]
    pub threadswaitingfor_rpccontext: Option<u64>,
}

impl Win32_PerfRawData_RemoteDesktopConnectionBrokerRedirectorPerformanceCounterProvider_RemoteDesktopConnectionBrokerRedirectorCounterset {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            connectiontime: None,
            contextacquisitionwaittime: None,
            rpccontext: None,
            threadswaitingfor_rpccontext: None,
        }
    }


    /// Sets the value of Connectiontime
    pub fn set_connectiontime(&mut self, value: u64) {
        self.connectiontime = Some(value);
    }

    /// Gets the value of Connectiontime
    pub fn get_connectiontime(&self) -> Option<&u64> {
        self.connectiontime.as_ref()
    }

    /// Sets the value of Contextacquisitionwaittime
    pub fn set_contextacquisitionwaittime(&mut self, value: u64) {
        self.contextacquisitionwaittime = Some(value);
    }

    /// Gets the value of Contextacquisitionwaittime
    pub fn get_contextacquisitionwaittime(&self) -> Option<&u64> {
        self.contextacquisitionwaittime.as_ref()
    }

    /// Sets the value of RPCContext
    pub fn set_rpccontext(&mut self, value: u64) {
        self.rpccontext = Some(value);
    }

    /// Gets the value of RPCContext
    pub fn get_rpccontext(&self) -> Option<&u64> {
        self.rpccontext.as_ref()
    }

    /// Sets the value of ThreadswaitingforRPCContext
    pub fn set_threadswaitingfor_rpccontext(&mut self, value: u64) {
        self.threadswaitingfor_rpccontext = Some(value);
    }

    /// Gets the value of ThreadswaitingforRPCContext
    pub fn get_threadswaitingfor_rpccontext(&self) -> Option<&u64> {
        self.threadswaitingfor_rpccontext.as_ref()
    }
}

