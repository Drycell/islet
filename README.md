![Build](https://github.sec.samsung.net/SYSSEC/arm-cca/actions/workflows/build.yml/badge.svg?branch=main)
![License](https://img.shields.io/badge/license-Samsung%20Inner%20Source-informational.svg)
![Test](https://art.sec.samsung.net/artifactory/rs8-dsca-da_generic/status/arm-cca/test.png)
![Coverage](https://art.sec.samsung.net/artifactory/rs8-dsca-da_generic/status/arm-cca/coverage.png)

# arm-cca
This repository contains code for confidential computing on the ARM CCA architecture.

## How to prepare build
```bash
./scripts/init.sh
```

## How to build
```bash
./scripts/build.sh
```

## How to run
```bash
./scripts/run.sh
```

## How to do unit-tests
```bash
./scripts/test.sh --unit-test
```

## How to measure line coverage of unit-tests
```bash
./scripts/test.sh --coverage
```

## How to connect T32
```bash
./scripts/run.sh --cadi-server
```

Then, execute the t32 application (e.g., ./t32marm-qt)
and run the script ./debug/t32.cmm via "File -> Run Script".

## List of Maintainers
- Beomheyn Kim (beomheyn.kim@samsung.com)
- Bokdeuk Jeong (bd.jeong@samsung.com)
- Sangwan Kwon (sangwan.kwon@samsung.com)
- Sungbae Yoo (sungbae.yoo@samsung.com)


## Governance
All decisions in this project are made by consensus, respecting the principles and rules of the community.  Please refer to the [Samsung Inner Source Governance](docs/Governance.md) in more detail.

