// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ComputerSystem_Dedicated
//////////////////////////////////////////////

/// ComputerSystem_Dedicated enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ComputerSystem_Dedicated {
    /// Not_Dedicated
    #[serde(rename = "Not_Dedicated")]
    NotDedicated = 0,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 1,
    /// Other
    #[serde(rename = "Other")]
    Other = 2,
    /// Storage
    #[serde(rename = "Storage")]
    Storage = 3,
    /// Router
    #[serde(rename = "Router")]
    Router = 4,
    /// Switch
    #[serde(rename = "Switch")]
    Switch = 5,
    /// Layer_3_Switch
    #[serde(rename = "Layer_3_Switch")]
    Layer3Switch = 6,
    /// Central_Office_Switch
    #[serde(rename = "Central_Office_Switch")]
    CentralOfficeSwitch = 7,
    /// Hub
    #[serde(rename = "Hub")]
    Hub = 8,
    /// Access_Server
    #[serde(rename = "Access_Server")]
    AccessServer = 9,
    /// Firewall
    #[serde(rename = "Firewall")]
    Firewall = 10,
    /// Print
    #[serde(rename = "Print")]
    Print = 11,
    /// I_O
    #[serde(rename = "I_O")]
    IO = 12,
    /// Web_Caching
    #[serde(rename = "Web_Caching")]
    WebCaching = 13,
    /// Management
    #[serde(rename = "Management")]
    Management = 14,
    /// Block_Server
    #[serde(rename = "Block_Server")]
    BlockServer = 15,
    /// File_Server
    #[serde(rename = "File_Server")]
    FileServer = 16,
    /// Mobile_User_Device
    #[serde(rename = "Mobile_User_Device")]
    MobileUserDevice = 17,
    /// Repeater
    #[serde(rename = "Repeater")]
    Repeater = 18,
    /// Bridge_Extender
    #[serde(rename = "Bridge_Extender")]
    BridgeExtender = 19,
    /// Gateway
    #[serde(rename = "Gateway")]
    Gateway = 20,
    /// Storage_Virtualizer
    #[serde(rename = "Storage_Virtualizer")]
    StorageVirtualizer = 21,
    /// Media_Library
    #[serde(rename = "Media_Library")]
    MediaLibrary = 22,
    /// ExtenderNode
    #[serde(rename = "ExtenderNode")]
    ExtenderNode = 23,
    /// NAS_Head
    #[serde(rename = "NAS_Head")]
    NASHead = 24,
    /// Self_contained_NAS
    #[serde(rename = "Self_contained_NAS")]
    SelfContainedNAS = 25,
    /// UPS
    #[serde(rename = "UPS")]
    UPS = 26,
    /// IP_Phone
    #[serde(rename = "IP_Phone")]
    IPPhone = 27,
    /// Management_Controller
    #[serde(rename = "Management_Controller")]
    ManagementController = 28,
    /// Chassis_Manager
    #[serde(rename = "Chassis_Manager")]
    ChassisManager = 29,
    /// Host_based_RAID_controller
    #[serde(rename = "Host_based_RAID_controller")]
    HostBasedRAIDController = 30,
    /// Storage_Device_Enclosure
    #[serde(rename = "Storage_Device_Enclosure")]
    StorageDeviceEnclosure = 31,
    /// Desktop
    #[serde(rename = "Desktop")]
    Desktop = 32,
    /// Laptop
    #[serde(rename = "Laptop")]
    Laptop = 33,
    /// Virtual_Tape_Library
    #[serde(rename = "Virtual_Tape_Library")]
    VirtualTapeLibrary = 34,
    /// Virtual_Library_System
    #[serde(rename = "Virtual_Library_System")]
    VirtualLibrarySystem = 35,
    /// Network_PC_Thin_Client
    #[serde(rename = "Network_PC_Thin_Client")]
    NetworkPCThinClient = 36,
    /// FC_Switch
    #[serde(rename = "FC_Switch")]
    FCSwitch = 37,
    /// Ethernet_Switch
    #[serde(rename = "Ethernet_Switch")]
    EthernetSwitch = 38,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 39,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 40,
}

impl Default for ComputerSystem_Dedicated {
    fn default() -> Self {
        Self::NotDedicated
    }
}

