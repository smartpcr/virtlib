// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Error_ProbableCause
//////////////////////////////////////////////

/// Error_ProbableCause enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Error_ProbableCause {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Adapter_Card_Error
    #[serde(rename = "Adapter_Card_Error")]
    AdapterCardError = 2,
    /// Application_Subsystem_Failure
    #[serde(rename = "Application_Subsystem_Failure")]
    ApplicationSubsystemFailure = 3,
    /// Bandwidth_Reduced
    #[serde(rename = "Bandwidth_Reduced")]
    BandwidthReduced = 4,
    /// Connection_Establishment_Error
    #[serde(rename = "Connection_Establishment_Error")]
    ConnectionEstablishmentError = 5,
    /// Communications_Protocol_Error
    #[serde(rename = "Communications_Protocol_Error")]
    CommunicationsProtocolError = 6,
    /// Communications_Subsystem_Failure
    #[serde(rename = "Communications_Subsystem_Failure")]
    CommunicationsSubsystemFailure = 7,
    /// Configuration_Customization_Error
    #[serde(rename = "Configuration_Customization_Error")]
    ConfigurationCustomizationError = 8,
    /// Congestion
    #[serde(rename = "Congestion")]
    Congestion = 9,
    /// Corrupt_Data
    #[serde(rename = "Corrupt_Data")]
    CorruptData = 10,
    /// CPU_Cycles_Limit_Exceeded
    #[serde(rename = "CPU_Cycles_Limit_Exceeded")]
    CPUCyclesLimitExceeded = 11,
    /// Dataset_Modem_Error
    #[serde(rename = "Dataset_Modem_Error")]
    DatasetModemError = 12,
    /// Degraded_Signal
    #[serde(rename = "Degraded_Signal")]
    DegradedSignal = 13,
    /// DTE_DCE_Interface_Error
    #[serde(rename = "DTE_DCE_Interface_Error")]
    DTEDCEInterfaceError = 14,
    /// Enclosure_Door_Open
    #[serde(rename = "Enclosure_Door_Open")]
    EnclosureDoorOpen = 15,
    /// Equipment_Malfunction
    #[serde(rename = "Equipment_Malfunction")]
    EquipmentMalfunction = 16,
    /// Excessive_Vibration
    #[serde(rename = "Excessive_Vibration")]
    ExcessiveVibration = 17,
    /// File_Format_Error
    #[serde(rename = "File_Format_Error")]
    FileFormatError = 18,
    /// Fire_Detected
    #[serde(rename = "Fire_Detected")]
    FireDetected = 19,
    /// Flood_Detected
    #[serde(rename = "Flood_Detected")]
    FloodDetected = 20,
    /// Framing_Error
    #[serde(rename = "Framing_Error")]
    FramingError = 21,
    /// HVAC_Problem
    #[serde(rename = "HVAC_Problem")]
    HVACProblem = 22,
    /// Humidity_Unacceptable
    #[serde(rename = "Humidity_Unacceptable")]
    HumidityUnacceptable = 23,
    /// I_O_Device_Error
    #[serde(rename = "I_O_Device_Error")]
    IODeviceError = 24,
    /// Input_Device_Error
    #[serde(rename = "Input_Device_Error")]
    InputDeviceError = 25,
    /// LAN_Error
    #[serde(rename = "LAN_Error")]
    LANError = 26,
    /// Non_Toxic_Leak_Detected
    #[serde(rename = "Non_Toxic_Leak_Detected")]
    NonToxicLeakDetected = 27,
    /// Local_Node_Transmission_Error
    #[serde(rename = "Local_Node_Transmission_Error")]
    LocalNodeTransmissionError = 28,
    /// Loss_of_Frame
    #[serde(rename = "Loss_of_Frame")]
    LossOfFrame = 29,
    /// Loss_of_Signal
    #[serde(rename = "Loss_of_Signal")]
    LossOfSignal = 30,
    /// Material_Supply_Exhausted
    #[serde(rename = "Material_Supply_Exhausted")]
    MaterialSupplyExhausted = 31,
    /// Multiplexer_Problem
    #[serde(rename = "Multiplexer_Problem")]
    MultiplexerProblem = 32,
    /// Out_of_Memory
    #[serde(rename = "Out_of_Memory")]
    OutOfMemory = 33,
    /// Output_Device_Error
    #[serde(rename = "Output_Device_Error")]
    OutputDeviceError = 34,
    /// Performance_Degraded
    #[serde(rename = "Performance_Degraded")]
    PerformanceDegraded = 35,
    /// Power_Problem
    #[serde(rename = "Power_Problem")]
    PowerProblem = 36,
    /// Pressure_Unacceptable
    #[serde(rename = "Pressure_Unacceptable")]
    PressureUnacceptable = 37,
    /// Processor_Problem__Internal_Machine_Error_
    #[serde(rename = "Processor_Problem__Internal_Machine_Error_")]
    ProcessorProblemInternalMachineError = 38,
    /// Pump_Failure
    #[serde(rename = "Pump_Failure")]
    PumpFailure = 39,
    /// Queue_Size_Exceeded
    #[serde(rename = "Queue_Size_Exceeded")]
    QueueSizeExceeded = 40,
    /// Receive_Failure
    #[serde(rename = "Receive_Failure")]
    ReceiveFailure = 41,
    /// Receiver_Failure
    #[serde(rename = "Receiver_Failure")]
    ReceiverFailure = 42,
    /// Remote_Node_Transmission_Error
    #[serde(rename = "Remote_Node_Transmission_Error")]
    RemoteNodeTransmissionError = 43,
    /// Resource_at_or_Nearing_Capacity
    #[serde(rename = "Resource_at_or_Nearing_Capacity")]
    ResourceAtOrNearingCapacity = 44,
    /// Response_Time_Excessive
    #[serde(rename = "Response_Time_Excessive")]
    ResponseTimeExcessive = 45,
    /// Retransmission_Rate_Excessive
    #[serde(rename = "Retransmission_Rate_Excessive")]
    RetransmissionRateExcessive = 46,
    /// Software_Error
    #[serde(rename = "Software_Error")]
    SoftwareError = 47,
    /// Software_Program_Abnormally_Terminated
    #[serde(rename = "Software_Program_Abnormally_Terminated")]
    SoftwareProgramAbnormallyTerminated = 48,
    /// Software_Program_Error__Incorrect_Results_
    #[serde(rename = "Software_Program_Error__Incorrect_Results_")]
    SoftwareProgramErrorIncorrectResults = 49,
    /// Storage_Capacity_Problem
    #[serde(rename = "Storage_Capacity_Problem")]
    StorageCapacityProblem = 50,
    /// Temperature_Unacceptable
    #[serde(rename = "Temperature_Unacceptable")]
    TemperatureUnacceptable = 51,
    /// Threshold_Crossed
    #[serde(rename = "Threshold_Crossed")]
    ThresholdCrossed = 52,
    /// Timing_Problem
    #[serde(rename = "Timing_Problem")]
    TimingProblem = 53,
    /// Toxic_Leak_Detected
    #[serde(rename = "Toxic_Leak_Detected")]
    ToxicLeakDetected = 54,
    /// Transmit_Failure
    #[serde(rename = "Transmit_Failure")]
    TransmitFailure = 55,
    /// Transmitter_Failure
    #[serde(rename = "Transmitter_Failure")]
    TransmitterFailure = 56,
    /// Underlying_Resource_Unavailable
    #[serde(rename = "Underlying_Resource_Unavailable")]
    UnderlyingResourceUnavailable = 57,
    /// Version_Mismatch
    #[serde(rename = "Version_Mismatch")]
    VersionMismatch = 58,
    /// Previous_Alert_Cleared
    #[serde(rename = "Previous_Alert_Cleared")]
    PreviousAlertCleared = 59,
    /// Login_Attempts_Failed
    #[serde(rename = "Login_Attempts_Failed")]
    LoginAttemptsFailed = 60,
    /// Software_Virus_Detected
    #[serde(rename = "Software_Virus_Detected")]
    SoftwareVirusDetected = 61,
    /// Hardware_Security_Breached
    #[serde(rename = "Hardware_Security_Breached")]
    HardwareSecurityBreached = 62,
    /// Denial_of_Service_Detected
    #[serde(rename = "Denial_of_Service_Detected")]
    DenialOfServiceDetected = 63,
    /// Security_Credential_Mismatch
    #[serde(rename = "Security_Credential_Mismatch")]
    SecurityCredentialMismatch = 64,
    /// Unauthorized_Access
    #[serde(rename = "Unauthorized_Access")]
    UnauthorizedAccess = 65,
    /// Alarm_Received
    #[serde(rename = "Alarm_Received")]
    AlarmReceived = 66,
    /// Loss_of_Pointer
    #[serde(rename = "Loss_of_Pointer")]
    LossOfPointer = 67,
    /// Payload_Mismatch
    #[serde(rename = "Payload_Mismatch")]
    PayloadMismatch = 68,
    /// Transmission_Error
    #[serde(rename = "Transmission_Error")]
    TransmissionError = 69,
    /// Excessive_Error_Rate
    #[serde(rename = "Excessive_Error_Rate")]
    ExcessiveErrorRate = 70,
    /// Trace_Problem
    #[serde(rename = "Trace_Problem")]
    TraceProblem = 71,
    /// Element_Unavailable
    #[serde(rename = "Element_Unavailable")]
    ElementUnavailable = 72,
    /// Element_Missing
    #[serde(rename = "Element_Missing")]
    ElementMissing = 73,
    /// Loss_of_Multi_Frame
    #[serde(rename = "Loss_of_Multi_Frame")]
    LossOfMultiFrame = 74,
    /// Broadcast_Channel_Failure
    #[serde(rename = "Broadcast_Channel_Failure")]
    BroadcastChannelFailure = 75,
    /// Invalid_Message_Received
    #[serde(rename = "Invalid_Message_Received")]
    InvalidMessageReceived = 76,
    /// Routing_Failure
    #[serde(rename = "Routing_Failure")]
    RoutingFailure = 77,
    /// Backplane_Failure
    #[serde(rename = "Backplane_Failure")]
    BackplaneFailure = 78,
    /// Identifier_Duplication
    #[serde(rename = "Identifier_Duplication")]
    IdentifierDuplication = 79,
    /// Protection_Path_Failure
    #[serde(rename = "Protection_Path_Failure")]
    ProtectionPathFailure = 80,
    /// Sync_Loss_or_Mismatch
    #[serde(rename = "Sync_Loss_or_Mismatch")]
    SyncLossOrMismatch = 81,
    /// Terminal_Problem
    #[serde(rename = "Terminal_Problem")]
    TerminalProblem = 82,
    /// Real_Time_Clock_Failure
    #[serde(rename = "Real_Time_Clock_Failure")]
    RealTimeClockFailure = 83,
    /// Antenna_Failure
    #[serde(rename = "Antenna_Failure")]
    AntennaFailure = 84,
    /// Battery_Charging_Failure
    #[serde(rename = "Battery_Charging_Failure")]
    BatteryChargingFailure = 85,
    /// Disk_Failure
    #[serde(rename = "Disk_Failure")]
    DiskFailure = 86,
    /// Frequency_Hopping_Failure
    #[serde(rename = "Frequency_Hopping_Failure")]
    FrequencyHoppingFailure = 87,
    /// Loss_of_Redundancy
    #[serde(rename = "Loss_of_Redundancy")]
    LossOfRedundancy = 88,
    /// Power_Supply_Failure
    #[serde(rename = "Power_Supply_Failure")]
    PowerSupplyFailure = 89,
    /// Signal_Quality_Problem
    #[serde(rename = "Signal_Quality_Problem")]
    SignalQualityProblem = 90,
    /// Battery_Discharging
    #[serde(rename = "Battery_Discharging")]
    BatteryDischarging = 91,
    /// Battery_Failure
    #[serde(rename = "Battery_Failure")]
    BatteryFailure = 92,
    /// Commercial_Power_Problem
    #[serde(rename = "Commercial_Power_Problem")]
    CommercialPowerProblem = 93,
    /// Fan_Failure
    #[serde(rename = "Fan_Failure")]
    FanFailure = 94,
    /// Engine_Failure
    #[serde(rename = "Engine_Failure")]
    EngineFailure = 95,
    /// Sensor_Failure
    #[serde(rename = "Sensor_Failure")]
    SensorFailure = 96,
    /// Fuse_Failure
    #[serde(rename = "Fuse_Failure")]
    FuseFailure = 97,
    /// Generator_Failure
    #[serde(rename = "Generator_Failure")]
    GeneratorFailure = 98,
    /// Low_Battery
    #[serde(rename = "Low_Battery")]
    LowBattery = 99,
    /// Low_Fuel
    #[serde(rename = "Low_Fuel")]
    LowFuel = 100,
    /// Low_Water
    #[serde(rename = "Low_Water")]
    LowWater = 101,
    /// Explosive_Gas
    #[serde(rename = "Explosive_Gas")]
    ExplosiveGas = 102,
    /// High_Winds
    #[serde(rename = "High_Winds")]
    HighWinds = 103,
    /// Ice_Buildup
    #[serde(rename = "Ice_Buildup")]
    IceBuildup = 104,
    /// Smoke
    #[serde(rename = "Smoke")]
    Smoke = 105,
    /// Memory_Mismatch
    #[serde(rename = "Memory_Mismatch")]
    MemoryMismatch = 106,
    /// Out_of_CPU_Cycles
    #[serde(rename = "Out_of_CPU_Cycles")]
    OutOfCPUCycles = 107,
    /// Software_Environment_Problem
    #[serde(rename = "Software_Environment_Problem")]
    SoftwareEnvironmentProblem = 108,
    /// Software_Download_Failure
    #[serde(rename = "Software_Download_Failure")]
    SoftwareDownloadFailure = 109,
    /// Element_Reinitialized
    #[serde(rename = "Element_Reinitialized")]
    ElementReinitialized = 110,
    /// Timeout
    #[serde(rename = "Timeout")]
    Timeout = 111,
    /// Logging_Problems
    #[serde(rename = "Logging_Problems")]
    LoggingProblems = 112,
    /// Leak_Detected
    #[serde(rename = "Leak_Detected")]
    LeakDetected = 113,
    /// Protection_Mechanism_Failure
    #[serde(rename = "Protection_Mechanism_Failure")]
    ProtectionMechanismFailure = 114,
    /// Protecting_Resource_Failure
    #[serde(rename = "Protecting_Resource_Failure")]
    ProtectingResourceFailure = 115,
    /// Database_Inconsistency
    #[serde(rename = "Database_Inconsistency")]
    DatabaseInconsistency = 116,
    /// Authentication_Failure
    #[serde(rename = "Authentication_Failure")]
    AuthenticationFailure = 117,
    /// Breach_of_Confidentiality
    #[serde(rename = "Breach_of_Confidentiality")]
    BreachOfConfidentiality = 118,
    /// Cable_Tamper
    #[serde(rename = "Cable_Tamper")]
    CableTamper = 119,
    /// Delayed_Information
    #[serde(rename = "Delayed_Information")]
    DelayedInformation = 120,
    /// Duplicate_Information
    #[serde(rename = "Duplicate_Information")]
    DuplicateInformation = 121,
    /// Information_Missing
    #[serde(rename = "Information_Missing")]
    InformationMissing = 122,
    /// Information_Modification
    #[serde(rename = "Information_Modification")]
    InformationModification = 123,
    /// Information_Out_of_Sequence
    #[serde(rename = "Information_Out_of_Sequence")]
    InformationOutOfSequence = 124,
    /// Key_Expired
    #[serde(rename = "Key_Expired")]
    KeyExpired = 125,
    /// Non_Repudiation_Failure
    #[serde(rename = "Non_Repudiation_Failure")]
    NonRepudiationFailure = 126,
    /// Out_of_Hours_Activity
    #[serde(rename = "Out_of_Hours_Activity")]
    OutOfHoursActivity = 127,
    /// Out_of_Service
    #[serde(rename = "Out_of_Service")]
    OutOfService = 128,
    /// Procedural_Error
    #[serde(rename = "Procedural_Error")]
    ProceduralError = 129,
    /// Unexpected_Information
    #[serde(rename = "Unexpected_Information")]
    UnexpectedInformation = 130,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 131,
}

impl Default for Error_ProbableCause {
    fn default() -> Self {
        Self::Unknown
    }
}

