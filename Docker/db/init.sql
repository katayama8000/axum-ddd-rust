-- データベースの作成
CREATE DATABASE IF NOT EXISTS mydatabase;

-- myuserに全ての権限を付与
GRANT ALL PRIVILEGES ON mydatabase.* TO 'myuser' @'%' IDENTIFIED BY 'mypassword';

-- mydatabaseを使用する
USE mydatabase;

-- Circlesテーブルの作成
CREATE TABLE IF NOT EXISTS circles (
    id CHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    capacity INT NOT NULL,
    owner_id CHAR(36) NOT NULL
);

-- Membersテーブルの作成
CREATE TABLE IF NOT EXISTS members (
    id CHAR(36) NOT NULL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    grade INT NOT NULL,
    circle_id CHAR(36),
    age INT NOT NULL DEFAULT 20,
    major VARCHAR(255) NOT NULL DEFAULT 'other',
    FOREIGN KEY (circle_id) REFERENCES circles(id) ON DELETE CASCADE
);

-- Circlesテーブルへの初期データ挿入
INSERT INTO
    circles (id, name, capacity, owner_id)
VALUES
    (UUID(), 'Circle A', 5, UUID()),
    (UUID(), 'Circle B', 8, UUID()),
    (UUID(), 'Circle C', 10, UUID());

-- Membersテーブルへの初期データ挿入
INSERT INTO
    members (id, name, grade, circle_id, age, major)
VALUES
    (
        UUID(),
        'Alice',
        3,
        (
            SELECT
                id
            FROM
                circles
            WHERE
                name = 'Circle A'
        ),
        21,
        'math'
    ),
    (
        UUID(),
        'Bob',
        2,
        (
            SELECT
                id
            FROM
                circles
            WHERE
                name = 'Circle B'
        ),
        22,
        'math'
    ),
    (
        UUID(),
        'Charlie',
        3,
        (
            SELECT
                id
            FROM
                circles
            WHERE
                name = 'Circle C'
        ),
        23,
        'math'
    ),
    (
        UUID(),
        'David',
        4,
        (
            SELECT
                id
            FROM
                circles
            WHERE
                name = 'Circle A'
        ),
        21,
        'math'
    ),
    (
        UUID(),
        'Eve',
        2,
        (
            SELECT
                id
            FROM
                circles
            WHERE
                name = 'Circle B'
        ),
        19,
        'math'
    ),
    (
        UUID(),
        'Frank',
        4,
        (
            SELECT
                id
            FROM
                circles
            WHERE
                name = 'Circle C'
        ),
        20,
        'math'
    );