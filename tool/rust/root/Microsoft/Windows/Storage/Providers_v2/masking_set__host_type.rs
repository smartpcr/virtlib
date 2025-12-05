// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source MaskingSet_HostType
//////////////////////////////////////////////

/// MaskingSet_HostType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum MaskingSet_HostType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Standard
    #[serde(rename = "Standard")]
    Standard = 2,
    /// Solaris
    #[serde(rename = "Solaris")]
    Solaris = 3,
    /// HPUX
    #[serde(rename = "HPUX")]
    HPUX = 4,
    /// OpenVMS
    #[serde(rename = "OpenVMS")]
    OpenVMS = 5,
    /// Tru64
    #[serde(rename = "Tru64")]
    Tru64 = 6,
    /// Netware
    #[serde(rename = "Netware")]
    Netware = 7,
    /// Sequent
    #[serde(rename = "Sequent")]
    Sequent = 8,
    /// AIX
    #[serde(rename = "AIX")]
    AIX = 9,
    /// DGUX
    #[serde(rename = "DGUX")]
    DGUX = 10,
    /// Dynix
    #[serde(rename = "Dynix")]
    Dynix = 11,
    /// Irix
    #[serde(rename = "Irix")]
    Irix = 12,
    /// Cisco_iSCSI_Storage_Router
    #[serde(rename = "Cisco_iSCSI_Storage_Router")]
    CiscoISCSIStorageRouter = 13,
    /// Linux
    #[serde(rename = "Linux")]
    Linux = 14,
    /// Microsoft_Windows
    #[serde(rename = "Microsoft_Windows")]
    MicrosoftWindows = 15,
    /// OS400
    #[serde(rename = "OS400")]
    OS400 = 16,
    /// TRESPASS
    #[serde(rename = "TRESPASS")]
    TRESPASS = 17,
    /// HI_UX
    #[serde(rename = "HI_UX")]
    HIUX = 18,
    /// VMware_ESXi
    #[serde(rename = "VMware_ESXi")]
    VMwareESXi = 19,
    /// Microsoft_Windows_Server_2008
    #[serde(rename = "Microsoft_Windows_Server_2008")]
    MicrosoftWindowsServer2008 = 20,
    /// Microsoft_Windows_Server_2003
    #[serde(rename = "Microsoft_Windows_Server_2003")]
    MicrosoftWindowsServer2003 = 21,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 22,
    /// Vendor_Specific
    #[serde(rename = "Vendor_Specific")]
    VendorSpecific = 23,
}

impl Default for MaskingSet_HostType {
    fn default() -> Self {
        Self::Unknown
    }
}

