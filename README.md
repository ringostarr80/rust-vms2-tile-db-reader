# Vms2TileDbReader

![license](https://img.shields.io/github/license/ringostarr80/rust-vms2-tile-db-reader)
![github_workflow_status](https://img.shields.io/github/actions/workflow/status/ringostarr80/rust-vms2-tile-db-reader/rust.yml)
[![codecov](https://codecov.io/gh/ringostarr80/rust-vms2-tile-db-reader/graph/badge.svg?token=vpMSVSlT84)](https://codecov.io/gh/ringostarr80/rust-vms2-tile-db-reader)
![github_tag](https://img.shields.io/github/v/tag/ringostarr80/rust-vms2-tile-db-reader)
[![Crates.io](https://img.shields.io/crates/v/vms2-tile-db-reader.svg)](https://crates.io/crates/vms2-tile-db-reader)

This is the rust version of the repository from https://github.com/locr-company/php-vms2-tile-db-reader

## Installation

```bash
cd /path/to/your/rust/project
cargo add vms2-tile-db-reader
```

## Basic Usage

```rust
use std::path::Path;

use vms2_tile_db_reader::{
    data_type::DataType,
    sources::Source,
    sources::SQLite
};

let tile_db = SQLite::new(Path::new("germany.sqlite")).unwrap();
let tile_data = tile_db.get_raw_data(
    34686,
    21566,
    16,
    String::from("building"),
    Some(String::from("*")),
    Some(DataType::Polygons)
).unwrap();

// do something with tile_data.
```
