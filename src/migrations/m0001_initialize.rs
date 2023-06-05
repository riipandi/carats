// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use sqlx::PgConnection;
use sqlx_migrator::error::Error;
use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;
use sqlx_migrator::sqlx::Postgres;

const MIGRATION_NAME: &str = "m0001_initialize";

pub(crate) struct M0001Operation;

#[async_trait::async_trait]
impl Operation<Postgres> for M0001Operation {
	async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let query = r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp""#;
		sqlx::query(query).execute(connection).await?;
		Ok(())
	}

	async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let query = r#"DROP EXTENSION IF EXISTS "uuid-ossp""#;
		sqlx::query(query).execute(connection).await?;
		Ok(())
	}
}

pub(crate) struct M0001Migration;

#[rustfmt::skip]
#[async_trait::async_trait]
impl Migration<Postgres> for M0001Migration {
    #[inline(always)]
    fn app(&self) -> &str { crate::PKG_NAME }

    #[inline(always)]
    fn name(&self) -> &str { MIGRATION_NAME }

    #[inline(always)]
    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> { vec![] }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(M0001Operation)]
    }
}
