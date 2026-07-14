# P7b-1b Alternative Containment Route

## Decision purpose

The existing LPAC diagnostic family is terminal. It proved that the broker can
create and verify the suspended zero-capability process and can now retain a
bounded debug trace, but the static canary still exits `0xC0000142` before its
denial probes. No further AppContainer retry, canary permutation, or diagnostic
is eligible.

This record designs an alternative; it does not install, enable, upgrade, or
execute one. The Forge compiler and protected Kernel are not being replaced.
The blocked component is the disposable execution boundary required before a
future renderer or other visual tool can be treated as contained.

## Current host evidence

Read-only inspection on 2026-07-13 records:

- Windows `EditionID=Core` (Home), build `26200.8655`, x64;
- a hypervisor is detected, but Hyper-V feature state cannot be queried without
  elevation and Microsoft does not support the Hyper-V role on Home;
- `wsl.exe --list --verbose` reports WSL is not installed; and
- neither Docker nor Podman is present.

These facts are host-specific and may be rechecked later. They grant no
authority to upgrade Windows, enable a feature, reboot, install a runtime,
download an image, or create a VM.

## Primary-source reconciliation

Sources were checked on 2026-07-13.

| Source | Retained requirement or limit |
|---|---|
| [Windows Sandbox](https://learn.microsoft.com/en-in/windows/security/application-security/application-isolation/windows-sandbox/) | Windows Sandbox is a disposable VM with a separate kernel and hardware-backed isolation. Supported editions include Pro, Enterprise, and Education; Home is not listed. Closing it discards its state. |
| [Configure Windows Sandbox](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-configure-using-wsb-file) | Networking and clipboard are enabled by default and must be disabled. Host mappings enlarge risk; input must be read-only and output must be one fresh bounded quarantine. vGPU can be disabled, falling back to WARP, to reduce attack surface. |
| [Windows Sandbox CLI](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-cli) | On Windows 11 24H2 and later, the CLI can create/control sandboxes, share folders, execute a command, and return its exit code. Process output is not returned, so admitted evidence still needs an explicit file receipt. |
| [Install Windows Sandbox](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-install) | Enabling the built-in feature requires virtualization prerequisites, administrator authority, and may require restart. None is authorized by this design. |
| [Hyper-V installation](https://learn.microsoft.com/en-us/windows-server/virtualization/hyper-v/get-started/Install-Hyper-V) | Client Hyper-V requires Pro or Enterprise and cannot be installed on Home. A full VM therefore has the same immediate edition gate plus guest-image lifecycle cost. |
| [Windows container requirements](https://learn.microsoft.com/en-us/virtualization/windowscontainers/deploy-containers/system-requirements) | Windows containers require Pro/Enterprise on client and Hyper-V for Hyper-V isolation. A container runtime and compatible image are additional dependencies. |
| [Windows container FAQ](https://learn.microsoft.com/en-us/virtualization/windowscontainers/about/faq) | Hyper-V isolation is the container mode Microsoft describes for untrusted-code separation. Process isolation is not an equivalent replacement. |
| [GPU acceleration in Windows containers](https://learn.microsoft.com/en-us/virtualization/windowscontainers/deploy-containers/gpu-acceleration) | Hyper-V-isolated Windows containers do not currently support GPU acceleration, making them a poor first visual-tool route. |
| [Create Process in Sandbox](https://learn.microsoft.com/en-us/windows/win32/secauthz/createprocessinsandbox) | The composable API remains experimental, has no public header, and uses AppContainer as the enforcement base for filesystem, network, and capability restrictions. It does not escape the failed startup family. |
| [Win32 application isolation](https://learn.microsoft.com/en-us/windows/security/book/application-security-application-isolation) | Packaged Win32 isolation is also AppContainer-based. WSL is presented as an integrated developer environment, not as the selected Windows untrusted-tool boundary. |

## Alternative comparison

| Route | Security and compatibility | Host/change cost | Decision |
|---|---|---|---|
| **A. Windows Sandbox disposable-VM protocol** | Separate Windows kernel, disposable state, Windows binary compatibility, WARP rendering with vGPU disabled | Requires a supported edition or another eligible host, feature enablement, admin action, and possibly restart | **Recommended design** |
| B. Hyper-V-isolated Windows container | Strong VM-backed container boundary and Windows compatibility | Requires Pro/Enterprise, Hyper-V, runtime installation, base images, version/licensing controls; no Hyper-V-isolated GPU acceleration | Defer; more supply-chain and lifecycle surface than Sandbox |
| C. Full Hyper-V VM | Strong explicit partition boundary and flexible guest | Requires Pro/Enterprise, VM image/license, patching, storage, snapshot, and teardown lifecycle | Reserve for tools Windows Sandbox cannot run |
| D. Experimental composable sandbox or packaged Win32 isolation | Useful future APIs but still AppContainer-rooted | Experimental/no public header or packaging/Visual Studio installation; does not address current AppContainer startup evidence | Reject for this replacement |
| E. WSL 2 or Linux container | Could support a future Linux-native tool | Not installed; changes tool/OS compatibility and integration surface; does not validate the exact Windows candidate | Reject until an actual Linux tool is selected |
| F. Third-party or cloud VM | Potential independent boundary | New program/provider, credentials, network, image trust, possible spending and publishing concerns | Separate owner program only |
| G. Declare containment unnecessary | No host change | Silently skips the P7b-1b evidence gate | Reject |

## Recommended staged route

### Alt-0 - capability-free Windows Sandbox protocol

This stage may be implemented without enabling or launching Windows Sandbox.
It defines and adversarially validates canonical records for:

- exact supported-host and feature evidence;
- a deny-by-default `.wsb` projection with networking, clipboard, audio input,
  video input, printer redirection, and vGPU disabled;
- protected client enabled where supported;
- exactly one immutable read-only input mapping and one newly created empty
  writable output quarantine mapping;
- explicit absolute guest paths, fixed memory/wall/process limits, and no
  inherited repository, user-content, credential, device, or external-network
  paths;
- one content-bound candidate, configuration, command, environment, result,
  teardown, and host-inventory receipt;
- strict output admission after sandbox termination; and
- no claim that protocol validation proves runtime isolation.

The first proof uses WARP, not vGPU. Enabling vGPU is a later, separate attack-
surface decision and cannot be inherited from a CPU-rendering pass.

### Alt-1 - host eligibility owner gate

After Alt-0 is verified, the owner chooses one of:

1. provide an already eligible Pro/Enterprise/Education Windows Sandbox host;
2. separately authorize and perform an edition/feature change on this host; or
3. leave executable perception blocked.

Codex must not buy an edition, enable Windows Sandbox, elevate, reboot, or move
work to another machine without that explicit choice. An edition change is not
treated as a routine test.

### Alt-2 - disposable-VM denial canary

Only after Alt-1 evidence passes and a fresh run is separately authorized:

1. create fresh host input and output quarantine directories outside the repo;
2. bind a source-reviewed canary and deny-by-default configuration;
3. launch one fresh Windows Sandbox instance with network and all redirections
   disabled and vGPU off;
4. execute the canary once using the Sandbox CLI, retaining the exit code and a
   bounded file receipt because CLI stdout is unavailable;
5. close the entire sandbox, then inspect only the fresh output quarantine;
6. verify repository/user sentinels and host process/network/configuration
   inventories are unchanged; and
7. remove only exact owned quarantine after resolved-path verification.

The canary may verify that only the declared input/output mappings exist,
external networking is unavailable, undeclared host paths are absent, and the
bounded output succeeds. Child processes are contained inside the disposable
VM rather than required to fail; guest-side job limits bound the test process
tree. A pass proves only this canary/configuration/host build.

### Alt-3 - renderer compatibility

P7b-1c remains separately gated. It may use the proven Sandbox profile first
with WARP and one exact locally reviewed renderer/tool. Imported assets, GPU,
vGPU, downloads, package installation, external network, and product claims
remain separate decisions.

## Adversarial failure matrix

| Failure | Required outcome |
|---|---|
| Unsupported edition, missing feature, or unverifiable build | Stop before configuration or launch. |
| Default networking, clipboard, vGPU, or redirection remains enabled | Reject configuration. |
| Input mapping is writable or output mapping is reused/nonempty | Reject before launch. |
| Repository, home directory, credentials, device, or broad drive is mapped | Reject configuration. |
| Sandbox CLI cannot return a bound instance/exit result | Fail; do not infer success from window state. |
| Any unexpected output, reparse point, stream, executable, or oversized receipt | Quarantine metadata only; no preview or admission. |
| Sandbox does not terminate or host residue remains | Block later trials and report exact residue. |
| vGPU is needed for the first canary | Stop; do not enlarge attack surface to rescue compatibility. |
| CPU/WARP canary passes | Proves only exact disposable-VM containment; renderer remains gated. |
| Owner declines host change | Keep P7b-1b, P7b-1c, F5 completion, and G1 gated. |

## P10 boundary

- **Baseline:** current LPAC route reaches only core DLL load and
  `0xC0000142`; denial remains unproved.
- **Expected gain:** a separate-kernel disposable Windows environment that can
  run ordinary Win32 binaries without weakening the failed LPAC profile.
- **Design cost:** one capability-free protocol/reference package now.
- **Future operating cost:** supported Windows edition/host, feature lifecycle,
  one disposable VM run, and strict quarantine handling.
- **Uncertainty:** the exact future renderer may not work under WARP or Windows
  Sandbox, and this Home host is not currently eligible.
- **Regression guard:** Alt-0 cannot call Sandbox, DISM, WSL, Docker, Hyper-V,
  package managers, network clients, or protected Kernel paths.
- **Stop/refocus:** if an eligible host cannot be supplied or authorized, leave
  executable perception blocked; do not fall back to a weaker process boundary.

## Exact owner decision

Recommended response: **`Build the capability-free Windows Sandbox protocol`**.
This authorizes only Alt-0 records, validator, hostile fixtures, and read-only
Forge integration. It does not authorize an edition upgrade, feature enable,
elevation, restart, Sandbox launch, program installation, image download,
renderer, GPU/vGPU, spending, credentials, publishing, promotion, or
protected-Kernel mutation.

Alternative response: **`Leave executable perception blocked`**.

## Superseding owner constraint

The owner subsequently rejected any Windows operating-system edition upgrade.
The Windows Sandbox host-change recommendation is therefore closed. The
no-upgrade dependency rebaseline in `P7B1B_NO_OS_UPGRADE_REBASELINE.md`
supersedes this record's recommended route while retaining its technical
comparison as evidence.
