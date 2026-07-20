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
    ('Getting Started with Rust', 'Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.', 1),
    ('Web Development with Axum', 'Axum is a modern web framework for Rust built on top of Tokio and Tower.', 1),
    ('Database Migrations with SQLx', 'SQLx provides excellent migration support for managing database schema changes over time.', 2),
    ('Building REST APIs in Rust', 'RESTful APIs are essential for modern web applications. Rust makes them fast and reliable.', 3),
    ('Error Handling in Rust', 'Rust provides powerful error handling mechanisms through the Result and Option types.', 2);
