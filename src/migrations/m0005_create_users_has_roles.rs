// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use sqlx::PgConnection;
use sqlx_migrator::error::Error;
use sqlx_migrator::migration::Migration;
use sqlx_migrator::operation::Operation;
use sqlx_migrator::sqlx::Postgres;

const MIGRATION_NAME: &str = "m0005_create_users_has_roles";

pub(crate) struct M0005Operation;

#[async_trait::async_trait]
impl Operation<Postgres> for M0005Operation {
	async fn up(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let queries = concat!(
			r#"
              CREATE TABLE IF NOT EXISTS users_has_roles (
                application_id uuid NOT NULL,
                role_id uuid NOT NULL,
                user_id uuid NOT NULL
              );
        "#,
			r#"
              ALTER TABLE users_has_roles ADD CONSTRAINT users_has_roles_application_id_role_id_user_id
              PRIMARY KEY(application_id,role_id,user_id);
            "#,
			r#"
              DO $$ BEGIN
                ALTER TABLE users_has_roles
                ADD CONSTRAINT users_has_roles_application_id_applications_id_fk FOREIGN KEY (application_id)
                REFERENCES applications(id) ON DELETE no action ON UPDATE no action;
              EXCEPTION
                WHEN duplicate_object THEN null;
              END $$;
            "#,
			r#"
              DO $$ BEGIN
                ALTER TABLE users_has_roles
                ADD CONSTRAINT users_has_roles_role_id_roles_id_fk FOREIGN KEY (role_id)
                REFERENCES roles(id) ON DELETE no action ON UPDATE no action;
              EXCEPTION
                WHEN duplicate_object THEN null;
              END $$;
            "#,
			r#"
              DO $$ BEGIN
                ALTER TABLE users_has_roles ADD CONSTRAINT users_has_roles_user_id_users_id_fk FOREIGN KEY (user_id)
                REFERENCES users(id) ON DELETE no action ON UPDATE no action;
              EXCEPTION
                WHEN duplicate_object THEN null;
              END $$;
            "#,
		);

		sqlx::query(queries).execute_many(connection).await;
		Ok(())
	}

	async fn down(&self, connection: &mut PgConnection) -> Result<(), Error> {
		let query = "DROP TABLE IF EXISTS users_has_roles";
		sqlx::query(query).execute(connection).await?;
		Ok(())
	}
}

pub(crate) struct M0005Migration;

#[rustfmt::skip]
#[async_trait::async_trait]
impl Migration<Postgres> for M0005Migration {
    #[inline(always)]
    fn app(&self) -> &str { crate::PKG_NAME }

    #[inline(always)]
    fn name(&self) -> &str { MIGRATION_NAME }

    fn parents(&self) -> Vec<Box<dyn Migration<Postgres>>> {
        vec![
          Box::new(crate::migrations::m0002_create_application_table::M0002Migration),
          Box::new(crate::migrations::m0003_create_users_table::M0003Migration),
          Box::new(crate::migrations::m0004_create_roles_table::M0004Migration),
        ]
    }

    fn operations(&self) -> Vec<Box<dyn Operation<Postgres>>> {
        vec![Box::new(M0005Operation)]
    }
}
