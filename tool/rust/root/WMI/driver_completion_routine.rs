// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DriverCompletionRoutine struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DriverCompletionRoutine {
    #[serde(flatten)]
    pub base: DiskIo_V2,

/// 
    #[serde(rename = "IrpPtr")]
    pub irp_ptr: Option<u32>,

/// 
    #[serde(rename = "Routine")]
    pub routine: Option<u32>,

/// 
    #[serde(rename = "UniqMatchId")]
    pub uniq_match_id: Option<u32>,
}

impl DriverCompletionRoutine {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: DiskIo_V2::new(),
            irp_ptr: None,
            routine: None,
            uniq_match_id: None,
        }
    }


    /// Sets the value of IrpPtr
    pub fn set_irp_ptr(&mut self, value: u32) {
        self.irp_ptr = Some(value);
    }

    /// Gets the value of IrpPtr
    pub fn get_irp_ptr(&self) -> Option<&u32> {
        self.irp_ptr.as_ref()
    }

    /// Sets the value of Routine
    pub fn set_routine(&mut self, value: u32) {
        self.routine = Some(value);
    }

    /// Gets the value of Routine
    pub fn get_routine(&self) -> Option<&u32> {
        self.routine.as_ref()
    }

    /// Sets the value of UniqMatchId
    pub fn set_uniq_match_id(&mut self, value: u32) {
        self.uniq_match_id = Some(value);
    }

    /// Gets the value of UniqMatchId
    pub fn get_uniq_match_id(&self) -> Option<&u32> {
        self.uniq_match_id.as_ref()
    }
}

