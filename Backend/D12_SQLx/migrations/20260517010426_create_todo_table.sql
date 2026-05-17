-- Add migration script here

CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    text VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO
    todos (text)
VALUES ('Buy Groceries'),
    ('Finish the Project'),
    ('Read a Book'),
    ('Go to the Gym'),
    ('Call Mom'),
    ('Pay Electricity Bill'),
    ('Complete Homework'),
    ('Clean the Room'),
    ('Prepare Dinner'),
    ('Walk the Dog');