// Copyright 2023 Aris Ripandi, and Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FilterOptions {
	pub page: Option<usize>,
	pub limit: Option<usize>,
}
