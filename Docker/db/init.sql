-- データベースの作成
CREATE DATABASE IF NOT EXISTS mydatabase;

-- myuserに全ての権限を付与
GRANT ALL PRIVILEGES ON mydatabase.* TO 'myuser' @'%' IDENTIFIED BY 'mypassword';

-- mydatabaseを使用する
USE mydatabase;

-- Circlesテーブルの作成
CREATE TABLE IF NOT EXISTS circles (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    capacity INT NOT NULL,
    owner_id INT NOT NULL
);

-- Membersテーブルの作成
CREATE TABLE IF NOT EXISTS members (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    grade INT NOT NULL,
    circle_id INT,
    is_owner BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (circle_id) REFERENCES circles(id) ON DELETE CASCADE
);

-- Circlesテーブルへの初期データ挿入
INSERT INTO
    circles (name, capacity, owner_id)
VALUES
    ('Circle A', 5, 1),
    ('Circle B', 8, 2),
    ('Circle C', 10, 3);

-- Membersテーブルへの初期データ挿入
INSERT INTO
    members (name, grade, circle_id, is_owner)
VALUES
    ('Alice', 3, 1, true),
    ('Bob', 2, 2, false),
    ('Charlie', 3, 3, false),
    ('David', 4, 1, false),
    ('Eve', 2, 2, false),
    ('Frank', 4, 3, false);

-- TODO: Mmmbersテーブルに age major フィールドを追加