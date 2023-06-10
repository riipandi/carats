CREATE TABLE IF NOT EXISTS customers (
	id uuid PRIMARY KEY DEFAULT uuid_generate_v1mc() NOT NULL,
	billable_id varchar(255) NOT NULL UNIQUE,
	billable_type varchar(255) NOT NULL UNIQUE,
	lemon_squeezy_id varchar(255) NOT NULL,
	trial_ends_at timestamptz,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL
);
