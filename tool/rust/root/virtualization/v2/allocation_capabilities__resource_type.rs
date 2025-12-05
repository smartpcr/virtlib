// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source AllocationCapabilities_ResourceType
//////////////////////////////////////////////

/// AllocationCapabilities_ResourceType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum AllocationCapabilities_ResourceType {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Computer_System
    #[serde(rename = "Computer_System")]
    ComputerSystem = 2,
    /// Processor
    #[serde(rename = "Processor")]
    Processor = 3,
    /// Memory
    #[serde(rename = "Memory")]
    Memory = 4,
    /// IDE_Controller
    #[serde(rename = "IDE_Controller")]
    IDEController = 5,
    /// Parallel_SCSI_HBA
    #[serde(rename = "Parallel_SCSI_HBA")]
    ParallelSCSIHBA = 6,
    /// FC_HBA
    #[serde(rename = "FC_HBA")]
    FCHBA = 7,
    /// iSCSI_HBA
    #[serde(rename = "iSCSI_HBA")]
    ISCSIHBA = 8,
    /// IB_HCA
    #[serde(rename = "IB_HCA")]
    IBHCA = 9,
    /// Ethernet_Adapter
    #[serde(rename = "Ethernet_Adapter")]
    EthernetAdapter = 10,
    /// Other_Network_Adapter
    #[serde(rename = "Other_Network_Adapter")]
    OtherNetworkAdapter = 11,
    /// I_O_Slot
    #[serde(rename = "I_O_Slot")]
    IOSlot = 12,
    /// I_O_Device
    #[serde(rename = "I_O_Device")]
    IODevice = 13,
    /// Floppy_Drive
    #[serde(rename = "Floppy_Drive")]
    FloppyDrive = 14,
    /// CD_Drive
    #[serde(rename = "CD_Drive")]
    CDDrive = 15,
    /// DVD_drive
    #[serde(rename = "DVD_drive")]
    DVDDrive = 16,
    /// Disk_Drive
    #[serde(rename = "Disk_Drive")]
    DiskDrive = 17,
    /// Tape_Drive
    #[serde(rename = "Tape_Drive")]
    TapeDrive = 18,
    /// Storage_Extent
    #[serde(rename = "Storage_Extent")]
    StorageExtent = 19,
    /// Other_Storage_Device
    #[serde(rename = "Other_Storage_Device")]
    OtherStorageDevice = 20,
    /// Serial_port
    #[serde(rename = "Serial_port")]
    SerialPort = 21,
    /// Parallel_port
    #[serde(rename = "Parallel_port")]
    ParallelPort = 22,
    /// USB_Controller
    #[serde(rename = "USB_Controller")]
    USBController = 23,
    /// Graphics_controller
    #[serde(rename = "Graphics_controller")]
    GraphicsController = 24,
    /// IEEE_1394_Controller
    #[serde(rename = "IEEE_1394_Controller")]
    IEEE1394Controller = 25,
    /// Partitionable_Unit
    #[serde(rename = "Partitionable_Unit")]
    PartitionableUnit = 26,
    /// Base_Partitionable_Unit
    #[serde(rename = "Base_Partitionable_Unit")]
    BasePartitionableUnit = 27,
    /// Power
    #[serde(rename = "Power")]
    Power = 28,
    /// Cooling_Capacity
    #[serde(rename = "Cooling_Capacity")]
    CoolingCapacity = 29,
    /// Ethernet_Switch_Port
    #[serde(rename = "Ethernet_Switch_Port")]
    EthernetSwitchPort = 30,
    /// Logical_Disk
    #[serde(rename = "Logical_Disk")]
    LogicalDisk = 31,
    /// Storage_Volume
    #[serde(rename = "Storage_Volume")]
    StorageVolume = 32,
    /// Ethernet_Connection
    #[serde(rename = "Ethernet_Connection")]
    EthernetConnection = 33,
    /// DMTF_reserved
    #[serde(rename = "DMTF_reserved")]
    DMTFReserved = 34,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 35,
}

impl Default for AllocationCapabilities_ResourceType {
    fn default() -> Self {
        Self::Other
    }
}

