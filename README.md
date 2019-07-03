# bleh
Bosh Luxurious stEmcell uploader, Honestly


This is a _very important_ tool for uploading BOSH stemcells.

## Installing

See releases tab. Not cross-platform (yet?).

## Usage

You must have BOSH CLI installed and working.

See
```
bleh -h
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
