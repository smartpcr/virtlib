// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_TCPIPCounters_TCPIPPerformanceDiagnosticsPerCPU struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_TCPIPCounters_TCPIPPerformanceDiagnosticsPerCPU {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "TCPcurrentconnections")]
    pub tcpcurrentconnections: Option<u32>,
}

impl Win32_PerfRawData_TCPIPCounters_TCPIPPerformanceDiagnosticsPerCPU {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            tcpcurrentconnections: None,
        }
    }


    /// Sets the value of TCPcurrentconnections
    pub fn set_tcpcurrentconnections(&mut self, value: u32) {
        self.tcpcurrentconnections = Some(value);
    }

    /// Gets the value of TCPcurrentconnections
    pub fn get_tcpcurrentconnections(&self) -> Option<&u32> {
        self.tcpcurrentconnections.as_ref()
    }
}

