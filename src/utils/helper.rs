// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

pub fn get_envar(key: &str, default: Option<&str>) -> String {
	match std::env::var(key) {
		Ok(val) => val,
		Err(_) => match default {
			Some(val) => val.to_string(),
			None => "".to_string(),
		},
	}
}
