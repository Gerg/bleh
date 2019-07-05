# bleh
Bosh Luxurious stEmcell uploader, Honestly


This is a _very important_ tool for uploading BOSH stemcells.

## Installing

See releases tab. Built for OSX and Linux.

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

### Release

#### OSX->OSX (or probably any other platform to itself)
```
make release
```

#### OSX->Linux

Note, this will statically link dependencies, so the resulting binary will be
~4x larger than a native build. You should probably just build it natively.

First Time:
```
brew install FiloSottile/musl-cross/musl-cross
```
This will take _forever_ to build.

```
rustup target add x86_64-unknown-linux-musl
```

Thereafter:
```
make release-cross-linux
```

Credit to https://grahamenos.com/rust-osx-linux-musl.html for figuring this out.

### Formatting
First Time:
```
rustup component add rustfmt
```

All the Time:
```
make fmt
```
