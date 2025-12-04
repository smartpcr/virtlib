// -----------------------------------------------------------------------
// <copyright file="VirtualMachineDefinition.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace VirtLib.Windows.Definitions;

public class VirtualMachineDefinition
{
    /// <summary>Gets or sets defines the Automatic Start Action settings.</summary>
    public AutomaticStartDefinition AutomaticStart { get; set; } = new AutomaticStartDefinition();

    /// <summary>Gets or sets defines the Automatic Stop Action settings.</summary>
    public AutomaticStopDefinition AutomaticStop { get; set; } = new AutomaticStopDefinition();

    /// <summary>Gets or sets defines the Checkpoints settings.</summary>
    public CheckpointsDefinition Checkpoints { get; set; } = new CheckpointsDefinition();

    /// <summary>Gets or sets defines the Integration Services settings.</summary>
    public IntegrationServicesDefinition IntegrationServices { get; set; } = new IntegrationServicesDefinition();

    /// <summary>Gets or sets defines the Memory settings.</summary>
    public MemoryDefinition Memory { get; set; } = new MemoryDefinition();

    /// <summary>Gets or sets the name of the virtual machine.</summary>
    public string Name { get; set; }

    /// <summary>Gets or sets defines the settings for up to 8 Network Adapters.</summary>
    /// <remarks>The configuration includes one Network Adapter by default.</remarks>
    public NetworkAdaptersDefinition NetworkAdapters { get; set; } = new NetworkAdaptersDefinition();

    /// <summary>Gets or sets notes for the virtual machine.</summary>
    public string Notes { get; set; }

    private string _path;

    /// <summary>Gets or sets the path where the configuration files will be stored.</summary>
    /// <remarks>Setting the configuration path will reset the Checkpoints and Smart Paging paths to their defaults.</remarks>
    public string Path
    {
        get => _path;
        set
        {
            _path = value;
            Checkpoints.Path = value;
            SmartPaging.Path = value;
        }
    }

    public Generation Generation { get; set; } = Generation.Gen2;

    /// <summary>Gets or sets defines the Processor settings.</summary>
    public ProcessorDefinition Processor { get; set; } = new ProcessorDefinition();

    /// <summary>Gets or sets defines the settings for up to 4 SCSI Controllers.</summary>
    /// <remarks>The configuration includes one SCSI Controller by default.</remarks>
    public ScsiControllersDefinition ScsiControllers { get; set; } = new ScsiControllersDefinition();

    /// <summary>Gets or sets defines the Security settings.</summary>
    public SecurityDefinition Security { get; set; } = new SecurityDefinition();

    /// <summary>Gets or sets defines the Smart Paging settings.</summary>
    public SmartPagingDefinition SmartPaging { get; set; } = new SmartPagingDefinition();

    /// <summary>Initializes a new instance of the <see cref="VirtualMachineDefinition"/> class with the specified virtual machine name and configuration path.</summary>
    /// <param name="name">The name of the virtual machine.</param>
    /// <param name="path">The path where the configuration files will be stored.</param>
    public VirtualMachineDefinition(string name, string path)
    {
        Name = name;
        Notes = "";
        Path = path;
        ScsiControllers.Add(new ScsiController());
        NetworkAdapters.Add(new NetworkAdapter());
    }
}