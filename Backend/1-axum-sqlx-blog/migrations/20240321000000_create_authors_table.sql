-- Create Authors Table
CREATE TABLE IF NOT EXISTS authors (
    id           SERIAL         PRIMARY KEY,
    name         VARCHAR(255)   NOT NULL,
    created_at   TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ    NOT NULL DEFAULT NOW()
); 

-- Insert fake data
INSERT INTO authors (name) VALUES 
    ('John Doe'),
    ('Jane Smith'),
    ('Alice Johnson');