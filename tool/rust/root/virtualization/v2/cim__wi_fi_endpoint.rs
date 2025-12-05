// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_WiFiEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_WiFiEndpoint {
    #[serde(flatten)]
    pub base: CIM_LANEndpoint,

/// AccessPointAddress shall contain the MAC address of the access point with which the WiFiEndpoint is currently associated. If the WiFiEndpoint is not currently associated, then AccessPointAddress shall be NULL.The MAC address shall be formatted as twelve hexadecimal digits (for example, "010203040506"), with each pair representing one of the six octets of the MAC address in "canonical" bit order. (Therefore, the Group address bit is found in the low order bit of the first character of the string.)
    #[serde(rename = "AccessPointAddress")]
    pub access_point_address: Option<String>,

/// Associated shall indicate whether or not the WiFiEndpoint is currently associated to an access point or client station.
    #[serde(rename = "Associated")]
    pub associated: Option<bool>,

/// AuthenticationMethod shall specify the method used to authenticate the WiFiEndpoint and the network to one another.
/// 	* Unknown (0): shall indicate that the authentication method is unknown to the server.	* Other (1): shall indicate that the authentication method is known to the server but not specified in the list below. If AuthenticationMethod contains 1, OtherAuthenticationMethod shall not be NULL and shall not be empty.
/// 	* Open System (2): shall indicate that the authentication method is Open System. AuthenticationMethod shall contain 2 only if EncryptionMethod contains 2 ("WEP").
/// 	* Shared Key (3): shall indicate that the authentication method is Shared Key. AuthenticationMethod shall contain 3 only if EncryptionMethod contains 2 ("WEP").
/// 	* WPA PSK (4): shall indicate that the authentication method is WPA (Wi-Fi Protected Access) PSK (Pre-Shared Key). AuthenticationMethod shall contain 4 only if EncryptionMethod contains 3 ("TKIP") or 4 ("CCMP").
/// 	* WPA IEEE 802.1x (5): shall indicate that the authentication method is WPA (Wi-Fi Protected Access) IEEE 802.1x. AuthenticationMethod shall contain 5 only if EncryptionMethod contains 3 ("TKIP") or 4 ("CCMP").
/// 	* WPA2 PSK (6): shall indicate that the authentication method is WPA2 (Wi-Fi Protected Access Version 2) PSK (Pre-Shared Key). AuthenticationMethod shall contain 6 only if EncryptionMethod contains 3 ("TKIP") or 4 ("CCMP").
/// 	* WPA2 IEEE 802.1x (7): shall indicate that the authentication method is WPA2 (Wi-Fi Protected Access Version 2) IEEE 802.1x. AuthenticationMethod shall contain 6 only if EncryptionMethod contains 3 ("TKIP") or 4 ("CCMP").
/// 	* CCKM IEEE 802.1x (8): CCKM (Cisco Centralized Key Management with LEAP or EAP-FAST)
    #[serde(rename = "AuthenticationMethod")]
    pub authentication_method: Option<WiFiEndpoint_AuthenticationMethod>,

/// BSSType shall indicate the Basic Service Set (BSS) Type of the network that corresponds to the instance. A Basic Service Set is a set of stations controlled by a single coordination function.
/// 	* Independent: the WiFiEndpoint is associated directly to another client station.
/// 	* Infrastructure: the WiFiEndpoint is associated to a network via an access point.
    #[serde(rename = "BSSType")]
    pub bsstype: Option<WiFiEndpoint_BSSType>,

/// EncryptionMethod shall specify the encryption method in use to protect the confidentiality of data sent and received by the WiFiEndpoint.
/// 	* Unknown (0): shall indicate that the encryption method is unknown to the server.
/// 	* Other (1): shall indicate that the encryption method is known to the server but not defined in the list below. If EncryptionMethod contains 1, OtherEncryptionMethod shall not be NULL and shall not be empty.
/// 	* WEP (2): shall indicate that the encryption method is Wired Equivalency Privacy (WEP). The value of EncryptionMethod shall be 2 only if the value of AuthenticationMethod is 2 ("Open System") or 3 ("Shared Key").
/// 	* TKIP (3): shall indicate that the encryption method is Temporal Key Integrity Protocol (TKIP). The value of EncryptionMethod shall be 3 only if the value of AuthenticationMethod is 4 ("WPA PSK"), 5 ("WPA IEEE 802.1x"), 6 ("WPA2 PSK"), or 7 ("WPA2 IEEE 802.1x").
/// 	* CCMP (4): shall indicate that the encryption method is Counter Mode with Cipher Block Chaining Message Authentication Code Protocol (CCMP). The value of EncryptionMethod shall be 4 only if the value of AuthenticationMethod is 4 ("WPA PSK"), 5 ("WPA IEEE 802.1x"), 6 ("WPA2 PSK"), or 7 ("WPA2 IEEE 802.1x").
/// 	* None (5): shall indicate that no encryption method is in use. The value of EncryptionMethod shall be 5 only if the value of AuthenticationMethod is 2 ("Open System") or 3 ("Shared Key").
    #[serde(rename = "EncryptionMethod")]
    pub encryption_method: Option<WiFiEndpoint_EncryptionMethod>,

/// IEEE8021xAuthenticationProtocol shall contain the EAP (Extensible Authentication Protocol) type if and only if AuthenticationMethod contains "WPA IEEE 802.1x" or "WPA2 IEEE 802.1x" or "CCKM IEEE 802.1x"
/// .	* EAP-TLS (0): shall indicate the Transport Layer Security EAP type specified in RFC 2716.
/// 	* EAP-TTLS/MSCHAPv2 (1): shall indicate the Tunneled TLS Authentication Protocol EAP type (specified in draft-ietf-pppext-eap-ttls) with Microsoft PPP CHAP Extensions, Version 2 (MSCHAPv2) as the inner authentication method.
/// 	* PEAPv0/EAP-MSCHAPv2 (2): shall indicate the Protected Extensible Authentication Protocol (PEAP) Version 0 EAP type (specified in draft-kamath-pppext-peapv0), with Microsoft PPP CHAP Extensions, Version 2 (MSCHAPv2) as the inner authentication method.
/// 	* PEAPv1/EAP-GTC (3): shall indicate the Protected Extensible Authentication Protocol (PEAP) Version 1 EAP type (specified in draft-josefsson-pppext-eap-tls-eap), with Generic Token Card (GTC) as the inner authentication method.
/// 	* EAP-FAST/MSCHAPv2 (4): shall indicate the Flexible Authentication Extensible Authentication Protocol EAP type specified in IETF RFC 4851, with Microsoft PPP CHAP Extensions, Version 2 (MSCHAPv2) as the inner authentication method.
/// 	* EAP-FAST/GTC (5): shall indicate the Flexible Authentication Extensible Authentication Protocol EAP type specified in IETF RFC 4851, with Generic Token Card (GTC) as the inner authentication method.
/// 	* EAP-MD5 (6): shall indicate the EAP MD5 authentication method, specified in RFC 3748.
/// 	* EAP-PSK (7): shall indicate the EAP-PSK (Pre-shared Key) Protocol specified in RFC 4764.
/// 	* EAP-SIM (8): shall indicate the Extensible Authentication Protocol Method for Global System for Mobile Communications (GSM) Subscriber Identity Modules (EAP-SIM), specified in RFC 4186.
/// 	* EAP-AKA (9): shall indicate the Extensible Authentication Protocol Method for 3rd Generation Authentication and Key Agreement (EAP-AKA) authentication method, specified in RFC 4187.
/// 	* EAP-FAST/TLS (10): shall indicate the Flexible Authentication Extensible Authentication Protocol EAP type specified in IETF RFC 4851, with TLS as the inner authentication method.
    #[serde(rename = "IEEE8021xAuthenticationProtocol")]
    pub ieee8021x_authentication_protocol: Option<WiFiEndpoint_IEEE8021xAuthenticationProtocol>,

/// OtherAuthenticationMethod shall specify the 802.11 authentication method if and only if AuthenticationMethod contains "Other". The format of this string shall be vendor-specific.
    #[serde(rename = "OtherAuthenticationMethod")]
    pub other_authentication_method: Option<String>,

/// OtherEncryptionMethod shall specify the 802.11 encryption method if and only if EncryptionMethod contains "Other". The format of this string shall be vendor-specific.
    #[serde(rename = "OtherEncryptionMethod")]
    pub other_encryption_method: Option<String>,
}

impl CIM_WiFiEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LANEndpoint::new(),
            access_point_address: None,
            associated: None,
            authentication_method: None,
            bsstype: None,
            encryption_method: None,
            ieee8021x_authentication_protocol: None,
            other_authentication_method: None,
            other_encryption_method: None,
        }
    }


    /// Sets the value of AccessPointAddress
    pub fn set_access_point_address(&mut self, value: String) {
        self.access_point_address = Some(value);
    }

    /// Gets the value of AccessPointAddress
    pub fn get_access_point_address(&self) -> Option<&String> {
        self.access_point_address.as_ref()
    }

    /// Sets the value of Associated
    pub fn set_associated(&mut self, value: bool) {
        self.associated = Some(value);
    }

    /// Gets the value of Associated
    pub fn get_associated(&self) -> Option<&bool> {
        self.associated.as_ref()
    }

    /// Sets the value of AuthenticationMethod
    pub fn set_authentication_method(&mut self, value: WiFiEndpoint_AuthenticationMethod) {
        self.authentication_method = Some(value);
    }

    /// Gets the value of AuthenticationMethod
    pub fn get_authentication_method(&self) -> Option<&WiFiEndpoint_AuthenticationMethod> {
        self.authentication_method.as_ref()
    }

    /// Sets the value of BSSType
    pub fn set_bsstype(&mut self, value: WiFiEndpoint_BSSType) {
        self.bsstype = Some(value);
    }

    /// Gets the value of BSSType
    pub fn get_bsstype(&self) -> Option<&WiFiEndpoint_BSSType> {
        self.bsstype.as_ref()
    }

    /// Sets the value of EncryptionMethod
    pub fn set_encryption_method(&mut self, value: WiFiEndpoint_EncryptionMethod) {
        self.encryption_method = Some(value);
    }

    /// Gets the value of EncryptionMethod
    pub fn get_encryption_method(&self) -> Option<&WiFiEndpoint_EncryptionMethod> {
        self.encryption_method.as_ref()
    }

    /// Sets the value of IEEE8021xAuthenticationProtocol
    pub fn set_ieee8021x_authentication_protocol(&mut self, value: WiFiEndpoint_IEEE8021xAuthenticationProtocol) {
        self.ieee8021x_authentication_protocol = Some(value);
    }

    /// Gets the value of IEEE8021xAuthenticationProtocol
    pub fn get_ieee8021x_authentication_protocol(&self) -> Option<&WiFiEndpoint_IEEE8021xAuthenticationProtocol> {
        self.ieee8021x_authentication_protocol.as_ref()
    }

    /// Sets the value of OtherAuthenticationMethod
    pub fn set_other_authentication_method(&mut self, value: String) {
        self.other_authentication_method = Some(value);
    }

    /// Gets the value of OtherAuthenticationMethod
    pub fn get_other_authentication_method(&self) -> Option<&String> {
        self.other_authentication_method.as_ref()
    }

    /// Sets the value of OtherEncryptionMethod
    pub fn set_other_encryption_method(&mut self, value: String) {
        self.other_encryption_method = Some(value);
    }

    /// Gets the value of OtherEncryptionMethod
    pub fn get_other_encryption_method(&self) -> Option<&String> {
        self.other_encryption_method.as_ref()
    }
}

