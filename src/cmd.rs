// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use dialoguer::{theme::ColorfulTheme, Confirm};
use sqlx::{migrate::Migrator, Pool, Postgres};

use crate::{utils, PKG_NAME, PKG_VERSION};

#[derive(rust_embed::RustEmbed)]
#[folder = "migrations/"]
#[include = "*.sql"]
struct Migrations;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

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

	let success_message = "🍀 Database migration succes";
	let failed_message = "🍀 Could not execute database migration";

	if force {
		println!("🍀 Running database migration automatically");
		match migrate_up(pool).await {
			Ok(_) => println!("{}", success_message),
			Err(e) => println!("{}: {:?}", failed_message, e),
		}
	} else if Confirm::with_theme(&ColorfulTheme::default())
		.with_prompt("Do you want to continue?")
		.wait_for_newline(true)
		.interact()
		.unwrap()
	{
		match migrate_up(pool).await {
			Ok(_) => println!("{}", success_message),
			Err(e) => println!("{}: {:?}", failed_message, e),
		}
	} else {
		println!("Sayonara 👋");
	}
}

async fn migrate_up(pool: Pool<Postgres>) -> Result<(), sqlx::Error> {
	for mf in MIGRATOR.iter() {
		println!("🚀 Running migration {:} - {:}", mf.version, mf.description);
		MIGRATOR.run(&pool).await.unwrap();
	}
	Ok(())
}

// #[allow(dead_code)]
// async fn migrate_down(pool: Pool<Postgres>) {
// 	let mut migrator = Migrator::new(&pool);
// 	migrator.add_migrations(migrations::migrations());
// 	migrator.revert_all().await.unwrap();
// 	println!("🍀 Database migration reverted")
// }
