CREATE TABLE Members (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    age INT NOT NULL,
    grade VARCHAR(10) NOT NULL,
    major VARCHAR(255) NOT NULL
);

CREATE TABLE Circles (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    capacity INT NOT NULL,
    owner_id VARCHAR(36) NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES Members(id)
);

CREATE TABLE CircleMembers (
    circle_id VARCHAR(36),
    member_id VARCHAR(36),
    PRIMARY KEY (circle_id, member_id),
    FOREIGN KEY (circle_id) REFERENCES Circles(id),
    FOREIGN KEY (member_id) REFERENCES Members(id)
);

INSERT INTO
    Members (id, name, age, grade, major)
VALUES
    ('1', 'Alice', 22, 'Third', 'Computer Science'),
    ('2', 'Bob', 21, 'Second', 'Engineering'),
    ('3', 'Charlie', 23, 'Fourth', 'Mathematics');

INSERT INTO
    Circles (id, name, capacity, owner_id)
VALUES
    ('101', 'Programming Club', 10, '1'),
    ('102', 'Robotics Club', 15, '2'),
    ('103', 'Math Club', 8, '3');

INSERT INTO
    CircleMembers (circle_id, member_id)
VALUES
    ('101', '1'),
    ('102', '1');