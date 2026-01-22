# ProxSnap - A Snapshot manager & cleanup tool for Proxmox VE Clusters
<img align="left" src="https://cdn.gyptazy.com/img/Prox-Snap-Snapshot-Management-Tool-Proxmox-Clusters-gyptazy.jpg"/>
<br>

<p float="center"><img src="https://img.shields.io/github/license/gyptazy/ProxSnap"/><img src="https://img.shields.io/github/contributors/gyptazy/ProxSnap"/><img src="https://img.shields.io/github/last-commit/gyptazy/ProxSnap/main"/><img src="https://img.shields.io/github/issues-raw/gyptazy/ProxSnap"/><img src="https://img.shields.io/github/issues-pr/gyptazy/ProxSnap"/></p>

ProxSnap (written by [gyptazy](https://gyptazy.com/proxsnap/)) is a lightweight CLI tool for auditing and cleaning up snapshots across Proxmox VE clusters. It follows the same installation and configuration patterns as other tools from the Prox Tools collection, keeping setup simple and predictable.

## Installation

### Debian Repository (Recommended)
The easiest way to install ProxSnap is via the gyptazy open-source solutions Debian repository. This integrates ProxSnap into the system package manager and provides automatic updates.

```
echo "deb https://repo.gyptazy.com/stable /" > /etc/apt/sources.list.d/proxlb.list
wget -O /etc/apt/trusted.gpg.d/proxlb.asc https://repo.gyptazy.com/repository.gpg
apt-get update && apt-get -y install proxsnap
```

After installation, run ProxSnap directly from the CLI:
```
proxsnap
```

### Debian Package
Alternatively, you can download and install the prebuilt .deb package manually.
```
wget https://cdn.gyptazy.com/debian/proxclmc/proxsnap_1.0.0_amd64.deb
dpkg -i proxsnap_1.0.0_amd64.deb
```

## Configuration
By default, ProxSnap looks for its configuration file at:
```
/etc/proxsnap/proxsnap.yaml
```
A custom path can be specified using the -c option.

ProxSnap shares the same configuration layout as ProxLB, allowing both tools to reuse credentials and API settings.

Example configuration:
```
proxmox_api:
  hosts: virt01-de-ne01.dev.gyptazy.com
  user: proxsnap@pam
  token_id: test
  token_secret: ac1079bb-3333-3333-3333-8a3262da6077
  ssl_verification: false
  timeout: 10
```
Authentication is handled via API tokens. SSL verification and timeouts can be adjusted to match your environment.

## Usage

### List Snapshots
List all snapshots across the cluster:
```
proxsnap -l
```
ProxSnap prints a node-by-node overview of all VMs and containers, highlighting guests with snapshots and showing snapshot names, creation dates, and age.

### Find Old Snapshots
Identify snapshots older than a specific date:
```
proxsnap -d 2026-01-04
```
This runs in dry-run mode by default and does not modify anything.

#### Remove Old Snapshots
To remove snapshots older than the given date, add -r:
```
proxsnap -d 2026-01-04 -r
```
Each deletion is logged explicitly. Only snapshots matching the cutoff date are removed.

## Disclaimer
This software is provided “as is”, without warranty of any kind. Use it at your own risk.The authors and contributors are not liable for any damages resulting from its use.