// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Header_BuildInfo_TypeGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Header_BuildInfo_TypeGroup {
    #[serde(flatten)]
    pub base: EventTraceEvent,

/// 
    #[serde(rename = "BuildString")]
    pub build_string: Option<String>,
}

impl Header_BuildInfo_TypeGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EventTraceEvent::new(),
            build_string: None,
        }
    }


    /// Sets the value of BuildString
    pub fn set_build_string(&mut self, value: String) {
        self.build_string = Some(value);
    }

    /// Gets the value of BuildString
    pub fn get_build_string(&self) -> Option<&String> {
        self.build_string.as_ref()
    }
}

