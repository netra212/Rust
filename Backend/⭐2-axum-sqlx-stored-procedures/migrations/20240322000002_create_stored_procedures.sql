-- =========================================================
-- Stored Procedure 1: get_author_stats
--
-- Returns statistics for a given author:
--   - blog_count  : total number of blogs written
--   - avg_length  : average character length of blog content
--   - latest_blog : title of the most recently created blog
--
-- Usage: SELECT * FROM get_author_stats(1);
-- =========================================================
CREATE OR REPLACE FUNCTION get_author_stats(p_author_id INT)
RETURNS TABLE (
    author_id   INT,
    blog_count  BIGINT,
    avg_length  FLOAT8,
    latest_blog TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    SELECT
        a.id::INT,
        COUNT(b.id)::BIGINT,
        AVG(LENGTH(b.content))::FLOAT8,
        MAX(b.title)::TEXT
    FROM authors a
    LEFT JOIN blogs b ON b.author_id = a.id
    WHERE a.id = p_author_id
    GROUP BY a.id;
END;
$$;


-- =========================================================
-- Stored Procedure 2: search_blogs
--
-- Case-insensitive full-text search across blog titles
-- and content. Returns matching blogs ordered by date.
--
-- RETURNS SETOF blogs — references the actual table type so
-- SQLx can infer column nullability from the table schema.
-- This avoids needing `col as "col!"` overrides in Rust.
--
-- Usage: SELECT * FROM search_blogs('rust');
-- =========================================================
CREATE OR REPLACE FUNCTION search_blogs(p_query TEXT)
RETURNS SETOF blogs
LANGUAGE sql STABLE
AS $$
    SELECT *
    FROM blogs
    WHERE title   ILIKE '%' || p_query || '%'
       OR content ILIKE '%' || p_query || '%'
    ORDER BY created_at DESC;
$$;
