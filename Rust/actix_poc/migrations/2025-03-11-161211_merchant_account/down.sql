-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS merchant_account;
DROP TYPE IF EXISTS payment_type CASCADE;
DROP TYPE IF EXISTS country_enum CASCADE;