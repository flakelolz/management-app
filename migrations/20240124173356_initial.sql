CREATE TABLE employee (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT
);

CREATE TABLE project (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT,
  assigned_to INTEGER REFERENCES employee(id)
);

INSERT INTO employee (name) VALUES ('Alice');
INSERT INTO employee (name) VALUES ('Bob');
INSERT INTO employee (name) VALUES ('Cathy');

INSERT INTO project (name, assigned_to) VALUES ('Project A', 1);
INSERT INTO project (name, assigned_to) VALUES ('Project B', 2);
INSERT INTO project (name, assigned_to) VALUES ('Project C', 3);
