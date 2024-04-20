-- Create the database
CREATE DATABASE IF NOT EXISTS mydatabase;

USE mydatabase;

-- Create the user table
CREATE TABLE IF NOT EXISTS usertable (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

-- Create the Todo table
CREATE TABLE IF NOT EXISTS todotable (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    is_done BOOLEAN NOT NULL DEFAULT false,
    user_id INT,
    FOREIGN KEY (user_id) REFERENCES usertable(id) ON DELETE CASCADE
);

-- Insert initial user data
INSERT INTO
    usertable (name, password)
VALUES
    ('user1', 'password1'),
    ('user2', 'password2'),
    ('user3', 'password3');

-- Insert initial Todo data
INSERT INTO
    todotable (name, user_id)
VALUES
    ('Task 1 for user1', 1),
    ('Task 2 for user1', 1),
    ('Task 1 for user2', 2),
    ('Task 1 for user3', 3);