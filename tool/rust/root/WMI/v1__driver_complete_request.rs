// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// V1_DriverCompleteRequest struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct V1_DriverCompleteRequest {
    #[serde(flatten)]
    pub base: DiskIo_V1,

/// 
    #[serde(rename = "Irp")]
    pub irp: Option<u32>,

/// 
    #[serde(rename = "RoutineAddr")]
    pub routine_addr: Option<u32>,

/// 
    #[serde(rename = "UniqMatchId")]
    pub uniq_match_id: Option<u32>,
}

impl V1_DriverCompleteRequest {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: DiskIo_V1::new(),
            irp: None,
            routine_addr: None,
            uniq_match_id: None,
        }
    }


    /// Sets the value of Irp
    pub fn set_irp(&mut self, value: u32) {
        self.irp = Some(value);
    }

    /// Gets the value of Irp
    pub fn get_irp(&self) -> Option<&u32> {
        self.irp.as_ref()
    }

    /// Sets the value of RoutineAddr
    pub fn set_routine_addr(&mut self, value: u32) {
        self.routine_addr = Some(value);
    }

    /// Gets the value of RoutineAddr
    pub fn get_routine_addr(&self) -> Option<&u32> {
        self.routine_addr.as_ref()
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

