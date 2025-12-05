// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Processor_Family
//////////////////////////////////////////////

/// Processor_Family enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Processor_Family {
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 2,
    /// _8086
    #[serde(rename = "_8086")]
    V8086 = 3,
    /// _80286
    #[serde(rename = "_80286")]
    V80286 = 4,
    /// _80386
    #[serde(rename = "_80386")]
    V80386 = 5,
    /// _80486
    #[serde(rename = "_80486")]
    V80486 = 6,
    /// _8087
    #[serde(rename = "_8087")]
    V8087 = 7,
    /// _80287
    #[serde(rename = "_80287")]
    V80287 = 8,
    /// _80387
    #[serde(rename = "_80387")]
    V80387 = 9,
    /// _80487
    #[serde(rename = "_80487")]
    V80487 = 10,
    /// Pentium_R__brand
    #[serde(rename = "Pentium_R__brand")]
    PentiumRBrand = 11,
    /// Pentium_R__Pro
    #[serde(rename = "Pentium_R__Pro")]
    PentiumRPro = 12,
    /// Pentium_R__II
    #[serde(rename = "Pentium_R__II")]
    PentiumRII = 13,
    /// Pentium_R__processor_with_MMX_TM__technology
    #[serde(rename = "Pentium_R__processor_with_MMX_TM__technology")]
    PentiumRProcessorWithMMXTMTechnology = 14,
    /// Celeron_TM_
    #[serde(rename = "Celeron_TM_")]
    CeleronTM = 15,
    /// Pentium_R__II_Xeon_TM_
    #[serde(rename = "Pentium_R__II_Xeon_TM_")]
    PentiumRIIXeonTM = 16,
    /// Pentium_R__III
    #[serde(rename = "Pentium_R__III")]
    PentiumRIII = 17,
    /// M1_Family
    #[serde(rename = "M1_Family")]
    M1Family = 18,
    /// M2_Family
    #[serde(rename = "M2_Family")]
    M2Family = 19,
    /// Intel_R__Celeron_R__M_processor
    #[serde(rename = "Intel_R__Celeron_R__M_processor")]
    IntelRCeleronRMProcessor = 20,
    /// Intel_R__Pentium_R__4_HT_processor
    #[serde(rename = "Intel_R__Pentium_R__4_HT_processor")]
    IntelRPentiumR4HTProcessor = 21,
    /// K5_Family
    #[serde(rename = "K5_Family")]
    K5Family = 24,
    /// K6_Family
    #[serde(rename = "K6_Family")]
    K6Family = 25,
    /// K6_2
    #[serde(rename = "K6_2")]
    K62 = 26,
    /// K6_3
    #[serde(rename = "K6_3")]
    K63 = 27,
    /// AMD_Athlon_TM__Processor_Family
    #[serde(rename = "AMD_Athlon_TM__Processor_Family")]
    AMDAthlonTMProcessorFamily = 28,
    /// AMD_R__Duron_TM__Processor
    #[serde(rename = "AMD_R__Duron_TM__Processor")]
    AMDRDuronTMProcessor = 29,
    /// AMD29000_Family
    #[serde(rename = "AMD29000_Family")]
    AMD29000Family = 30,
    /// K6_2plus
    #[serde(rename = "K6_2plus")]
    K62plus = 31,
    /// Power_PC_Family
    #[serde(rename = "Power_PC_Family")]
    PowerPCFamily = 32,
    /// Power_PC_601
    #[serde(rename = "Power_PC_601")]
    PowerPC601 = 33,
    /// Power_PC_603
    #[serde(rename = "Power_PC_603")]
    PowerPC603 = 34,
    /// Power_PC_603plus
    #[serde(rename = "Power_PC_603plus")]
    PowerPC603plus = 35,
    /// Power_PC_604
    #[serde(rename = "Power_PC_604")]
    PowerPC604 = 36,
    /// Power_PC_620
    #[serde(rename = "Power_PC_620")]
    PowerPC620 = 37,
    /// Power_PC_X704
    #[serde(rename = "Power_PC_X704")]
    PowerPCX704 = 38,
    /// Power_PC_750
    #[serde(rename = "Power_PC_750")]
    PowerPC750 = 39,
    /// Intel_R__Core_TM__Duo_processor
    #[serde(rename = "Intel_R__Core_TM__Duo_processor")]
    IntelRCoreTMDuoProcessor = 40,
    /// Intel_R__Core_TM__Duo_mobile_processor
    #[serde(rename = "Intel_R__Core_TM__Duo_mobile_processor")]
    IntelRCoreTMDuoMobileProcessor = 41,
    /// Intel_R__Core_TM__Solo_mobile_processor
    #[serde(rename = "Intel_R__Core_TM__Solo_mobile_processor")]
    IntelRCoreTMSoloMobileProcessor = 42,
    /// Intel_R__Atom_TM__processor
    #[serde(rename = "Intel_R__Atom_TM__processor")]
    IntelRAtomTMProcessor = 43,
    /// Alpha_Family
    #[serde(rename = "Alpha_Family")]
    AlphaFamily = 48,
    /// Alpha_21064
    #[serde(rename = "Alpha_21064")]
    Alpha21064 = 49,
    /// Alpha_21066
    #[serde(rename = "Alpha_21066")]
    Alpha21066 = 50,
    /// Alpha_21164
    #[serde(rename = "Alpha_21164")]
    Alpha21164 = 51,
    /// Alpha_21164PC
    #[serde(rename = "Alpha_21164PC")]
    Alpha21164PC = 52,
    /// Alpha_21164a
    #[serde(rename = "Alpha_21164a")]
    Alpha21164a = 53,
    /// Alpha_21264
    #[serde(rename = "Alpha_21264")]
    Alpha21264 = 54,
    /// Alpha_21364
    #[serde(rename = "Alpha_21364")]
    Alpha21364 = 55,
    /// AMD_Turion_TM__II_Ultra_Dual_Core_Mobile_M_Processor_Family
    #[serde(rename = "AMD_Turion_TM__II_Ultra_Dual_Core_Mobile_M_Processor_Family")]
    AMDTurionTMIIUltraDualCoreMobileMProcessorFamily = 56,
    /// AMD_Turion_TM__II_Dual_Core_Mobile_M_Processor_Family
    #[serde(rename = "AMD_Turion_TM__II_Dual_Core_Mobile_M_Processor_Family")]
    AMDTurionTMIIDualCoreMobileMProcessorFamily = 57,
    /// AMD_Athlon_TM__II_Dual_Core_Mobile_M_Processor_Family
    #[serde(rename = "AMD_Athlon_TM__II_Dual_Core_Mobile_M_Processor_Family")]
    AMDAthlonTMIIDualCoreMobileMProcessorFamily = 58,
    /// AMD_Opteron_TM__6100_Series_Processor
    #[serde(rename = "AMD_Opteron_TM__6100_Series_Processor")]
    AMDOpteronTM6100SeriesProcessor = 59,
    /// AMD_Opteron_TM__4100_Series_Processor
    #[serde(rename = "AMD_Opteron_TM__4100_Series_Processor")]
    AMDOpteronTM4100SeriesProcessor = 60,
    /// MIPS_Family
    #[serde(rename = "MIPS_Family")]
    MIPSFamily = 64,
    /// MIPS_R4000
    #[serde(rename = "MIPS_R4000")]
    MIPSR4000 = 65,
    /// MIPS_R4200
    #[serde(rename = "MIPS_R4200")]
    MIPSR4200 = 66,
    /// MIPS_R4400
    #[serde(rename = "MIPS_R4400")]
    MIPSR4400 = 67,
    /// MIPS_R4600
    #[serde(rename = "MIPS_R4600")]
    MIPSR4600 = 68,
    /// MIPS_R10000
    #[serde(rename = "MIPS_R10000")]
    MIPSR10000 = 69,
    /// SPARC_Family
    #[serde(rename = "SPARC_Family")]
    SPARCFamily = 80,
    /// SuperSPARC
    #[serde(rename = "SuperSPARC")]
    SuperSPARC = 81,
    /// microSPARC_II
    #[serde(rename = "microSPARC_II")]
    MicroSPARCII = 82,
    /// microSPARC_IIep
    #[serde(rename = "microSPARC_IIep")]
    MicroSPARCIIep = 83,
    /// UltraSPARC
    #[serde(rename = "UltraSPARC")]
    UltraSPARC = 84,
    /// UltraSPARC_II
    #[serde(rename = "UltraSPARC_II")]
    UltraSPARCII = 85,
    /// UltraSPARC_IIi
    #[serde(rename = "UltraSPARC_IIi")]
    UltraSPARCIIi = 86,
    /// UltraSPARC_III
    #[serde(rename = "UltraSPARC_III")]
    UltraSPARCIII = 87,
    /// UltraSPARC_IIIi
    #[serde(rename = "UltraSPARC_IIIi")]
    UltraSPARCIIIi = 88,
    /// _68040
    #[serde(rename = "_68040")]
    V68040 = 96,
    /// _68xxx_Family
    #[serde(rename = "_68xxx_Family")]
    V68xxxFamily = 97,
    /// _68000
    #[serde(rename = "_68000")]
    V68000 = 98,
    /// _68010
    #[serde(rename = "_68010")]
    V68010 = 99,
    /// _68020
    #[serde(rename = "_68020")]
    V68020 = 100,
    /// _68030
    #[serde(rename = "_68030")]
    V68030 = 101,
    /// Hobbit_Family
    #[serde(rename = "Hobbit_Family")]
    HobbitFamily = 112,
    /// Crusoe_TM__TM5000_Family
    #[serde(rename = "Crusoe_TM__TM5000_Family")]
    CrusoeTMTM5000Family = 120,
    /// Crusoe_TM__TM3000_Family
    #[serde(rename = "Crusoe_TM__TM3000_Family")]
    CrusoeTMTM3000Family = 121,
    /// Efficeon_TM__TM8000_Family
    #[serde(rename = "Efficeon_TM__TM8000_Family")]
    EfficeonTMTM8000Family = 122,
    /// Weitek
    #[serde(rename = "Weitek")]
    Weitek = 128,
    /// Itanium_TM__Processor
    #[serde(rename = "Itanium_TM__Processor")]
    ItaniumTMProcessor = 130,
    /// AMD_Athlon_TM__64_Processor_Family
    #[serde(rename = "AMD_Athlon_TM__64_Processor_Family")]
    AMDAthlonTM64ProcessorFamily = 131,
    /// AMD_Opteron_TM__Processor_Family
    #[serde(rename = "AMD_Opteron_TM__Processor_Family")]
    AMDOpteronTMProcessorFamily = 132,
    /// AMD_Sempron_TM__Processor_Family
    #[serde(rename = "AMD_Sempron_TM__Processor_Family")]
    AMDSempronTMProcessorFamily = 133,
    /// AMD_Turion_TM__64_Mobile_Technology
    #[serde(rename = "AMD_Turion_TM__64_Mobile_Technology")]
    AMDTurionTM64MobileTechnology = 134,
    /// Dual_Core_AMD_Opteron_TM__Processor_Family
    #[serde(rename = "Dual_Core_AMD_Opteron_TM__Processor_Family")]
    DualCoreAMDOpteronTMProcessorFamily = 135,
    /// AMD_Athlon_TM__64_X2_Dual_Core_Processor_Family
    #[serde(rename = "AMD_Athlon_TM__64_X2_Dual_Core_Processor_Family")]
    AMDAthlonTM64X2DualCoreProcessorFamily = 136,
    /// AMD_Turion_TM__64_X2_Mobile_Technology
    #[serde(rename = "AMD_Turion_TM__64_X2_Mobile_Technology")]
    AMDTurionTM64X2MobileTechnology = 137,
    /// Quad_Core_AMD_Opteron_TM__Processor_Family
    #[serde(rename = "Quad_Core_AMD_Opteron_TM__Processor_Family")]
    QuadCoreAMDOpteronTMProcessorFamily = 138,
    /// Third_Generation_AMD_Opteron_TM__Processor_Family
    #[serde(rename = "Third_Generation_AMD_Opteron_TM__Processor_Family")]
    ThirdGenerationAMDOpteronTMProcessorFamily = 139,
    /// AMD_Phenom_TM__FX_Quad_Core_Processor_Family
    #[serde(rename = "AMD_Phenom_TM__FX_Quad_Core_Processor_Family")]
    AMDPhenomTMFXQuadCoreProcessorFamily = 140,
    /// AMD_Phenom_TM__X4_Quad_Core_Processor_Family
    #[serde(rename = "AMD_Phenom_TM__X4_Quad_Core_Processor_Family")]
    AMDPhenomTMX4QuadCoreProcessorFamily = 141,
    /// AMD_Phenom_TM__X2_Dual_Core_Processor_Family
    #[serde(rename = "AMD_Phenom_TM__X2_Dual_Core_Processor_Family")]
    AMDPhenomTMX2DualCoreProcessorFamily = 142,
    /// AMD_Athlon_TM__X2_Dual_Core_Processor_Family
    #[serde(rename = "AMD_Athlon_TM__X2_Dual_Core_Processor_Family")]
    AMDAthlonTMX2DualCoreProcessorFamily = 143,
    /// PA_RISC_Family
    #[serde(rename = "PA_RISC_Family")]
    PARISCFamily = 144,
    /// PA_RISC_8500
    #[serde(rename = "PA_RISC_8500")]
    PARISC8500 = 145,
    /// PA_RISC_8000
    #[serde(rename = "PA_RISC_8000")]
    PARISC8000 = 146,
    /// PA_RISC_7300LC
    #[serde(rename = "PA_RISC_7300LC")]
    PARISC7300LC = 147,
    /// PA_RISC_7200
    #[serde(rename = "PA_RISC_7200")]
    PARISC7200 = 148,
    /// PA_RISC_7100LC
    #[serde(rename = "PA_RISC_7100LC")]
    PARISC7100LC = 149,
    /// PA_RISC_7100
    #[serde(rename = "PA_RISC_7100")]
    PARISC7100 = 150,
    /// V30_Family
    #[serde(rename = "V30_Family")]
    V30Family = 160,
    /// Quad_Core_Intel_R__Xeon_R__processor_3200_Series
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor_3200_Series")]
    QuadCoreIntelRXeonRProcessor3200Series = 161,
    /// Dual_Core_Intel_R__Xeon_R__processor_3000_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_3000_Series")]
    DualCoreIntelRXeonRProcessor3000Series = 162,
    /// Quad_Core_Intel_R__Xeon_R__processor_5300_Series
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor_5300_Series")]
    QuadCoreIntelRXeonRProcessor5300Series = 163,
    /// Dual_Core_Intel_R__Xeon_R__processor_5100_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_5100_Series")]
    DualCoreIntelRXeonRProcessor5100Series = 164,
    /// Dual_Core_Intel_R__Xeon_R__processor_5000_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_5000_Series")]
    DualCoreIntelRXeonRProcessor5000Series = 165,
    /// Dual_Core_Intel_R__Xeon_R__processor_LV
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_LV")]
    DualCoreIntelRXeonRProcessorLV = 166,
    /// Dual_Core_Intel_R__Xeon_R__processor_ULV
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_ULV")]
    DualCoreIntelRXeonRProcessorULV = 167,
    /// Dual_Core_Intel_R__Xeon_R__processor_7100_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_7100_Series")]
    DualCoreIntelRXeonRProcessor7100Series = 168,
    /// Quad_Core_Intel_R__Xeon_R__processor_5400_Series
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor_5400_Series")]
    QuadCoreIntelRXeonRProcessor5400Series = 169,
    /// Quad_Core_Intel_R__Xeon_R__processor
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor")]
    QuadCoreIntelRXeonRProcessor = 170,
    /// Dual_Core_Intel_R__Xeon_R__processor_5200_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_5200_Series")]
    DualCoreIntelRXeonRProcessor5200Series = 171,
    /// Dual_Core_Intel_R__Xeon_R__processor_7200_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_7200_Series")]
    DualCoreIntelRXeonRProcessor7200Series = 172,
    /// Quad_Core_Intel_R__Xeon_R__processor_7300_Series
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor_7300_Series")]
    QuadCoreIntelRXeonRProcessor7300Series = 173,
    /// Quad_Core_Intel_R__Xeon_R__processor_7400_Series
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor_7400_Series")]
    QuadCoreIntelRXeonRProcessor7400Series = 174,
    /// Multi_Core_Intel_R__Xeon_R__processor_7400_Series
    #[serde(rename = "Multi_Core_Intel_R__Xeon_R__processor_7400_Series")]
    MultiCoreIntelRXeonRProcessor7400Series = 175,
    /// Pentium_R__III_Xeon_TM_
    #[serde(rename = "Pentium_R__III_Xeon_TM_")]
    PentiumRIIIXeonTM = 176,
    /// Pentium_R__III_Processor_with_Intel_R__SpeedStep_TM__Technology
    #[serde(rename = "Pentium_R__III_Processor_with_Intel_R__SpeedStep_TM__Technology")]
    PentiumRIIIProcessorWithIntelRSpeedStepTMTechnology = 177,
    /// Pentium_R__4
    #[serde(rename = "Pentium_R__4")]
    PentiumR4 = 178,
    /// Intel_R__Xeon_TM_
    #[serde(rename = "Intel_R__Xeon_TM_")]
    IntelRXeonTM = 179,
    /// AS400_Family
    #[serde(rename = "AS400_Family")]
    AS400Family = 180,
    /// Intel_R__Xeon_TM__processor_MP
    #[serde(rename = "Intel_R__Xeon_TM__processor_MP")]
    IntelRXeonTMProcessorMP = 181,
    /// AMD_Athlon_TM__XP_Family
    #[serde(rename = "AMD_Athlon_TM__XP_Family")]
    AMDAthlonTMXPFamily = 182,
    /// AMD_Athlon_TM__MP_Family
    #[serde(rename = "AMD_Athlon_TM__MP_Family")]
    AMDAthlonTMMPFamily = 183,
    /// Intel_R__Itanium_R__2
    #[serde(rename = "Intel_R__Itanium_R__2")]
    IntelRItaniumR2 = 184,
    /// Intel_R__Pentium_R__M_processor
    #[serde(rename = "Intel_R__Pentium_R__M_processor")]
    IntelRPentiumRMProcessor = 185,
    /// Intel_R__Celeron_R__D_processor
    #[serde(rename = "Intel_R__Celeron_R__D_processor")]
    IntelRCeleronRDProcessor = 186,
    /// Intel_R__Pentium_R__D_processor
    #[serde(rename = "Intel_R__Pentium_R__D_processor")]
    IntelRPentiumRDProcessor = 187,
    /// Intel_R__Pentium_R__Processor_Extreme_Edition
    #[serde(rename = "Intel_R__Pentium_R__Processor_Extreme_Edition")]
    IntelRPentiumRProcessorExtremeEdition = 188,
    /// Intel_R__Core_TM__Solo_Processor
    #[serde(rename = "Intel_R__Core_TM__Solo_Processor")]
    IntelRCoreTMSoloProcessor = 189,
    /// K7
    #[serde(rename = "K7")]
    K7 = 190,
    /// Intel_R__Core_TM_2_Duo_Processor
    #[serde(rename = "Intel_R__Core_TM_2_Duo_Processor")]
    IntelRCoreTM2DuoProcessor = 191,
    /// Intel_R__Core_TM_2_Solo_processor
    #[serde(rename = "Intel_R__Core_TM_2_Solo_processor")]
    IntelRCoreTM2SoloProcessor = 192,
    /// Intel_R__Core_TM_2_Extreme_processor
    #[serde(rename = "Intel_R__Core_TM_2_Extreme_processor")]
    IntelRCoreTM2ExtremeProcessor = 193,
    /// Intel_R__Core_TM_2_Quad_processor
    #[serde(rename = "Intel_R__Core_TM_2_Quad_processor")]
    IntelRCoreTM2QuadProcessor = 194,
    /// Intel_R__Core_TM_2_Extreme_mobile_processor
    #[serde(rename = "Intel_R__Core_TM_2_Extreme_mobile_processor")]
    IntelRCoreTM2ExtremeMobileProcessor = 195,
    /// Intel_R__Core_TM_2_Duo_mobile_processor
    #[serde(rename = "Intel_R__Core_TM_2_Duo_mobile_processor")]
    IntelRCoreTM2DuoMobileProcessor = 196,
    /// Intel_R__Core_TM_2_Solo_mobile_processor
    #[serde(rename = "Intel_R__Core_TM_2_Solo_mobile_processor")]
    IntelRCoreTM2SoloMobileProcessor = 197,
    /// Intel_R__Core_TM__i7_processor
    #[serde(rename = "Intel_R__Core_TM__i7_processor")]
    IntelRCoreTMI7Processor = 198,
    /// Dual_Core_Intel_R__Celeron_R__Processor
    #[serde(rename = "Dual_Core_Intel_R__Celeron_R__Processor")]
    DualCoreIntelRCeleronRProcessor = 199,
    /// S_390_and_zSeries_Family
    #[serde(rename = "S_390_and_zSeries_Family")]
    S390AndZSeriesFamily = 200,
    /// ESA_390_G4
    #[serde(rename = "ESA_390_G4")]
    ESA390G4 = 201,
    /// ESA_390_G5
    #[serde(rename = "ESA_390_G5")]
    ESA390G5 = 202,
    /// ESA_390_G6
    #[serde(rename = "ESA_390_G6")]
    ESA390G6 = 203,
    /// z_Architectur_base
    #[serde(rename = "z_Architectur_base")]
    ZArchitecturBase = 204,
    /// Intel_R__Core_TM__i5_processor
    #[serde(rename = "Intel_R__Core_TM__i5_processor")]
    IntelRCoreTMI5Processor = 205,
    /// Intel_R__Core_TM__i3_processor
    #[serde(rename = "Intel_R__Core_TM__i3_processor")]
    IntelRCoreTMI3Processor = 206,
    /// VIA_C7_TM__M_Processor_Family
    #[serde(rename = "VIA_C7_TM__M_Processor_Family")]
    VIAC7TMMProcessorFamily = 210,
    /// VIA_C7_TM__D_Processor_Family
    #[serde(rename = "VIA_C7_TM__D_Processor_Family")]
    VIAC7TMDProcessorFamily = 211,
    /// VIA_C7_TM__Processor_Family
    #[serde(rename = "VIA_C7_TM__Processor_Family")]
    VIAC7TMProcessorFamily = 212,
    /// VIA_Eden_TM__Processor_Family
    #[serde(rename = "VIA_Eden_TM__Processor_Family")]
    VIAEdenTMProcessorFamily = 213,
    /// Multi_Core_Intel_R__Xeon_R__processor
    #[serde(rename = "Multi_Core_Intel_R__Xeon_R__processor")]
    MultiCoreIntelRXeonRProcessor = 214,
    /// Dual_Core_Intel_R__Xeon_R__processor_3xxx_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_3xxx_Series")]
    DualCoreIntelRXeonRProcessor3xxxSeries = 215,
    /// Quad_Core_Intel_R__Xeon_R__processor_3xxx_Series
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor_3xxx_Series")]
    QuadCoreIntelRXeonRProcessor3xxxSeries = 216,
    /// VIA_Nano_TM__Processor_Family
    #[serde(rename = "VIA_Nano_TM__Processor_Family")]
    VIANanoTMProcessorFamily = 217,
    /// Dual_Core_Intel_R__Xeon_R__processor_5xxx_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_5xxx_Series")]
    DualCoreIntelRXeonRProcessor5xxxSeries = 218,
    /// Quad_Core_Intel_R__Xeon_R__processor_5xxx_Series
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor_5xxx_Series")]
    QuadCoreIntelRXeonRProcessor5xxxSeries = 219,
    /// Dual_Core_Intel_R__Xeon_R__processor_7xxx_Series
    #[serde(rename = "Dual_Core_Intel_R__Xeon_R__processor_7xxx_Series")]
    DualCoreIntelRXeonRProcessor7xxxSeries = 221,
    /// Quad_Core_Intel_R__Xeon_R__processor_7xxx_Series
    #[serde(rename = "Quad_Core_Intel_R__Xeon_R__processor_7xxx_Series")]
    QuadCoreIntelRXeonRProcessor7xxxSeries = 222,
    /// Multi_Core_Intel_R__Xeon_R__processor_7xxx_Series
    #[serde(rename = "Multi_Core_Intel_R__Xeon_R__processor_7xxx_Series")]
    MultiCoreIntelRXeonRProcessor7xxxSeries = 223,
    /// Multi_Core_Intel_R__Xeon_R__processor_3400_Series
    #[serde(rename = "Multi_Core_Intel_R__Xeon_R__processor_3400_Series")]
    MultiCoreIntelRXeonRProcessor3400Series = 224,
    /// Embedded_AMD_Opteron_TM__Quad_Core_Processor_Family
    #[serde(rename = "Embedded_AMD_Opteron_TM__Quad_Core_Processor_Family")]
    EmbeddedAMDOpteronTMQuadCoreProcessorFamily = 230,
    /// AMD_Phenom_TM__Triple_Core_Processor_Family
    #[serde(rename = "AMD_Phenom_TM__Triple_Core_Processor_Family")]
    AMDPhenomTMTripleCoreProcessorFamily = 231,
    /// AMD_Turion_TM__Ultra_Dual_Core_Mobile_Processor_Family
    #[serde(rename = "AMD_Turion_TM__Ultra_Dual_Core_Mobile_Processor_Family")]
    AMDTurionTMUltraDualCoreMobileProcessorFamily = 232,
    /// AMD_Turion_TM__Dual_Core_Mobile_Processor_Family
    #[serde(rename = "AMD_Turion_TM__Dual_Core_Mobile_Processor_Family")]
    AMDTurionTMDualCoreMobileProcessorFamily = 233,
    /// AMD_Athlon_TM__Dual_Core_Processor_Family
    #[serde(rename = "AMD_Athlon_TM__Dual_Core_Processor_Family")]
    AMDAthlonTMDualCoreProcessorFamily = 234,
    /// AMD_Sempron_TM__SI_Processor_Family
    #[serde(rename = "AMD_Sempron_TM__SI_Processor_Family")]
    AMDSempronTMSIProcessorFamily = 235,
    /// AMD_Phenom_TM__II_Processor_Family
    #[serde(rename = "AMD_Phenom_TM__II_Processor_Family")]
    AMDPhenomTMIIProcessorFamily = 236,
    /// AMD_Athlon_TM__II_Processor_Family
    #[serde(rename = "AMD_Athlon_TM__II_Processor_Family")]
    AMDAthlonTMIIProcessorFamily = 237,
    /// Six_Core_AMD_Opteron_TM__Processor_Family
    #[serde(rename = "Six_Core_AMD_Opteron_TM__Processor_Family")]
    SixCoreAMDOpteronTMProcessorFamily = 238,
    /// AMD_Sempron_TM__M_Processor_Family
    #[serde(rename = "AMD_Sempron_TM__M_Processor_Family")]
    AMDSempronTMMProcessorFamily = 239,
    /// i860
    #[serde(rename = "i860")]
    I860 = 250,
    /// i960
    #[serde(rename = "i960")]
    I960 = 251,
    /// Reserved__SMBIOS_Extension_
    #[serde(rename = "Reserved__SMBIOS_Extension_")]
    ReservedSMBIOSExtension = 254,
    /// Reserved__Un_initialized_Flash_Content___Lo_
    #[serde(rename = "Reserved__Un_initialized_Flash_Content___Lo_")]
    ReservedUnInitializedFlashContentLo = 255,
    /// SH_3
    #[serde(rename = "SH_3")]
    SH3 = 260,
    /// SH_4
    #[serde(rename = "SH_4")]
    SH4 = 261,
    /// ARM
    #[serde(rename = "ARM")]
    ARM = 280,
    /// StrongARM
    #[serde(rename = "StrongARM")]
    StrongARM = 281,
    /// _6x86
    #[serde(rename = "_6x86")]
    V6x86 = 300,
    /// MediaGX
    #[serde(rename = "MediaGX")]
    MediaGX = 301,
    /// MII
    #[serde(rename = "MII")]
    MII = 302,
    /// WinChip
    #[serde(rename = "WinChip")]
    WinChip = 320,
    /// DSP
    #[serde(rename = "DSP")]
    DSP = 350,
    /// Video_Processor
    #[serde(rename = "Video_Processor")]
    VideoProcessor = 500,
    /// Reserved__For_Future_Special_Purpose_Assignment_
    #[serde(rename = "Reserved__For_Future_Special_Purpose_Assignment_")]
    ReservedForFutureSpecialPurposeAssignment = 65534,
    /// Reserved__Un_initialized_Flash_Content___Hi_
    #[serde(rename = "Reserved__Un_initialized_Flash_Content___Hi_")]
    ReservedUnInitializedFlashContentHi = 65535,
}

impl Default for Processor_Family {
    fn default() -> Self {
        Self::Other
    }
}

