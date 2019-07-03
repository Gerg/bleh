# bleh
Bosh Luxurious stEmcell uploader, Honestly


This is a _very important_ tool for uploading BOSH stemcells.

## Installing

See releases tab. Not cross-platform (yet?).

## Usage

You must have BOSH CLI installed and working.

```
Usage: bleh STEMCELL_VERSION [options]

Options:
    -h, --help          print this help menu
    -i, --infrastructure INFRASTRUCTURE
                        what infrastructure (options: alicloud, aws, azure,
                        gcp, openstack, vcloud, vsphere, bosh-lite; default:
                        bosh-lite)
```

## Dev

### Build
```
make build
```

### Run
```
./target/debug/bleh
```

### Formatting
First Time:
```
rustup component add rustfmt
```

All the Time:
```
make fmt
```
