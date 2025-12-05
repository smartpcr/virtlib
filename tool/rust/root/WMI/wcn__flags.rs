// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WCN_Flags
//////////////////////////////////////////////

/// WCN_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WCN_Flags {
    /// TR_DEFAULT
    #[serde(rename = "TR_DEFAULT")]
    TRDEFAULT = 1,
    /// TR_SM
    #[serde(rename = "TR_SM")]
    TRSM = 2,
    /// TR_BUF
    #[serde(rename = "TR_BUF")]
    TRBUF = 3,
    /// TR_TLV
    #[serde(rename = "TR_TLV")]
    TRTLV = 4,
    /// TR_CACHE
    #[serde(rename = "TR_CACHE")]
    TRCACHE = 5,
    /// TR_DLL
    #[serde(rename = "TR_DLL")]
    TRDLL = 6,
    /// TR_RPC
    #[serde(rename = "TR_RPC")]
    TRRPC = 7,
    /// TR_POLICY
    #[serde(rename = "TR_POLICY")]
    TRPOLICY = 8,
    /// TR_PROTO
    #[serde(rename = "TR_PROTO")]
    TRPROTO = 9,
    /// TR_CRYPTO
    #[serde(rename = "TR_CRYPTO")]
    TRCRYPTO = 10,
    /// TR_WSD
    #[serde(rename = "TR_WSD")]
    TRWSD = 11,
    /// TR_EAP
    #[serde(rename = "TR_EAP")]
    TREAP = 12,
    /// TR_TRANS
    #[serde(rename = "TR_TRANS")]
    TRTRANS = 13,
    /// TR_WQ
    #[serde(rename = "TR_WQ")]
    TRWQ = 14,
    /// TR_UTIL
    #[serde(rename = "TR_UTIL")]
    TRUTIL = 15,
    /// TR_COM
    #[serde(rename = "TR_COM")]
    TRCOM = 16,
    /// TR_FD
    #[serde(rename = "TR_FD")]
    TRFD = 17,
    /// TR_WIZ
    #[serde(rename = "TR_WIZ")]
    TRWIZ = 18,
}

impl Default for WCN_Flags {
    fn default() -> Self {
        Self::TRDEFAULT
    }
}

