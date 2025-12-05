// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SplitIo_Info struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SplitIo_Info {
    #[serde(flatten)]
    pub base: SplitIo,

/// 
    #[serde(rename = "ChildIrp")]
    pub child_irp: Option<u32>,

/// 
    #[serde(rename = "ParentIrp")]
    pub parent_irp: Option<u32>,
}

impl SplitIo_Info {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SplitIo::new(),
            child_irp: None,
            parent_irp: None,
        }
    }


    /// Sets the value of ChildIrp
    pub fn set_child_irp(&mut self, value: u32) {
        self.child_irp = Some(value);
    }

    /// Gets the value of ChildIrp
    pub fn get_child_irp(&self) -> Option<&u32> {
        self.child_irp.as_ref()
    }

    /// Sets the value of ParentIrp
    pub fn set_parent_irp(&mut self, value: u32) {
        self.parent_irp = Some(value);
    }

    /// Gets the value of ParentIrp
    pub fn get_parent_irp(&self) -> Option<&u32> {
        self.parent_irp.as_ref()
    }
}

