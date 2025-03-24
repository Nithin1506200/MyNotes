-- Your SQL goes here


-- Main table
CREATE TABLE merchant_account (
    id VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),  -- Unique identifier
    name VARCHAR(255) NOT NULL,                     -- Name with max 255 characters
    created_at TIMESTAMP NOT NULL DEFAULT now(), -- Creation time
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(), -- Auto-update time
    email VARCHAR UNIQUE NOT NULL CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'), -- Email validation
    address JSONB NOT NULL,                         -- Address stored as JSON
    allowed_payments JSONB NOT NULL,                 -- Array of payments stored as JSON
    active BOOLEAN NOT null
);

-- Index for faster searches on email
CREATE INDEX idx_users_email ON merchant_account(email);