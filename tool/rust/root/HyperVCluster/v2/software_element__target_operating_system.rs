// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SoftwareElement_TargetOperatingSystem
//////////////////////////////////////////////

/// SoftwareElement_TargetOperatingSystem enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SoftwareElement_TargetOperatingSystem {
    /// Unknown
    #[serde(rename = "Unknown")]
    Unknown = 0,
    /// Other
    #[serde(rename = "Other")]
    Other = 1,
    /// MACOS
    #[serde(rename = "MACOS")]
    MACOS = 2,
    /// ATTUNIX
    #[serde(rename = "ATTUNIX")]
    ATTUNIX = 3,
    /// DGUX
    #[serde(rename = "DGUX")]
    DGUX = 4,
    /// DECNT
    #[serde(rename = "DECNT")]
    DECNT = 5,
    /// Tru64_UNIX
    #[serde(rename = "Tru64_UNIX")]
    Tru64UNIX = 6,
    /// OpenVMS
    #[serde(rename = "OpenVMS")]
    OpenVMS = 7,
    /// HPUX
    #[serde(rename = "HPUX")]
    HPUX = 8,
    /// AIX
    #[serde(rename = "AIX")]
    AIX = 9,
    /// MVS
    #[serde(rename = "MVS")]
    MVS = 10,
    /// OS400
    #[serde(rename = "OS400")]
    OS400 = 11,
    /// OS_2
    #[serde(rename = "OS_2")]
    OS2 = 12,
    /// JavaVM
    #[serde(rename = "JavaVM")]
    JavaVM = 13,
    /// MSDOS
    #[serde(rename = "MSDOS")]
    MSDOS = 14,
    /// WIN3x
    #[serde(rename = "WIN3x")]
    WIN3x = 15,
    /// WIN95
    #[serde(rename = "WIN95")]
    WIN95 = 16,
    /// WIN98
    #[serde(rename = "WIN98")]
    WIN98 = 17,
    /// WINNT
    #[serde(rename = "WINNT")]
    WINNT = 18,
    /// WINCE
    #[serde(rename = "WINCE")]
    WINCE = 19,
    /// NCR3000
    #[serde(rename = "NCR3000")]
    NCR3000 = 20,
    /// NetWare
    #[serde(rename = "NetWare")]
    NetWare = 21,
    /// OSF
    #[serde(rename = "OSF")]
    OSF = 22,
    /// DC_OS
    #[serde(rename = "DC_OS")]
    DCOS = 23,
    /// Reliant_UNIX
    #[serde(rename = "Reliant_UNIX")]
    ReliantUNIX = 24,
    /// SCO_UnixWare
    #[serde(rename = "SCO_UnixWare")]
    SCOUnixWare = 25,
    /// SCO_OpenServer
    #[serde(rename = "SCO_OpenServer")]
    SCOOpenServer = 26,
    /// Sequent
    #[serde(rename = "Sequent")]
    Sequent = 27,
    /// IRIX
    #[serde(rename = "IRIX")]
    IRIX = 28,
    /// Solaris
    #[serde(rename = "Solaris")]
    Solaris = 29,
    /// SunOS
    #[serde(rename = "SunOS")]
    SunOS = 30,
    /// U6000
    #[serde(rename = "U6000")]
    U6000 = 31,
    /// ASERIES
    #[serde(rename = "ASERIES")]
    ASERIES = 32,
    /// HP_NonStop_OS
    #[serde(rename = "HP_NonStop_OS")]
    HPNonStopOS = 33,
    /// HP_NonStop_OSS
    #[serde(rename = "HP_NonStop_OSS")]
    HPNonStopOSS = 34,
    /// BS2000
    #[serde(rename = "BS2000")]
    BS2000 = 35,
    /// LINUX
    #[serde(rename = "LINUX")]
    LINUX = 36,
    /// Lynx
    #[serde(rename = "Lynx")]
    Lynx = 37,
    /// XENIX
    #[serde(rename = "XENIX")]
    XENIX = 38,
    /// VM
    #[serde(rename = "VM")]
    VM = 39,
    /// Interactive_UNIX
    #[serde(rename = "Interactive_UNIX")]
    InteractiveUNIX = 40,
    /// BSDUNIX
    #[serde(rename = "BSDUNIX")]
    BSDUNIX = 41,
    /// FreeBSD
    #[serde(rename = "FreeBSD")]
    FreeBSD = 42,
    /// NetBSD
    #[serde(rename = "NetBSD")]
    NetBSD = 43,
    /// GNU_Hurd
    #[serde(rename = "GNU_Hurd")]
    GNUHurd = 44,
    /// OS9
    #[serde(rename = "OS9")]
    OS9 = 45,
    /// MACH_Kernel
    #[serde(rename = "MACH_Kernel")]
    MACHKernel = 46,
    /// Inferno
    #[serde(rename = "Inferno")]
    Inferno = 47,
    /// QNX
    #[serde(rename = "QNX")]
    QNX = 48,
    /// EPOC
    #[serde(rename = "EPOC")]
    EPOC = 49,
    /// IxWorks
    #[serde(rename = "IxWorks")]
    IxWorks = 50,
    /// VxWorks
    #[serde(rename = "VxWorks")]
    VxWorks = 51,
    /// MiNT
    #[serde(rename = "MiNT")]
    MiNT = 52,
    /// BeOS
    #[serde(rename = "BeOS")]
    BeOS = 53,
    /// HP_MPE
    #[serde(rename = "HP_MPE")]
    HPMPE = 54,
    /// NextStep
    #[serde(rename = "NextStep")]
    NextStep = 55,
    /// PalmPilot
    #[serde(rename = "PalmPilot")]
    PalmPilot = 56,
    /// Rhapsody
    #[serde(rename = "Rhapsody")]
    Rhapsody = 57,
    /// Windows_2000
    #[serde(rename = "Windows_2000")]
    Windows2000 = 58,
    /// Dedicated
    #[serde(rename = "Dedicated")]
    Dedicated = 59,
    /// OS_390
    #[serde(rename = "OS_390")]
    OS390 = 60,
    /// VSE
    #[serde(rename = "VSE")]
    VSE = 61,
    /// TPF
    #[serde(rename = "TPF")]
    TPF = 62,
    /// Windows__R__Me
    #[serde(rename = "Windows__R__Me")]
    WindowsRMe = 63,
    /// Caldera_Open_UNIX
    #[serde(rename = "Caldera_Open_UNIX")]
    CalderaOpenUNIX = 64,
    /// OpenBSD
    #[serde(rename = "OpenBSD")]
    OpenBSD = 65,
    /// Not_Applicable
    #[serde(rename = "Not_Applicable")]
    NotApplicable = 66,
    /// Windows_XP
    #[serde(rename = "Windows_XP")]
    WindowsXP = 67,
    /// z_OS
    #[serde(rename = "z_OS")]
    ZOS = 68,
    /// Microsoft_Windows_Server_2003
    #[serde(rename = "Microsoft_Windows_Server_2003")]
    MicrosoftWindowsServer2003 = 69,
    /// Microsoft_Windows_Server_2003_64_Bit
    #[serde(rename = "Microsoft_Windows_Server_2003_64_Bit")]
    MicrosoftWindowsServer200364Bit = 70,
    /// Windows_XP_64_Bit
    #[serde(rename = "Windows_XP_64_Bit")]
    WindowsXP64Bit = 71,
    /// Windows_XP_Embedded
    #[serde(rename = "Windows_XP_Embedded")]
    WindowsXPEmbedded = 72,
    /// Windows_Vista
    #[serde(rename = "Windows_Vista")]
    WindowsVista = 73,
    /// Windows_Vista_64_Bit
    #[serde(rename = "Windows_Vista_64_Bit")]
    WindowsVista64Bit = 74,
    /// Windows_Embedded_for_Point_of_Service
    #[serde(rename = "Windows_Embedded_for_Point_of_Service")]
    WindowsEmbeddedForPointOfService = 75,
    /// Microsoft_Windows_Server_2008
    #[serde(rename = "Microsoft_Windows_Server_2008")]
    MicrosoftWindowsServer2008 = 76,
    /// Microsoft_Windows_Server_2008_64_Bit
    #[serde(rename = "Microsoft_Windows_Server_2008_64_Bit")]
    MicrosoftWindowsServer200864Bit = 77,
    /// FreeBSD_64_Bit
    #[serde(rename = "FreeBSD_64_Bit")]
    FreeBSD64Bit = 78,
    /// RedHat_Enterprise_Linux
    #[serde(rename = "RedHat_Enterprise_Linux")]
    RedHatEnterpriseLinux = 79,
    /// RedHat_Enterprise_Linux_64_Bit
    #[serde(rename = "RedHat_Enterprise_Linux_64_Bit")]
    RedHatEnterpriseLinux64Bit = 80,
    /// Solaris_64_Bit
    #[serde(rename = "Solaris_64_Bit")]
    Solaris64Bit = 81,
    /// SUSE
    #[serde(rename = "SUSE")]
    SUSE = 82,
    /// SUSE_64_Bit
    #[serde(rename = "SUSE_64_Bit")]
    SUSE64Bit = 83,
    /// SLES
    #[serde(rename = "SLES")]
    SLES = 84,
    /// SLES_64_Bit
    #[serde(rename = "SLES_64_Bit")]
    SLES64Bit = 85,
    /// Novell_OES
    #[serde(rename = "Novell_OES")]
    NovellOES = 86,
    /// Novell_Linux_Desktop
    #[serde(rename = "Novell_Linux_Desktop")]
    NovellLinuxDesktop = 87,
    /// Sun_Java_Desktop_System
    #[serde(rename = "Sun_Java_Desktop_System")]
    SunJavaDesktopSystem = 88,
    /// Mandriva
    #[serde(rename = "Mandriva")]
    Mandriva = 89,
    /// Mandriva_64_Bit
    #[serde(rename = "Mandriva_64_Bit")]
    Mandriva64Bit = 90,
    /// TurboLinux
    #[serde(rename = "TurboLinux")]
    TurboLinux = 91,
    /// TurboLinux_64_Bit
    #[serde(rename = "TurboLinux_64_Bit")]
    TurboLinux64Bit = 92,
    /// Ubuntu
    #[serde(rename = "Ubuntu")]
    Ubuntu = 93,
    /// Ubuntu_64_Bit
    #[serde(rename = "Ubuntu_64_Bit")]
    Ubuntu64Bit = 94,
    /// Debian
    #[serde(rename = "Debian")]
    Debian = 95,
    /// Debian_64_Bit
    #[serde(rename = "Debian_64_Bit")]
    Debian64Bit = 96,
    /// Linux_2_4_x
    #[serde(rename = "Linux_2_4_x")]
    Linux24X = 97,
    /// Linux_2_4_x_64_Bit
    #[serde(rename = "Linux_2_4_x_64_Bit")]
    Linux24X64Bit = 98,
    /// Linux_2_6_x
    #[serde(rename = "Linux_2_6_x")]
    Linux26X = 99,
    /// Linux_2_6_x_64_Bit
    #[serde(rename = "Linux_2_6_x_64_Bit")]
    Linux26X64Bit = 100,
    /// Linux_64_Bit
    #[serde(rename = "Linux_64_Bit")]
    Linux64Bit = 101,
    /// Other_64_Bit
    #[serde(rename = "Other_64_Bit")]
    Other64Bit = 102,
    /// Microsoft_Windows_Server_2008_R2
    #[serde(rename = "Microsoft_Windows_Server_2008_R2")]
    MicrosoftWindowsServer2008R2 = 103,
    /// VMware_ESXi
    #[serde(rename = "VMware_ESXi")]
    VMwareESXi = 104,
    /// Microsoft_Windows_7
    #[serde(rename = "Microsoft_Windows_7")]
    MicrosoftWindows7 = 105,
    /// CentOS_32_bit
    #[serde(rename = "CentOS_32_bit")]
    CentOS32Bit = 106,
    /// CentOS_64_bit
    #[serde(rename = "CentOS_64_bit")]
    CentOS64Bit = 107,
    /// Oracle_Enterprise_Linux_32_bit
    #[serde(rename = "Oracle_Enterprise_Linux_32_bit")]
    OracleEnterpriseLinux32Bit = 108,
    /// Oracle_Enterprise_Linux_64_bit
    #[serde(rename = "Oracle_Enterprise_Linux_64_bit")]
    OracleEnterpriseLinux64Bit = 109,
    /// eComStation_32_bitx
    #[serde(rename = "eComStation_32_bitx")]
    EComStation32Bitx = 110,
}

impl Default for SoftwareElement_TargetOperatingSystem {
    fn default() -> Self {
        Self::Unknown
    }
}

