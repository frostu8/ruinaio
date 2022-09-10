-- Nodes table
CREATE TABLE node (
    id SERIAL PRIMARY KEY,
    slug VARCHAR(128) NOT NULL UNIQUE,
    body TEXT NOT NULL
);

-- Relationships
CREATE TABLE relation (
    parent_id INTEGER NOT NULL REFERENCES node(id) ON DELETE CASCADE,
    child_id INTEGER NOT NULL REFERENCES node(id) ON DELETE CASCADE,

    PRIMARY KEY (parent_id, child_id)
);

-- Images
CREATE TABLE images (
    -- Used to look up the file in the object storage.
    hash CHAR(64) NOT NULL,
    -- The actual filename.
    filename VARCHAR(256) NOT NULL,
    -- The parent node's id.
    node_id INTEGER NOT NULL REFERENCES node(id) ON DELETE CASCADE,

    PRIMARY KEY (filename, node_id)
);

