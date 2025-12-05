// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_PerfOS_Objects struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_PerfOS_Objects {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Events")]
    pub events: Option<u32>,

/// 
    #[serde(rename = "Mutexes")]
    pub mutexes: Option<u32>,

/// 
    #[serde(rename = "Processes")]
    pub processes: Option<u32>,

/// 
    #[serde(rename = "Sections")]
    pub sections: Option<u32>,

/// 
    #[serde(rename = "Semaphores")]
    pub semaphores: Option<u32>,

/// 
    #[serde(rename = "Threads")]
    pub threads: Option<u32>,
}

impl Win32_PerfFormattedData_PerfOS_Objects {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            events: None,
            mutexes: None,
            processes: None,
            sections: None,
            semaphores: None,
            threads: None,
        }
    }


    /// Sets the value of Events
    pub fn set_events(&mut self, value: u32) {
        self.events = Some(value);
    }

    /// Gets the value of Events
    pub fn get_events(&self) -> Option<&u32> {
        self.events.as_ref()
    }

    /// Sets the value of Mutexes
    pub fn set_mutexes(&mut self, value: u32) {
        self.mutexes = Some(value);
    }

    /// Gets the value of Mutexes
    pub fn get_mutexes(&self) -> Option<&u32> {
        self.mutexes.as_ref()
    }

    /// Sets the value of Processes
    pub fn set_processes(&mut self, value: u32) {
        self.processes = Some(value);
    }

    /// Gets the value of Processes
    pub fn get_processes(&self) -> Option<&u32> {
        self.processes.as_ref()
    }

    /// Sets the value of Sections
    pub fn set_sections(&mut self, value: u32) {
        self.sections = Some(value);
    }

    /// Gets the value of Sections
    pub fn get_sections(&self) -> Option<&u32> {
        self.sections.as_ref()
    }

    /// Sets the value of Semaphores
    pub fn set_semaphores(&mut self, value: u32) {
        self.semaphores = Some(value);
    }

    /// Gets the value of Semaphores
    pub fn get_semaphores(&self) -> Option<&u32> {
        self.semaphores.as_ref()
    }

    /// Sets the value of Threads
    pub fn set_threads(&mut self, value: u32) {
        self.threads = Some(value);
    }

    /// Gets the value of Threads
    pub fn get_threads(&self) -> Option<&u32> {
        self.threads.as_ref()
    }
}

