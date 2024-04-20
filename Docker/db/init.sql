-- サークルテーブル
CREATE TABLE Circles (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    capacity INT NOT NULL,
    owner_id VARCHAR(36) NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES Members(id)
);

-- メンバーテーブル
CREATE TABLE Members (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    age INT NOT NULL,
    grade VARCHAR(10) NOT NULL,
    major VARCHAR(255) NOT NULL
);

-- サークルメンバー関連テーブル
CREATE TABLE CircleMembers (
    circle_id VARCHAR(36),
    member_id VARCHAR(36),
    PRIMARY KEY (circle_id, member_id),
    FOREIGN KEY (circle_id) REFERENCES Circles(id),
    FOREIGN KEY (member_id) REFERENCES Members(id)
);

-- Membersテーブルに初期データを挿入
INSERT INTO Members (id, name, age, grade, major) VALUES
('1', 'Alice', 22, 'Third', 'Computer Science'),
('2', 'Bob', 21, 'Second', 'Engineering'),
('3', 'Charlie', 23, 'Fourth', 'Mathematics');

-- Circlesテーブルに初期データを挿入
INSERT INTO Circles (id, name, capacity, owner_id) VALUES
('101', 'Programming Club', 10, '1'),
('102', 'Robotics Club', 15, '2'),
('103', 'Math Club', 8, '3');

-- CircleMembersテーブルに初期データを挿入 (例: AliceがProgramming ClubとRobotics Clubに参加)
INSERT INTO CircleMembers (circle_id, member_id) VALUES
('101', '1'), -- Alice (Programming Club)
('102', '1'); -- Alice (Robotics Club)

