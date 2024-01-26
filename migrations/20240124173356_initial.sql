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

CREATE TABLE tasks (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  project_id INTEGER REFERENCES project(id),
  employee_id INTEGER REFERENCES employee(id)
);

INSERT INTO tasks (project_id, employee_id) VALUES (1, 1);
INSERT INTO tasks (project_id, employee_id) VALUES (1, 2);
INSERT INTO tasks (project_id, employee_id) VALUES (2, 1);
INSERT INTO tasks (project_id, employee_id) VALUES (2, 3);
INSERT INTO tasks (project_id, employee_id) VALUES (3, 2);
INSERT INTO tasks (project_id, employee_id) VALUES (3, 3);
