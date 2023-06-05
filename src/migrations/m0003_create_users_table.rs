// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use sqlx::PgConnection;
use sqlx_migrator::error::Error;
use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;
use sqlx_migrator::sqlx::Postgres;

const MIGRATION_NAME: &str = "m0003_create_users_table";

pub(crate) struct M0003Operation;

#[async_trait::async_trait]
impl Operation<Postgres> for M0003Operation {
	async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let queries = concat!(
			r#"
              CREATE TABLE IF NOT EXISTS users (
                id uuid DEFAULT uuid_generate_v1mc() NOT NULL,
                email varchar(256) NOT NULL,
                name varchar(256) NOT NULL,
                application_id uuid,
                encrypted_password varchar(256) NOT NULL,
                created_at TIMESTAMPTZ DEFAULT timezone('UTC', now()) NOT NULL,
                updated_at TIMESTAMPTZ DEFAULT timezone('UTC', now()) NOT NULL
              );
            "#,
			r#"ALTER TABLE users ADD CONSTRAINT users_email_application_id PRIMARY KEY(email,application_id);"#,
			r#"
              DO $$ BEGIN
                ALTER TABLE users
                ADD CONSTRAINT users_application_id_applications_id_fk FOREIGN KEY (application_id)
                REFERENCES applications(id) ON DELETE no action ON UPDATE no action;
              EXCEPTION
                WHEN duplicate_object THEN null;
              END $$;
            "#,
			r#"CREATE UNIQUE INDEX IF NOT EXISTS users_id_index ON users (id);"#,
		);

		sqlx::query(queries).execute_many(connection).await;
		Ok(())
	}

	async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let queries = concat!(
			"DROP INDEX IF EXISTS users_id_index;",
			"DROP TABLE IF EXISTS users;"
		);
		sqlx::query(queries).execute_many(connection).await;
		Ok(())
	}
}

pub(crate) struct M0003Migration;

#[rustfmt::skip]
#[async_trait::async_trait]
impl Migration<Postgres> for M0003Migration {
    #[inline(always)]
    fn app(&self) -> &str { crate::PKG_NAME }

    #[inline(always)]
    fn name(&self) -> &str { MIGRATION_NAME }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![Box::new(crate::migrations::m0002_create_application_table::M0002Migration)]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(M0003Operation)]
    }
}
