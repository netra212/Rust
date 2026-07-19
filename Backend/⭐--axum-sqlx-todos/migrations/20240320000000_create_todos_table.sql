-- Create Todos Table
CREATE TABLE IF NOT EXISTS todos (
    id           SERIAL         PRIMARY KEY,
    text         VARCHAR(255)   NOT NULL,
    created_at   TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ    NOT NULL DEFAULT NOW()
); 

-- Insert fake data
INSERT INTO todos (text) VALUES 
    ('Learn Rust programming language'),
    ('Build a web API with Axum'),
    ('Master SQLx and PostgreSQL');
