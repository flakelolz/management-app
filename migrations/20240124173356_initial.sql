CREATE TABLE employee (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT
);

CREATE TABLE project (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT
);

INSERT INTO employee (name) VALUES ('Alice');
INSERT INTO employee (name) VALUES ('Bob');
INSERT INTO employee (name) VALUES ('Cathy');

INSERT INTO project (name) VALUES ('Project A');
INSERT INTO project (name) VALUES ('Project B');
INSERT INTO project (name) VALUES ('Project C');
