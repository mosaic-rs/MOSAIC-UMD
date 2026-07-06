# Universal Measurement Data Structure (UMD)
## A Child Project of [MOSAIC](https://github.com/mosaic-rs/MOSAIC)

[![Rust](https://img.shields.io/badge/languages-Rust-Blue
)](https://www.rust-lang.org/)

MOSAIC-UMD is the data manager for the MOSAIC platform, converting varying articulatory data into a reproducible, structured, compressed, fast, and immutable data structure. It uses a network of drivers (which can be downloaded from the internt or created yourself in MOSAIC) to transform common or proprietary data into a UMD file. The methods gauranteed to be supported by MOSAIC-UMD are:

| Capture Method | Supprt Level | Notes |
| :--- | :---: | ---: |
| OpenFace v2.2.0 | Full Support on 1.0.0-alpha release |  |
| Cartsens EMA | Full Support on 1.0.0-alpha release | AG100 - 501 |
| DeepLabCut | Full Support on 1.0.0-alpha release | For rtMRI and Ultrasound based on customizable drivers|
| Custom Drivers | Full Support on 1.0.0-alpha release | Automatically create drivers to translate proprietary data types |

The UMD exports to the Apache Parquet format, meaning your data is compressed and immutable. The data is also structured into a clean schema with the following columns:

*Z axis coordinates are always optional to accomodate 2D data*

__Administrative__
| Column Name | Data Type | Optional/Required |
| :--- | :---: | ---: |
| frame | u32 | Required |
| timestamp | f32 | Required |
| confidence | f32 | Required |
| pose | bool | Required |
| coordinate_number | u32 | Required |
| types | String | Required |

__Pose__
| Column Name | Data Type | Optional/Required |
| :--- | :---: | ---: |
| pose_x | f64 | Optional' Required |
| pose_y | f64 | Optional' Required |
| pose_z | f64 | Optional |
| pose_x_uncertainty | f64 | Optional' Required |
| pose_y_uncertainty | f64 | Optional' Required |
| pose_z_uncertainty | f64 | Optional |

*Uncertainty is calculated by default*

*' Optional because you can skip pose calculation*

__Raw Coordinates__
| Column Name | Data Type | Optional/Required |
| :--- | :---: | ---: |
| x_raw | f64 | Required |
| y_raw | f64 | Required |
| y_raw | f64 | Optional |
| x_raw_uncertainty | f64 | Required |
| y_raw_uncertainty | f64 | Required |
| z_raw_uncertainty | f64 | Optional |
*Uncertainty is calculated by default*

__Centered Coordinates__
| Column Name | Data Type | Optional/Required |
| :--- | :---: | ---: |
| x_centered | f64 | Required |
| y_centered | f64 | Required |
| y_centered | f64 | Optional |
| x_centered_uncertainty | f64 | Required |
| y_centered_uncertainty | f64 | Required |
| z_centered_uncertainty | f64 | Optional |
*Uncertainty is calculated by default*

__Pose Coordinates__
| Column Name | Data Type | Optional/Required |
| :--- | :---: | ---: |
| x_rotates | f64 | Required |
| y_rotates | f64 | Required |
| y_rotates | f64 | Optional |
| x_rotates_uncertainty | f64 | Required |
| y_rotates_uncertainty | f64 | Required |
| z_rotates_uncertainty | f64 | Optional |
*Uncertainty is calculated by default*

__Anchor Coordinates__
| Column Name | Data Type | Optional/Required |
| :--- | :---: | ---: |
| x_anchor | f64 | Required |
| y_anchor | f64 | Required |
| y_anchor | f64 | Required |
| x_anchor_uncertainty | f64 | Required |
| y_anchor_uncertainty | f64 | Required |
| z_anchor_uncertainty | f64 | Optional |
*Uncertainty is calculated by default*

