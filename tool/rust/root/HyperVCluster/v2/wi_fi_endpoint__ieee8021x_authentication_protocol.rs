// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source WiFiEndpoint_IEEE8021xAuthenticationProtocol
//////////////////////////////////////////////

/// WiFiEndpoint_IEEE8021xAuthenticationProtocol enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum WiFiEndpoint_IEEE8021xAuthenticationProtocol {
    /// EAP_TLS
    #[serde(rename = "EAP_TLS")]
    EAPTLS = 0,
    /// EAP_TTLS_MSCHAPv2
    #[serde(rename = "EAP_TTLS_MSCHAPv2")]
    EAPTTLSMSCHAPv2 = 1,
    /// PEAPv0_EAP_MSCHAPv2
    #[serde(rename = "PEAPv0_EAP_MSCHAPv2")]
    PEAPv0EAPMSCHAPv2 = 2,
    /// PEAPv1_EAP_GTC
    #[serde(rename = "PEAPv1_EAP_GTC")]
    PEAPv1EAPGTC = 3,
    /// EAP_FAST_MSCHAPv2
    #[serde(rename = "EAP_FAST_MSCHAPv2")]
    EAPFASTMSCHAPv2 = 4,
    /// EAP_FAST_GTC
    #[serde(rename = "EAP_FAST_GTC")]
    EAPFASTGTC = 5,
    /// EAP_MD5
    #[serde(rename = "EAP_MD5")]
    EAPMD5 = 6,
    /// EAP_PSK
    #[serde(rename = "EAP_PSK")]
    EAPPSK = 7,
    /// EAP_SIM
    #[serde(rename = "EAP_SIM")]
    EAPSIM = 8,
    /// EAP_AKA
    #[serde(rename = "EAP_AKA")]
    EAPAKA = 9,
    /// EAP_FAST_TLS
    #[serde(rename = "EAP_FAST_TLS")]
    EAPFASTTLS = 10,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 11,
}

impl Default for WiFiEndpoint_IEEE8021xAuthenticationProtocol {
    fn default() -> Self {
        Self::EAPTLS
    }
}

