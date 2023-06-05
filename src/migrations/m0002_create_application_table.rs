// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use sqlx::PgConnection;
use sqlx_migrator::error::Error;
use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;
use sqlx_migrator::sqlx::Postgres;

const MIGRATION_NAME: &str = "m0002_create_application_table";

pub(crate) struct M0002Operation;

#[async_trait::async_trait]
impl Operation<Postgres> for M0002Operation {
	async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let query = "
          CREATE TABLE IF NOT EXISTS applications (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v1mc() NOT NULL,
            name VARCHAR(256) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT timezone('UTC', now()) NOT NULL,
            updated_at TIMESTAMPTZ DEFAULT timezone('UTC', now()) NOT NULL
          )
        ";

		sqlx::query(query).execute(connection).await?;
		Ok(())
	}

	async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let query = "DROP TABLE IF EXISTS applications";
		sqlx::query(query).execute(connection).await?;
		Ok(())
	}
}

pub(crate) struct M0002Migration;

#[rustfmt::skip]
#[async_trait::async_trait]
impl Migration<Postgres> for M0002Migration {
    #[inline(always)]
    fn app(&self) -> &str { crate::PKG_NAME }

    #[inline(always)]
    fn name(&self) -> &str { MIGRATION_NAME }

    #[inline(always)]
    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> { vec![] }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(M0002Operation)]
    }
}
