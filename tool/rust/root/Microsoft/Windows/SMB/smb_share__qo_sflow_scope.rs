// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SmbShare_QoSFlowScope
//////////////////////////////////////////////

/// SmbShare_QoSFlowScope enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SmbShare_QoSFlowScope {
    /// _69
    #[serde(rename = "_69")]
    V69 = 0,
    /// _70
    #[serde(rename = "_70")]
    V70 = 1,
    /// _71
    #[serde(rename = "_71")]
    V71 = 2,
    /// _72
    #[serde(rename = "_72")]
    V72 = 3,
}

impl Default for SmbShare_QoSFlowScope {
    fn default() -> Self {
        Self::V69
    }
}

