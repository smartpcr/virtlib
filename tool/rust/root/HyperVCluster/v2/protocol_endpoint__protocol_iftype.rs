// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ProtocolEndpoint_ProtocolIFType
//////////////////////////////////////////////

/// ProtocolEndpoint_ProtocolIFType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ProtocolEndpoint_ProtocolIFType {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Regular_1822
    #[serde(rename = "Regular_1822")]
    Regular1822 = 2,
    /// HDH_1822
    #[serde(rename = "HDH_1822")]
    HDH1822 = 3,
    /// DDN_X_25
    #[serde(rename = "DDN_X_25")]
    DDNX25 = 4,
    /// RFC877_X_25
    #[serde(rename = "RFC877_X_25")]
    RFC877X25 = 5,
    /// Ethernet_CSMA_CD
    #[serde(rename = "Ethernet_CSMA_CD")]
    EthernetCSMACD = 6,
    /// ISO_802_3_CSMA_CD
    #[serde(rename = "ISO_802_3_CSMA_CD")]
    ISO8023CSMACD = 7,
    /// ISO_802_4_Token_Bus
    #[serde(rename = "ISO_802_4_Token_Bus")]
    ISO8024TokenBus = 8,
    /// ISO_802_5_Token_Ring
    #[serde(rename = "ISO_802_5_Token_Ring")]
    ISO8025TokenRing = 9,
    /// ISO_802_6_MAN
    #[serde(rename = "ISO_802_6_MAN")]
    ISO8026MAN = 10,
    /// StarLAN
    #[serde(rename = "StarLAN")]
    StarLAN = 11,
    /// Proteon_10Mbit
    #[serde(rename = "Proteon_10Mbit")]
    Proteon10Mbit = 12,
    /// Proteon_80Mbit
    #[serde(rename = "Proteon_80Mbit")]
    Proteon80Mbit = 13,
    /// HyperChannel
    #[serde(rename = "HyperChannel")]
    HyperChannel = 14,
    /// FDDI
    #[serde(rename = "FDDI")]
    FDDI = 15,
    /// LAP_B
    #[serde(rename = "LAP_B")]
    LAPB = 16,
    /// SDLC
    #[serde(rename = "SDLC")]
    SDLC = 17,
    /// DS1
    #[serde(rename = "DS1")]
    DS1 = 18,
    /// E1
    #[serde(rename = "E1")]
    E1 = 19,
    /// Basic_ISDN
    #[serde(rename = "Basic_ISDN")]
    BasicISDN = 20,
    /// Primary_ISDN
    #[serde(rename = "Primary_ISDN")]
    PrimaryISDN = 21,
    /// Proprietary_Point_to_Point_Serial
    #[serde(rename = "Proprietary_Point_to_Point_Serial")]
    ProprietaryPointToPointSerial = 22,
    /// PPP
    #[serde(rename = "PPP")]
    PPP = 23,
    /// Software_Loopback
    #[serde(rename = "Software_Loopback")]
    SoftwareLoopback = 24,
    /// EON
    #[serde(rename = "EON")]
    EON = 25,
    /// Ethernet_3Mbit
    #[serde(rename = "Ethernet_3Mbit")]
    Ethernet3Mbit = 26,
    /// NSIP
    #[serde(rename = "NSIP")]
    NSIP = 27,
    /// SLIP
    #[serde(rename = "SLIP")]
    SLIP = 28,
    /// Ultra
    #[serde(rename = "Ultra")]
    Ultra = 29,
    /// DS3
    #[serde(rename = "DS3")]
    DS3 = 30,
    /// SIP
    #[serde(rename = "SIP")]
    SIP = 31,
    /// Frame_Relay
    #[serde(rename = "Frame_Relay")]
    FrameRelay = 32,
    /// RS_232
    #[serde(rename = "RS_232")]
    RS232 = 33,
    /// Parallel
    #[serde(rename = "Parallel")]
    Parallel = 34,
    /// ARCNet
    #[serde(rename = "ARCNet")]
    ARCNet = 35,
    /// ARCNet_Plus
    #[serde(rename = "ARCNet_Plus")]
    ARCNetPlus = 36,
    /// ATM
    #[serde(rename = "ATM")]
    ATM = 37,
    /// MIO_X_25
    #[serde(rename = "MIO_X_25")]
    MIOX25 = 38,
    /// SONET
    #[serde(rename = "SONET")]
    SONET = 39,
    /// X_25_PLE
    #[serde(rename = "X_25_PLE")]
    X25PLE = 40,
    /// ISO_802_211c
    #[serde(rename = "ISO_802_211c")]
    ISO802211c = 41,
    /// LocalTalk
    #[serde(rename = "LocalTalk")]
    LocalTalk = 42,
    /// SMDS_DXI
    #[serde(rename = "SMDS_DXI")]
    SMDSDXI = 43,
    /// Frame_Relay_Service
    #[serde(rename = "Frame_Relay_Service")]
    FrameRelayService = 44,
    /// V_35
    #[serde(rename = "V_35")]
    V35 = 45,
    /// HSSI
    #[serde(rename = "HSSI")]
    HSSI = 46,
    /// HIPPI
    #[serde(rename = "HIPPI")]
    HIPPI = 47,
    /// Modem
    #[serde(rename = "Modem")]
    Modem = 48,
    /// AAL5
    #[serde(rename = "AAL5")]
    AAL5 = 49,
    /// SONET_Path
    #[serde(rename = "SONET_Path")]
    SONETPath = 50,
    /// SONET_VT
    #[serde(rename = "SONET_VT")]
    SONETVT = 51,
    /// SMDS_ICIP
    #[serde(rename = "SMDS_ICIP")]
    SMDSICIP = 52,
    /// Proprietary_Virtual_Internal
    #[serde(rename = "Proprietary_Virtual_Internal")]
    ProprietaryVirtualInternal = 53,
    /// Proprietary_Multiplexor
    #[serde(rename = "Proprietary_Multiplexor")]
    ProprietaryMultiplexor = 54,
    /// IEEE_802_12
    #[serde(rename = "IEEE_802_12")]
    IEEE80212 = 55,
    /// Fibre_Channel
    #[serde(rename = "Fibre_Channel")]
    FibreChannel = 56,
    /// HIPPI_Interface
    #[serde(rename = "HIPPI_Interface")]
    HIPPIInterface = 57,
    /// Frame_Relay_Interconnect
    #[serde(rename = "Frame_Relay_Interconnect")]
    FrameRelayInterconnect = 58,
    /// ATM_Emulated_LAN_for_802_3
    #[serde(rename = "ATM_Emulated_LAN_for_802_3")]
    ATMEmulatedLANFor8023 = 59,
    /// ATM_Emulated_LAN_for_802_5
    #[serde(rename = "ATM_Emulated_LAN_for_802_5")]
    ATMEmulatedLANFor8025 = 60,
    /// ATM_Emulated_Circuit
    #[serde(rename = "ATM_Emulated_Circuit")]
    ATMEmulatedCircuit = 61,
    /// Fast_Ethernet__100BaseT_
    #[serde(rename = "Fast_Ethernet__100BaseT_")]
    FastEthernet100BaseT = 62,
    /// ISDN
    #[serde(rename = "ISDN")]
    ISDN = 63,
    /// V_11
    #[serde(rename = "V_11")]
    V11 = 64,
    /// V_36
    #[serde(rename = "V_36")]
    V36 = 65,
    /// G703_at_64K
    #[serde(rename = "G703_at_64K")]
    G703At64K = 66,
    /// G703_at_2Mb
    #[serde(rename = "G703_at_2Mb")]
    G703At2Mb = 67,
    /// QLLC
    #[serde(rename = "QLLC")]
    QLLC = 68,
    /// Fast_Ethernet_100BaseFX
    #[serde(rename = "Fast_Ethernet_100BaseFX")]
    FastEthernet100BaseFX = 69,
    /// Channel
    #[serde(rename = "Channel")]
    Channel = 70,
    /// IEEE_802_11
    #[serde(rename = "IEEE_802_11")]
    IEEE80211 = 71,
    /// IBM_260_370_OEMI_Channel
    #[serde(rename = "IBM_260_370_OEMI_Channel")]
    IBM260370OEMIChannel = 72,
    /// ESCON
    #[serde(rename = "ESCON")]
    ESCON = 73,
    /// Data_Link_Switching
    #[serde(rename = "Data_Link_Switching")]
    DataLinkSwitching = 74,
    /// ISDN_S_T_Interface
    #[serde(rename = "ISDN_S_T_Interface")]
    ISDNSTInterface = 75,
    /// ISDN_U_Interface
    #[serde(rename = "ISDN_U_Interface")]
    ISDNUInterface = 76,
    /// LAP_D
    #[serde(rename = "LAP_D")]
    LAPD = 77,
    /// IP_Switch
    #[serde(rename = "IP_Switch")]
    IPSwitch = 78,
    /// Remote_Source_Route_Bridging
    #[serde(rename = "Remote_Source_Route_Bridging")]
    RemoteSourceRouteBridging = 79,
    /// ATM_Logical
    #[serde(rename = "ATM_Logical")]
    ATMLogical = 80,
    /// DS0
    #[serde(rename = "DS0")]
    DS0 = 81,
    /// DS0_Bundle
    #[serde(rename = "DS0_Bundle")]
    DS0Bundle = 82,
    /// BSC
    #[serde(rename = "BSC")]
    BSC = 83,
    /// Async
    #[serde(rename = "Async")]
    AsyncValue = 84,
    /// Combat_Net_Radio
    #[serde(rename = "Combat_Net_Radio")]
    CombatNetRadio = 85,
    /// ISO_802_5r_DTR
    #[serde(rename = "ISO_802_5r_DTR")]
    ISO8025rDTR = 86,
    /// Ext_Pos_Loc_Report_System
    #[serde(rename = "Ext_Pos_Loc_Report_System")]
    ExtPosLocReportSystem = 87,
    /// AppleTalk_Remote_Access_Protocol
    #[serde(rename = "AppleTalk_Remote_Access_Protocol")]
    AppleTalkRemoteAccessProtocol = 88,
    /// Proprietary_Connectionless
    #[serde(rename = "Proprietary_Connectionless")]
    ProprietaryConnectionless = 89,
    /// ITU_X_29_Host_PAD
    #[serde(rename = "ITU_X_29_Host_PAD")]
    ITUX29HostPAD = 90,
    /// ITU_X_3_Terminal_PAD
    #[serde(rename = "ITU_X_3_Terminal_PAD")]
    ITUX3TerminalPAD = 91,
    /// Frame_Relay_MPI
    #[serde(rename = "Frame_Relay_MPI")]
    FrameRelayMPI = 92,
    /// ITU_X_213
    #[serde(rename = "ITU_X_213")]
    ITUX213 = 93,
    /// ADSL
    #[serde(rename = "ADSL")]
    ADSL = 94,
    /// RADSL
    #[serde(rename = "RADSL")]
    RADSL = 95,
    /// SDSL
    #[serde(rename = "SDSL")]
    SDSL = 96,
    /// VDSL
    #[serde(rename = "VDSL")]
    VDSL = 97,
    /// ISO_802_5_CRFP
    #[serde(rename = "ISO_802_5_CRFP")]
    ISO8025CRFP = 98,
    /// Myrinet
    #[serde(rename = "Myrinet")]
    Myrinet = 99,
    /// Voice_Receive_and_Transmit
    #[serde(rename = "Voice_Receive_and_Transmit")]
    VoiceReceiveAndTransmit = 100,
    /// Voice_Foreign_Exchange_Office
    #[serde(rename = "Voice_Foreign_Exchange_Office")]
    VoiceForeignExchangeOffice = 101,
    /// Voice_Foreign_Exchange_Service
    #[serde(rename = "Voice_Foreign_Exchange_Service")]
    VoiceForeignExchangeService = 102,
    /// Voice_Encapsulation
    #[serde(rename = "Voice_Encapsulation")]
    VoiceEncapsulation = 103,
    /// Voice_over_IP
    #[serde(rename = "Voice_over_IP")]
    VoiceOverIP = 104,
    /// ATM_DXI
    #[serde(rename = "ATM_DXI")]
    ATMDXI = 105,
    /// ATM_FUNI
    #[serde(rename = "ATM_FUNI")]
    ATMFUNI = 106,
    /// ATM_IMA
    #[serde(rename = "ATM_IMA")]
    ATMIMA = 107,
    /// PPP_Multilink_Bundle
    #[serde(rename = "PPP_Multilink_Bundle")]
    PPPMultilinkBundle = 108,
    /// IP_over_CDLC
    #[serde(rename = "IP_over_CDLC")]
    IPOverCDLC = 109,
    /// IP_over_CLAW
    #[serde(rename = "IP_over_CLAW")]
    IPOverCLAW = 110,
    /// Stack_to_Stack
    #[serde(rename = "Stack_to_Stack")]
    StackToStack = 111,
    /// Virtual_IP_Address
    #[serde(rename = "Virtual_IP_Address")]
    VirtualIPAddress = 112,
    /// MPC
    #[serde(rename = "MPC")]
    MPC = 113,
    /// IP_over_ATM
    #[serde(rename = "IP_over_ATM")]
    IPOverATM = 114,
    /// ISO_802_5j_Fibre_Token_Ring
    #[serde(rename = "ISO_802_5j_Fibre_Token_Ring")]
    ISO8025jFibreTokenRing = 115,
    /// TDLC
    #[serde(rename = "TDLC")]
    TDLC = 116,
    /// Gigabit_Ethernet
    #[serde(rename = "Gigabit_Ethernet")]
    GigabitEthernet = 117,
    /// HDLC
    #[serde(rename = "HDLC")]
    HDLC = 118,
    /// LAP_F
    #[serde(rename = "LAP_F")]
    LAPF = 119,
    /// V_37
    #[serde(rename = "V_37")]
    V37 = 120,
    /// X_25_MLP
    #[serde(rename = "X_25_MLP")]
    X25MLP = 121,
    /// X_25_Hunt_Group
    #[serde(rename = "X_25_Hunt_Group")]
    X25HuntGroup = 122,
    /// Transp_HDLC
    #[serde(rename = "Transp_HDLC")]
    TranspHDLC = 123,
    /// Interleave_Channel
    #[serde(rename = "Interleave_Channel")]
    InterleaveChannel = 124,
    /// FAST_Channel
    #[serde(rename = "FAST_Channel")]
    FASTChannel = 125,
    /// IP__for_APPN_HPR_in_IP_Networks_
    #[serde(rename = "IP__for_APPN_HPR_in_IP_Networks_")]
    IPForAPPNHPRInIPNetworks = 126,
    /// CATV_MAC_Layer
    #[serde(rename = "CATV_MAC_Layer")]
    CATVMACLayer = 127,
    /// CATV_Downstream
    #[serde(rename = "CATV_Downstream")]
    CATVDownstream = 128,
    /// CATV_Upstream
    #[serde(rename = "CATV_Upstream")]
    CATVUpstream = 129,
    /// Avalon_12MPP_Switch
    #[serde(rename = "Avalon_12MPP_Switch")]
    Avalon12MPPSwitch = 130,
    /// Tunnel
    #[serde(rename = "Tunnel")]
    Tunnel = 131,
    /// Coffee
    #[serde(rename = "Coffee")]
    Coffee = 132,
    /// Circuit_Emulation_Service
    #[serde(rename = "Circuit_Emulation_Service")]
    CircuitEmulationService = 133,
    /// ATM_SubInterface
    #[serde(rename = "ATM_SubInterface")]
    ATMSubInterface = 134,
    /// Layer_2_VLAN_using_802_1Q
    #[serde(rename = "Layer_2_VLAN_using_802_1Q")]
    Layer2VLANUsing8021Q = 135,
    /// Layer_3_VLAN_using_IP
    #[serde(rename = "Layer_3_VLAN_using_IP")]
    Layer3VLANUsingIP = 136,
    /// Layer_3_VLAN_using_IPX
    #[serde(rename = "Layer_3_VLAN_using_IPX")]
    Layer3VLANUsingIPX = 137,
    /// Digital_Power_Line
    #[serde(rename = "Digital_Power_Line")]
    DigitalPowerLine = 138,
    /// Multimedia_Mail_over_IP
    #[serde(rename = "Multimedia_Mail_over_IP")]
    MultimediaMailOverIP = 139,
    /// DTM
    #[serde(rename = "DTM")]
    DTM = 140,
    /// DCN
    #[serde(rename = "DCN")]
    DCN = 141,
    /// IP_Forwarding
    #[serde(rename = "IP_Forwarding")]
    IPForwarding = 142,
    /// MSDSL
    #[serde(rename = "MSDSL")]
    MSDSL = 143,
    /// IEEE_1394
    #[serde(rename = "IEEE_1394")]
    IEEE1394 = 144,
    /// IF_GSN_HIPPI_6400
    #[serde(rename = "IF_GSN_HIPPI_6400")]
    IFGSNHIPPI6400 = 145,
    /// DVB_RCC_MAC_Layer
    #[serde(rename = "DVB_RCC_MAC_Layer")]
    DVBRCCMACLayer = 146,
    /// DVB_RCC_Downstream
    #[serde(rename = "DVB_RCC_Downstream")]
    DVBRCCDownstream = 147,
    /// DVB_RCC_Upstream
    #[serde(rename = "DVB_RCC_Upstream")]
    DVBRCCUpstream = 148,
    /// ATM_Virtual
    #[serde(rename = "ATM_Virtual")]
    ATMVirtual = 149,
    /// MPLS_Tunnel
    #[serde(rename = "MPLS_Tunnel")]
    MPLSTunnel = 150,
    /// SRP
    #[serde(rename = "SRP")]
    SRP = 151,
    /// Voice_over_ATM
    #[serde(rename = "Voice_over_ATM")]
    VoiceOverATM = 152,
    /// Voice_over_Frame_Relay
    #[serde(rename = "Voice_over_Frame_Relay")]
    VoiceOverFrameRelay = 153,
    /// ISDL
    #[serde(rename = "ISDL")]
    ISDL = 154,
    /// Composite_Link
    #[serde(rename = "Composite_Link")]
    CompositeLink = 155,
    /// SS7_Signaling_Link
    #[serde(rename = "SS7_Signaling_Link")]
    SS7SignalingLink = 156,
    /// Proprietary_P2P_Wireless
    #[serde(rename = "Proprietary_P2P_Wireless")]
    ProprietaryP2PWireless = 157,
    /// Frame_Forward
    #[serde(rename = "Frame_Forward")]
    FrameForward = 158,
    /// RFC1483_Multiprotocol_over_ATM
    #[serde(rename = "RFC1483_Multiprotocol_over_ATM")]
    RFC1483MultiprotocolOverATM = 159,
    /// USB
    #[serde(rename = "USB")]
    USB = 160,
    /// IEEE_802_3ad_Link_Aggregate
    #[serde(rename = "IEEE_802_3ad_Link_Aggregate")]
    IEEE8023adLinkAggregate = 161,
    /// BGP_Policy_Accounting
    #[serde(rename = "BGP_Policy_Accounting")]
    BGPPolicyAccounting = 162,
    /// FRF__16_Multilink_FR
    #[serde(rename = "FRF__16_Multilink_FR")]
    FRF16MultilinkFR = 163,
    /// H_323_Gatekeeper
    #[serde(rename = "H_323_Gatekeeper")]
    H323Gatekeeper = 164,
    /// H_323_Proxy
    #[serde(rename = "H_323_Proxy")]
    H323Proxy = 165,
    /// MPLS
    #[serde(rename = "MPLS")]
    MPLS = 166,
    /// Multi_Frequency_Signaling_Link
    #[serde(rename = "Multi_Frequency_Signaling_Link")]
    MultiFrequencySignalingLink = 167,
    /// HDSL_2
    #[serde(rename = "HDSL_2")]
    HDSL2 = 168,
    /// S_HDSL
    #[serde(rename = "S_HDSL")]
    SHDSL = 169,
    /// DS1_Facility_Data_Link
    #[serde(rename = "DS1_Facility_Data_Link")]
    DS1FacilityDataLink = 170,
    /// Packet_over_SONET_SDH
    #[serde(rename = "Packet_over_SONET_SDH")]
    PacketOverSONETSDH = 171,
    /// DVB_ASI_Input
    #[serde(rename = "DVB_ASI_Input")]
    DVBASIInput = 172,
    /// DVB_ASI_Output
    #[serde(rename = "DVB_ASI_Output")]
    DVBASIOutput = 173,
    /// Power_Line
    #[serde(rename = "Power_Line")]
    PowerLine = 174,
    /// Non_Facility_Associated_Signaling
    #[serde(rename = "Non_Facility_Associated_Signaling")]
    NonFacilityAssociatedSignaling = 175,
    /// TR008
    #[serde(rename = "TR008")]
    TR008 = 176,
    /// GR303_RDT
    #[serde(rename = "GR303_RDT")]
    GR303RDT = 177,
    /// GR303_IDT
    #[serde(rename = "GR303_IDT")]
    GR303IDT = 178,
    /// ISUP
    #[serde(rename = "ISUP")]
    ISUP = 179,
    /// Proprietary_Wireless_MAC_Layer
    #[serde(rename = "Proprietary_Wireless_MAC_Layer")]
    ProprietaryWirelessMACLayer = 180,
    /// Proprietary_Wireless_Downstream
    #[serde(rename = "Proprietary_Wireless_Downstream")]
    ProprietaryWirelessDownstream = 181,
    /// Proprietary_Wireless_Upstream
    #[serde(rename = "Proprietary_Wireless_Upstream")]
    ProprietaryWirelessUpstream = 182,
    /// HIPERLAN_Type_2
    #[serde(rename = "HIPERLAN_Type_2")]
    HIPERLANType2 = 183,
    /// Proprietary_Broadband_Wireless_Access_Point_to_Mulipoint
    #[serde(rename = "Proprietary_Broadband_Wireless_Access_Point_to_Mulipoint")]
    ProprietaryBroadbandWirelessAccessPointToMulipoint = 184,
    /// SONET_Overhead_Channel
    #[serde(rename = "SONET_Overhead_Channel")]
    SONETOverheadChannel = 185,
    /// Digital_Wrapper_Overhead_Channel
    #[serde(rename = "Digital_Wrapper_Overhead_Channel")]
    DigitalWrapperOverheadChannel = 186,
    /// ATM_Adaptation_Layer_2
    #[serde(rename = "ATM_Adaptation_Layer_2")]
    ATMAdaptationLayer2 = 187,
    /// Radio_MAC
    #[serde(rename = "Radio_MAC")]
    RadioMAC = 188,
    /// ATM_Radio
    #[serde(rename = "ATM_Radio")]
    ATMRadio = 189,
    /// Inter_Machine_Trunk
    #[serde(rename = "Inter_Machine_Trunk")]
    InterMachineTrunk = 190,
    /// MVL_DSL
    #[serde(rename = "MVL_DSL")]
    MVLDSL = 191,
    /// Long_Read_DSL
    #[serde(rename = "Long_Read_DSL")]
    LongReadDSL = 192,
    /// Frame_Relay_DLCI_Endpoint
    #[serde(rename = "Frame_Relay_DLCI_Endpoint")]
    FrameRelayDLCIEndpoint = 193,
    /// ATM_VCI_Endpoint
    #[serde(rename = "ATM_VCI_Endpoint")]
    ATMVCIEndpoint = 194,
    /// Optical_Channel
    #[serde(rename = "Optical_Channel")]
    OpticalChannel = 195,
    /// Optical_Transport
    #[serde(rename = "Optical_Transport")]
    OpticalTransport = 196,
    /// Proprietary_ATM
    #[serde(rename = "Proprietary_ATM")]
    ProprietaryATM = 197,
    /// Voice_over_Cable
    #[serde(rename = "Voice_over_Cable")]
    VoiceOverCable = 198,
    /// Infiniband
    #[serde(rename = "Infiniband")]
    Infiniband = 199,
    /// TE_Link
    #[serde(rename = "TE_Link")]
    TELink = 200,
    /// Q_2931
    #[serde(rename = "Q_2931")]
    Q2931 = 201,
    /// Virtual_Trunk_Group
    #[serde(rename = "Virtual_Trunk_Group")]
    VirtualTrunkGroup = 202,
    /// SIP_Trunk_Group
    #[serde(rename = "SIP_Trunk_Group")]
    SIPTrunkGroup = 203,
    /// SIP_Signaling
    #[serde(rename = "SIP_Signaling")]
    SIPSignaling = 204,
    /// CATV_Upstream_Channel
    #[serde(rename = "CATV_Upstream_Channel")]
    CATVUpstreamChannel = 205,
    /// Econet
    #[serde(rename = "Econet")]
    Econet = 206,
    /// FSAN_155Mb_PON
    #[serde(rename = "FSAN_155Mb_PON")]
    FSAN155MbPON = 207,
    /// FSAN_622Mb_PON
    #[serde(rename = "FSAN_622Mb_PON")]
    FSAN622MbPON = 208,
    /// Transparent_Bridge
    #[serde(rename = "Transparent_Bridge")]
    TransparentBridge = 209,
    /// Line_Group
    #[serde(rename = "Line_Group")]
    LineGroup = 210,
    /// Voice_E_M_Feature_Group
    #[serde(rename = "Voice_E_M_Feature_Group")]
    VoiceEMFeatureGroup = 211,
    /// Voice_FGD_EANA
    #[serde(rename = "Voice_FGD_EANA")]
    VoiceFGDEANA = 212,
    /// Voice_DID
    #[serde(rename = "Voice_DID")]
    VoiceDID = 213,
    /// MPEG_Transport
    #[serde(rename = "MPEG_Transport")]
    MPEGTransport = 214,
    /// _6To4
    #[serde(rename = "_6To4")]
    V6To4 = 215,
    /// GTP
    #[serde(rename = "GTP")]
    GTP = 216,
    /// Paradyne_EtherLoop_1
    #[serde(rename = "Paradyne_EtherLoop_1")]
    ParadyneEtherLoop1 = 217,
    /// Paradyne_EtherLoop_2
    #[serde(rename = "Paradyne_EtherLoop_2")]
    ParadyneEtherLoop2 = 218,
    /// Optical_Channel_Group
    #[serde(rename = "Optical_Channel_Group")]
    OpticalChannelGroup = 219,
    /// HomePNA
    #[serde(rename = "HomePNA")]
    HomePNA = 220,
    /// GFP
    #[serde(rename = "GFP")]
    GFP = 221,
    /// ciscoISLvlan
    #[serde(rename = "ciscoISLvlan")]
    CiscoISLvlan = 222,
    /// actelisMetaLOOP
    #[serde(rename = "actelisMetaLOOP")]
    ActelisMetaLOOP = 223,
    /// Fcip
    #[serde(rename = "Fcip")]
    Fcip = 224,
    /// IANA_Reserved
    #[serde(rename = "IANA_Reserved")]
    IANAReserved = 225,
    /// IPv4
    #[serde(rename = "IPv4")]
    IPv4 = 4096,
    /// IPv6
    #[serde(rename = "IPv6")]
    IPv6 = 4097,
    /// IPv4_v6
    #[serde(rename = "IPv4_v6")]
    IPv4V6 = 4098,
    /// IPX
    #[serde(rename = "IPX")]
    IPX = 4099,
    /// DECnet
    #[serde(rename = "DECnet")]
    DECnet = 4100,
    /// SNA
    #[serde(rename = "SNA")]
    SNA = 4101,
    /// CONP
    #[serde(rename = "CONP")]
    CONP = 4102,
    /// CLNP
    #[serde(rename = "CLNP")]
    CLNP = 4103,
    /// VINES
    #[serde(rename = "VINES")]
    VINES = 4104,
    /// XNS
    #[serde(rename = "XNS")]
    XNS = 4105,
    /// ISDN_B_Channel_Endpoint
    #[serde(rename = "ISDN_B_Channel_Endpoint")]
    ISDNBChannelEndpoint = 4106,
    /// ISDN_D_Channel_Endpoint
    #[serde(rename = "ISDN_D_Channel_Endpoint")]
    ISDNDChannelEndpoint = 4107,
    /// BGP
    #[serde(rename = "BGP")]
    BGP = 4108,
    /// OSPF
    #[serde(rename = "OSPF")]
    OSPF = 4109,
    /// UDP
    #[serde(rename = "UDP")]
    UDP = 4110,
    /// TCP
    #[serde(rename = "TCP")]
    TCP = 4111,
    /// _802_11a
    #[serde(rename = "_802_11a")]
    V80211a = 4112,
    /// _802_11b
    #[serde(rename = "_802_11b")]
    V80211b = 4113,
    /// _802_11g
    #[serde(rename = "_802_11g")]
    V80211g = 4114,
    /// _802_11h
    #[serde(rename = "_802_11h")]
    V80211h = 4115,
    /// NFS
    #[serde(rename = "NFS")]
    NFS = 4200,
    /// CIFS
    #[serde(rename = "CIFS")]
    CIFS = 4201,
    /// DAFS
    #[serde(rename = "DAFS")]
    DAFS = 4202,
    /// WebDAV
    #[serde(rename = "WebDAV")]
    WebDAV = 4203,
    /// HTTP
    #[serde(rename = "HTTP")]
    HTTP = 4204,
    /// FTP
    #[serde(rename = "FTP")]
    FTP = 4205,
    /// NDMP
    #[serde(rename = "NDMP")]
    NDMP = 4300,
    /// Telnet
    #[serde(rename = "Telnet")]
    Telnet = 4400,
    /// SSH
    #[serde(rename = "SSH")]
    SSH = 4401,
    /// SM_CLP
    #[serde(rename = "SM_CLP")]
    SMCLP = 4402,
    /// SMTP
    #[serde(rename = "SMTP")]
    SMTP = 4403,
    /// LDAP
    #[serde(rename = "LDAP")]
    LDAP = 4404,
    /// RDP
    #[serde(rename = "RDP")]
    RDP = 4405,
    /// HTTPS
    #[serde(rename = "HTTPS")]
    HTTPS = 4406,
    /// DMTF_Reserved
    #[serde(rename = "DMTF_Reserved")]
    DMTFReserved = 4407,
    /// Vendor_Reserved
    #[serde(rename = "Vendor_Reserved")]
    VendorReserved = 4408,
}

impl Default for ProtocolEndpoint_ProtocolIFType {
    fn default() -> Self {
        Self::Unknown
    }
}

