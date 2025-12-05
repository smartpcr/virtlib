// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ThreadMigration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreadMigration {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "IdealProcessorAdjust")]
    pub ideal_processor_adjust: Option<bool>,

/// 
    #[serde(rename = "OldIdealProcessorIndex")]
    pub old_ideal_processor_index: Option<u16>,

/// 
    #[serde(rename = "Priority")]
    pub priority: Option<u8>,

/// 
    #[serde(rename = "SourceProcessorIndex")]
    pub source_processor_index: Option<u16>,

/// 
    #[serde(rename = "TargetProcessorIndex")]
    pub target_processor_index: Option<u16>,

/// 
    #[serde(rename = "ThreadId")]
    pub thread_id: Option<u32>,
}

impl ThreadMigration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            ideal_processor_adjust: None,
            old_ideal_processor_index: None,
            priority: None,
            source_processor_index: None,
            target_processor_index: None,
            thread_id: None,
        }
    }


    /// Sets the value of IdealProcessorAdjust
    pub fn set_ideal_processor_adjust(&mut self, value: bool) {
        self.ideal_processor_adjust = Some(value);
    }

    /// Gets the value of IdealProcessorAdjust
    pub fn get_ideal_processor_adjust(&self) -> Option<&bool> {
        self.ideal_processor_adjust.as_ref()
    }

    /// Sets the value of OldIdealProcessorIndex
    pub fn set_old_ideal_processor_index(&mut self, value: u16) {
        self.old_ideal_processor_index = Some(value);
    }

    /// Gets the value of OldIdealProcessorIndex
    pub fn get_old_ideal_processor_index(&self) -> Option<&u16> {
        self.old_ideal_processor_index.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u8) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u8> {
        self.priority.as_ref()
    }

    /// Sets the value of SourceProcessorIndex
    pub fn set_source_processor_index(&mut self, value: u16) {
        self.source_processor_index = Some(value);
    }

    /// Gets the value of SourceProcessorIndex
    pub fn get_source_processor_index(&self) -> Option<&u16> {
        self.source_processor_index.as_ref()
    }

    /// Sets the value of TargetProcessorIndex
    pub fn set_target_processor_index(&mut self, value: u16) {
        self.target_processor_index = Some(value);
    }

    /// Gets the value of TargetProcessorIndex
    pub fn get_target_processor_index(&self) -> Option<&u16> {
        self.target_processor_index.as_ref()
    }

    /// Sets the value of ThreadId
    pub fn set_thread_id(&mut self, value: u32) {
        self.thread_id = Some(value);
    }

    /// Gets the value of ThreadId
    pub fn get_thread_id(&self) -> Option<&u32> {
        self.thread_id.as_ref()
    }
}

