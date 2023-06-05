// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use sqlx::PgConnection;
use sqlx_migrator::error::Error;
use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;
use sqlx_migrator::sqlx::Postgres;

const MIGRATION_NAME: &str = "m0004_create_roles_table";

pub(crate) struct M0004Operation;

#[async_trait::async_trait]
impl Operation<Postgres> for M0004Operation {
	async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let queries = concat!(
			r#"
            CREATE TABLE IF NOT EXISTS roles (
              id uuid DEFAULT uuid_generate_v1mc() NOT NULL,
              name varchar(256) NOT NULL,
              application_id uuid,
              permissions text[],
              created_at TIMESTAMPTZ DEFAULT timezone('UTC', now()) NOT NULL,
              updated_at TIMESTAMPTZ DEFAULT timezone('UTC', now()) NOT NULL
            );
          "#,
			r#"ALTER TABLE roles ADD CONSTRAINT roles_name_application_id PRIMARY KEY(name,application_id);"#,
			r#"
            DO $$ BEGIN
              ALTER TABLE roles
              ADD CONSTRAINT roles_application_id_applications_id_fk FOREIGN KEY (application_id)
              REFERENCES applications(id) ON DELETE no action ON UPDATE no action;
            EXCEPTION
              WHEN duplicate_object THEN null;
            END $$;
          "#,
			r#"CREATE UNIQUE INDEX IF NOT EXISTS roles_id_index ON roles (id);"#,
		);

		sqlx::query(queries).execute_many(connection).await;
		Ok(())
	}

	async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let queries = concat!(
			"DROP INDEX IF EXISTS roles_id_index;",
			"DROP TABLE IF EXISTS roles;",
		);
		sqlx::query(queries).execute(connection).await?;
		Ok(())
	}
}

pub(crate) struct M0004Migration;

#[rustfmt::skip]
#[async_trait::async_trait]
impl Migration<Postgres> for M0004Migration {
    #[inline(always)]
    fn app(&self) -> &str { crate::PKG_NAME }

    #[inline(always)]
    fn name(&self) -> &str { MIGRATION_NAME }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![Box::new(crate::migrations::m0002_create_application_table::M0002Migration)]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(M0004Operation)]
    }
}
