namespace VirtLib.Windows.Tests.Steps;

using System;
using System.Runtime.InteropServices;
using System.Security.Principal;
using Windows.Models;
using FluentAssertions;
using Models;
using TechTalk.SpecFlow;

[Binding]
public class SanityCheckSteps
{
    private readonly ScenarioContext _context;

    public SanityCheckSteps(ScenarioContext context)
    {
        _context = context;
    }

    [When(@"check current os platform")]
    public void WhenCheckCurrentOsPlatform()
    {
        var osDesc = RuntimeInformation.OSDescription;
        var osArch = RuntimeInformation.OSArchitecture;
        var osVersion = Environment.OSVersion.VersionString;
        var osPlatform = RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ? OSPlatform.Windows : OSPlatform.Linux;
        _context.Set(new OS
        {
            Platform = osPlatform,
            Architecture = osArch,
            Version = osVersion,
            Description = osDesc
        });
    }

    [Then(@"os platform should be ""(.*)""")]
    public void ThenOsPlatformShouldBe(string platform)
    {
        OSPlatform expectedPlatform = platform switch
        {
            "Windows" => OSPlatform.Windows,
            "Linux" => OSPlatform.Linux,
            _ => throw new ArgumentOutOfRangeException(nameof(platform), platform, null)
        };
        var os = _context.Get<OS>();
        os.Platform.Should().Be(expectedPlatform);
    }

    [When(@"check current user is admin")]
    public void WhenCheckCurrentUserIsAdmin()
    {
        var wi = WindowsIdentity.GetCurrent();
        var wp = new WindowsPrincipal(wi);
        var isAdmin = wp.IsInRole(WindowsBuiltInRole.Administrator);
        _context.Set(isAdmin, "isAdmin");
    }

    [Then(@"current user should be admin")]
    public void ThenCurrentUserShouldBeAdmin()
    {
        var isAdmin = _context.Get<bool>("isAdmin");
        isAdmin.Should().BeTrue();
    }
}