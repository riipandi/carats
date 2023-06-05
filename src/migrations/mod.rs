// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use sqlx_migrator::migration::Migration;
use sqlx_migrator::sqlx::Postgres;

pub(crate) mod m0001_initialize;
pub(crate) mod m0002_create_application_table;
pub(crate) mod m0003_create_users_table;
pub(crate) mod m0004_create_roles_table;
pub(crate) mod m0005_create_users_has_roles;

pub(crate) fn migrations() -> Vec<Box<dyn Migration<Postgres>>> {
	vec![
		Box::new(m0001_initialize::M0001Migration),
		Box::new(m0002_create_application_table::M0002Migration),
		Box::new(m0003_create_users_table::M0003Migration),
		Box::new(m0004_create_roles_table::M0004Migration),
		Box::new(m0005_create_users_has_roles::M0005Migration),
	]
}
