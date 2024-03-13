namespace Virt.Net.Tests;

using System;
using System.Runtime.Versioning;
using System.Security.Principal;
using FluentAssertions;
using virt.net;
using Xunit;

[SupportedOSPlatform("windows")]
public class HcsTests
{
    [Fact]
    public void RunElevated()
    {
        var wi = WindowsIdentity.GetCurrent();
        var wp = new WindowsPrincipal(wi);
        var isAdmin = wp.IsInRole(WindowsBuiltInRole.Administrator);
        isAdmin.Should().BeTrue();
    }

    [Fact]
    public void CanGetOsInfo()
    {
        var hcs = new HostComputeSystem(null);
        var (name, version) = hcs.GetOsInfo();
        name.Should().Contain("Microsoft Windows");
        version.Should().NotBeEmpty();
        Version.TryParse(version, out var winVersion).Should().BeTrue();
        winVersion.Should().NotBeNull();
        winVersion!.Major.Should().BeGreaterOrEqualTo(10);
    }

    [Fact]
    public void CanGetHyperVHost()
    {
        var hcs = new HostComputeSystem(null);
        var hv = hcs.GetHyperVHost();
        hv.Should().NotBeNull();
        hv!.Name.Should().Be("vmms");
        hv!.ElementName.Should().Be("Hyper-V Virtual System Management Service");
        hv!.Status.Should().Be("OK");
        hv!.Started.Should().BeTrue();
    }
}