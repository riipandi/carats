CREATE TABLE IF NOT EXISTS subscriptions (
	id uuid PRIMARY KEY DEFAULT uuid_generate_v1mc() NOT NULL,
	billable_id varchar(255) NOT NULL,
	billable_type varchar(255) NOT NULL,
  "type" varchar(255) NOT NULL,
  lemon_squeezy_id varchar(255) NOT NULL UNIQUE,
  status varchar(255) NOT NULL,
  product_id varchar(255) NOT NULL,
  variant_id varchar(255) NOT NULL,
  card_brand varchar(255),
  card_last_four varchar(255),
  pause_mode varchar(255),
  pause_resumes_at timestamptz,
  trial_ends_at timestamptz,
  renews_at timestamptz,
  ends_at timestamptz,
  created_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL,
  updated_at timestamptz DEFAULT timezone('utc'::text, now()) NOT NULL
);

CREATE INDEX IF NOT EXISTS subscriptions_billable_id_idx on subscriptions (billable_id);
CREATE INDEX IF NOT EXISTS subscriptions_billable_type_idx on subscriptions (billable_type);
