// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use dialoguer::{theme::ColorfulTheme, Confirm};
use sqlx::Pool;
use sqlx_migrator::migrator::{Info, Migrate, Migrator};
use sqlx_migrator::sqlx::Postgres;

use crate::{migrations, utils, PKG_NAME, PKG_VERSION};

pub fn version(short: bool) {
	if short {
		println!("{}", PKG_VERSION);
	} else {
		let build_timestamp = build_time::build_time_utc!("%Y-%m-%d %H:%M:%S UTC");
		println!("{} {} ({})", PKG_NAME, PKG_VERSION, build_timestamp);
	}
}

pub fn generate_secret() -> String {
	std::iter::repeat_with(fastrand::alphanumeric)
		.take(40)
		.collect()
}

pub async fn migrate(force: bool) {
	let uri = utils::get_envar("DATABASE_URL", None);
	let pool = Pool::<Postgres>::connect(&uri).await.unwrap();

	if force {
		println!("🍀 Running database migration automatically");
		migrate_up(pool).await
	} else if Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt("Do you want to continue?")
		.wait_for_newline(true)
		.interact()
		.unwrap()
	{
		migrate_up(pool).await
	} else {
		println!("Sayonara 👋");
	}
}

async fn migrate_up(pool: Pool<Postgres>) {
	let mut migrator = Migrator::new(&pool);
	migrator.add_migrations(migrations::migrations());
	migrator.apply_all().await.unwrap();
	println!("🍀 Database migration succeed")
}

#[allow(dead_code)]
async fn migrate_down(pool: Pool<Postgres>) {
	let mut migrator = Migrator::new(&pool);
	migrator.add_migrations(migrations::migrations());
	migrator.revert_all().await.unwrap();
	println!("🍀 Database migration reverted")
}
