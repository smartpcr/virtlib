// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Controller_ProtocolSupported
//////////////////////////////////////////////

/// Controller_ProtocolSupported enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Controller_ProtocolSupported {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// EISA
    #[serde(rename = "EISA")]
    EISA = 3,
    /// ISA
    #[serde(rename = "ISA")]
    ISA = 4,
    /// PCI
    #[serde(rename = "PCI")]
    PCI = 5,
    /// ATA_ATAPI
    #[serde(rename = "ATA_ATAPI")]
    ATAATAPI = 6,
    /// Flexible_Diskette
    #[serde(rename = "Flexible_Diskette")]
    FlexibleDiskette = 7,
    /// _1496
    #[serde(rename = "_1496")]
    V1496 = 8,
    /// SCSI_Parallel_Interface
    #[serde(rename = "SCSI_Parallel_Interface")]
    SCSIParallelInterface = 9,
    /// SCSI_Fibre_Channel_Protocol
    #[serde(rename = "SCSI_Fibre_Channel_Protocol")]
    SCSIFibreChannelProtocol = 10,
    /// SCSI_Serial_Bus_Protocol
    #[serde(rename = "SCSI_Serial_Bus_Protocol")]
    SCSISerialBusProtocol = 11,
    /// SCSI_Serial_Bus_Protocol_2__1394_
    #[serde(rename = "SCSI_Serial_Bus_Protocol_2__1394_")]
    SCSISerialBusProtocol21394 = 12,
    /// SCSI_Serial_Storage_Architecture
    #[serde(rename = "SCSI_Serial_Storage_Architecture")]
    SCSISerialStorageArchitecture = 13,
    /// VESA
    #[serde(rename = "VESA")]
    VESA = 14,
    /// PCMCIA
    #[serde(rename = "PCMCIA")]
    PCMCIA = 15,
    /// Universal_Serial_Bus
    #[serde(rename = "Universal_Serial_Bus")]
    UniversalSerialBus = 16,
    /// Parallel_Protocol
    #[serde(rename = "Parallel_Protocol")]
    ParallelProtocol = 17,
    /// ESCON
    #[serde(rename = "ESCON")]
    ESCON = 18,
    /// Diagnostic
    #[serde(rename = "Diagnostic")]
    Diagnostic = 19,
    /// I2C
    #[serde(rename = "I2C")]
    I2C = 20,
    /// Power
    #[serde(rename = "Power")]
    Power = 21,
    /// HIPPI
    #[serde(rename = "HIPPI")]
    HIPPI = 22,
    /// MultiBus
    #[serde(rename = "MultiBus")]
    MultiBus = 23,
    /// VME
    #[serde(rename = "VME")]
    VME = 24,
    /// IPI
    #[serde(rename = "IPI")]
    IPI = 25,
    /// IEEE_488
    #[serde(rename = "IEEE_488")]
    IEEE488 = 26,
    /// RS232
    #[serde(rename = "RS232")]
    RS232 = 27,
    /// IEEE_802_3_10BASE5
    #[serde(rename = "IEEE_802_3_10BASE5")]
    IEEE802310BASE5 = 28,
    /// IEEE_802_3_10BASE2
    #[serde(rename = "IEEE_802_3_10BASE2")]
    IEEE802310BASE2 = 29,
    /// IEEE_802_3_1BASE5
    #[serde(rename = "IEEE_802_3_1BASE5")]
    IEEE80231BASE5 = 30,
    /// IEEE_802_3_10BROAD36
    #[serde(rename = "IEEE_802_3_10BROAD36")]
    IEEE802310BROAD36 = 31,
    /// IEEE_802_3_100BASEVG
    #[serde(rename = "IEEE_802_3_100BASEVG")]
    IEEE8023100BASEVG = 32,
    /// IEEE_802_5_Token_Ring
    #[serde(rename = "IEEE_802_5_Token_Ring")]
    IEEE8025TokenRing = 33,
    /// ANSI_X3T9_5_FDDI
    #[serde(rename = "ANSI_X3T9_5_FDDI")]
    ANSIX3T95FDDI = 34,
    /// MCA
    #[serde(rename = "MCA")]
    MCA = 35,
    /// ESDI
    #[serde(rename = "ESDI")]
    ESDI = 36,
    /// IDE
    #[serde(rename = "IDE")]
    IDE = 37,
    /// CMD
    #[serde(rename = "CMD")]
    CMD = 38,
    /// ST506
    #[serde(rename = "ST506")]
    ST506 = 39,
    /// DSSI
    #[serde(rename = "DSSI")]
    DSSI = 40,
    /// QIC2
    #[serde(rename = "QIC2")]
    QIC2 = 41,
    /// Enhanced_ATA_IDE
    #[serde(rename = "Enhanced_ATA_IDE")]
    EnhancedATAIDE = 42,
    /// AGP
    #[serde(rename = "AGP")]
    AGP = 43,
    /// TWIRP__two_way_infrared_
    #[serde(rename = "TWIRP__two_way_infrared_")]
    TWIRPTwoWayInfrared = 44,
    /// FIR__fast_infrared_
    #[serde(rename = "FIR__fast_infrared_")]
    FIRFastInfrared = 45,
    /// SIR__serial_infrared_
    #[serde(rename = "SIR__serial_infrared_")]
    SIRSerialInfrared = 46,
    /// IrBus
    #[serde(rename = "IrBus")]
    IrBus = 47,
    /// Serial_ATA
    #[serde(rename = "Serial_ATA")]
    SerialATA = 48,
}

impl Default for Controller_ProtocolSupported {
    fn default() -> Self {
        Self::Other
    }
}

