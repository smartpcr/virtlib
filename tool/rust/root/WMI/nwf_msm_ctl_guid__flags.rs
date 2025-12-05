// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source NwfMsmCtlGuid_Flags
//////////////////////////////////////////////

/// NwfMsmCtlGuid_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum NwfMsmCtlGuid_Flags {
    /// DOT11_ASSOCIATE
    #[serde(rename = "DOT11_ASSOCIATE")]
    DOT11ASSOCIATE = 1,
    /// DOT11_ROAMING
    #[serde(rename = "DOT11_ROAMING")]
    DOT11ROAMING = 2,
    /// DOT11_1X
    #[serde(rename = "DOT11_1X")]
    DOT111X = 3,
    /// DOT11_PNP
    #[serde(rename = "DOT11_PNP")]
    DOT11PNP = 4,
    /// DOT11_SCAN
    #[serde(rename = "DOT11_SCAN")]
    DOT11SCAN = 5,
    /// DOT11_RECEIVE
    #[serde(rename = "DOT11_RECEIVE")]
    DOT11RECEIVE = 6,
    /// DOT11_SEND
    #[serde(rename = "DOT11_SEND")]
    DOT11SEND = 7,
    /// DOT11_IOCTL
    #[serde(rename = "DOT11_IOCTL")]
    DOT11IOCTL = 8,
    /// DOT11_OID
    #[serde(rename = "DOT11_OID")]
    DOT11OID = 9,
    /// DOT11_MISC
    #[serde(rename = "DOT11_MISC")]
    DOT11MISC = 10,
    /// DOT11_UPCALL
    #[serde(rename = "DOT11_UPCALL")]
    DOT11UPCALL = 11,
    /// DOT11_KEYMGR
    #[serde(rename = "DOT11_KEYMGR")]
    DOT11KEYMGR = 12,
    /// DOT11_PEER
    #[serde(rename = "DOT11_PEER")]
    DOT11PEER = 13,
    /// DOT11_SOFTAP
    #[serde(rename = "DOT11_SOFTAP")]
    DOT11SOFTAP = 14,
    /// DOT11_PAM
    #[serde(rename = "DOT11_PAM")]
    DOT11PAM = 15,
    /// DOT11_REPEATER
    #[serde(rename = "DOT11_REPEATER")]
    DOT11REPEATER = 16,
    /// DOT11_APROUTER
    #[serde(rename = "DOT11_APROUTER")]
    DOT11APROUTER = 17,
    /// DOT11_WME
    #[serde(rename = "DOT11_WME")]
    DOT11WME = 18,
    /// DOT11_CONFIG
    #[serde(rename = "DOT11_CONFIG")]
    DOT11CONFIG = 19,
    /// DOT11_NOTIFY_OBJECT
    #[serde(rename = "DOT11_NOTIFY_OBJECT")]
    DOT11NOTIFYOBJECT = 20,
}

impl Default for NwfMsmCtlGuid_Flags {
    fn default() -> Self {
        Self::DOT11ASSOCIATE
    }
}

