-- Create Blogs Table
CREATE TABLE IF NOT EXISTS blogs (
    id           SERIAL         PRIMARY KEY,
    title        VARCHAR(255)   NOT NULL,
    content      TEXT           NOT NULL,
    author_id    INTEGER        NOT NULL REFERENCES authors(id) ON DELETE CASCADE,
    created_at   TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ    NOT NULL DEFAULT NOW()
); 

-- Insert fake data
INSERT INTO blogs (title, content, author_id) VALUES 
    ('Getting Started with Rust', 'Rust is a systems programming language...', 1),
    ('Web Development with Axum', 'Axum is a modern web framework for Rust...', 1),
    ('Database Migrations', 'SQLx provides excellent migration support...', 2),
    ('Building APIs', 'RESTful APIs are essential for modern web applications...', 3),
    ('Error Handling in Rust', 'Rust provides powerful error handling mechanisms...', 2);