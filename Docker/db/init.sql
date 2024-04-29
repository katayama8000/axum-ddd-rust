CREATE DATABASE IF NOT EXISTS mydatabase;

GRANT ALL PRIVILEGES ON mydatabase.* TO 'myuser' @'%' IDENTIFIED BY 'mypassword';

USE mydatabase;

-- Create Circles table
CREATE TABLE IF NOT EXISTS circles (
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    capacity INT NOT NULL,
    owner_id BINARY(16) NOT NULL
);

-- Create Members table
CREATE TABLE IF NOT EXISTS members (
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    grade INT NOT NULL,
    circle_id BINARY(16),
    is_owner BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (circle_id) REFERENCES circles(id) ON DELETE CASCADE
);

-- Insert initial data into Circles table
INSERT INTO
    circles (id, name, capacity, owner_id)
VALUES
    (
        UNHEX(REPLACE(UUID(), '-', '')),
        'Circle A',
        5,
        UNHEX(REPLACE(UUID(), '-', ''))
    ),
    (
        UNHEX(REPLACE(UUID(), '-', '')),
        'Circle B',
        8,
        UNHEX(REPLACE(UUID(), '-', ''))
    ),
    (
        UNHEX(REPLACE(UUID(), '-', '')),
        'Circle C',
        10,
        UNHEX(REPLACE(UUID(), '-', ''))
    );

-- Insert initial data into Members table
INSERT INTO
    members (id, name, grade, circle_id, is_owner)
VALUES
    (
        UNHEX(REPLACE(UUID(), '-', '')),
        'Alice',
        3,
        (
            SELECT
                id
            FROM
                circles
            ORDER BY
                RAND()
            LIMIT
                1
        ), true
    ),
    (
        UNHEX(REPLACE(UUID(), '-', '')),
        'Bob',
        2,
        (
            SELECT
                id
            FROM
                circles
            ORDER BY
                RAND()
            LIMIT
                1
        ), false
    ), (
        UNHEX(REPLACE(UUID(), '-', '')),
        'Charlie',
        3,
        (
            SELECT
                id
            FROM
                circles
            ORDER BY
                RAND()
            LIMIT
                1
        ), false
    ), (
        UNHEX(REPLACE(UUID(), '-', '')),
        'David',
        4,
        (
            SELECT
                id
            FROM
                circles
            ORDER BY
                RAND()
            LIMIT
                1
        ), false
    ), (
        UNHEX(REPLACE(UUID(), '-', '')),
        'Eve',
        2,
        (
            SELECT
                id
            FROM
                circles
            ORDER BY
                RAND()
            LIMIT
                1
        ), false
    ), (
        UNHEX(REPLACE(UUID(), '-', '')),
        'Frank',
        4,
        (
            SELECT
                id
            FROM
                circles
            ORDER BY
                RAND()
            LIMIT
                1
        ), false
    );