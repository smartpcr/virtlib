// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source FCPort_SupportedFC4Types
//////////////////////////////////////////////

/// FCPort_SupportedFC4Types enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum FCPort_SupportedFC4Types {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// ISO_IEC_8802___2_LLC
    #[serde(rename = "ISO_IEC_8802___2_LLC")]
    ISOIEC88022LLC = 4,
    /// IP_over_FC
    #[serde(rename = "IP_over_FC")]
    IPOverFC = 5,
    /// SCSI___FCP
    #[serde(rename = "SCSI___FCP")]
    SCSIFCP = 8,
    /// SCSI___GPP
    #[serde(rename = "SCSI___GPP")]
    SCSIGPP = 9,
    /// IPI___3_Master
    #[serde(rename = "IPI___3_Master")]
    IPI3Master = 17,
    /// IPI___3_Slave
    #[serde(rename = "IPI___3_Slave")]
    IPI3Slave = 18,
    /// IPI___3_Peer
    #[serde(rename = "IPI___3_Peer")]
    IPI3Peer = 19,
    /// CP_IPI___3_Master
    #[serde(rename = "CP_IPI___3_Master")]
    CPIPI3Master = 21,
    /// CP_IPI___3_Slave
    #[serde(rename = "CP_IPI___3_Slave")]
    CPIPI3Slave = 22,
    /// CP_IPI___3_Peer
    #[serde(rename = "CP_IPI___3_Peer")]
    CPIPI3Peer = 23,
    /// SBCCS_Channel
    #[serde(rename = "SBCCS_Channel")]
    SBCCSChannel = 25,
    /// SBCCS_Control_Unit
    #[serde(rename = "SBCCS_Control_Unit")]
    SBCCSControlUnit = 26,
    /// FC_SB_2_Channel
    #[serde(rename = "FC_SB_2_Channel")]
    FCSB2Channel = 27,
    /// FC_SB_2_Control_Unit
    #[serde(rename = "FC_SB_2_Control_Unit")]
    FCSB2ControlUnit = 28,
    /// Fibre_Channel_Services__FC_GS__FC_GS_2__FC_GS_3_
    #[serde(rename = "Fibre_Channel_Services__FC_GS__FC_GS_2__FC_GS_3_")]
    FibreChannelServicesFCGSFCGS2FCGS3 = 32,
    /// FC_SW
    #[serde(rename = "FC_SW")]
    FCSW = 34,
    /// FC___SNMP
    #[serde(rename = "FC___SNMP")]
    FCSNMP = 36,
    /// HIPPI___FP
    #[serde(rename = "HIPPI___FP")]
    HIPPIFP = 64,
    /// BBL_Control
    #[serde(rename = "BBL_Control")]
    BBLControl = 80,
    /// BBL_FDDI_Encapsulated_LAN_PDU
    #[serde(rename = "BBL_FDDI_Encapsulated_LAN_PDU")]
    BBLFDDIEncapsulatedLANPDU = 81,
    /// BBL_802_3_Encapsulated_LAN_PDU
    #[serde(rename = "BBL_802_3_Encapsulated_LAN_PDU")]
    BBL8023EncapsulatedLANPDU = 82,
    /// FC___VI
    #[serde(rename = "FC___VI")]
    FCVI = 88,
    /// FC___AV
    #[serde(rename = "FC___AV")]
    FCAV = 96,
    /// Vendor_Unique
    #[serde(rename = "Vendor_Unique")]
    VendorUnique = 255,
}

impl Default for FCPort_SupportedFC4Types {
    fn default() -> Self {
        Self::Unknown
    }
}

