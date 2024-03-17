const employeeInput = document.getElementById("newEmployee");
const addEmployeeBtn = document.getElementById("addEmployeeBtn");
const employeeTable = document.getElementById("employeeTable");
const projectInput = document.getElementById("newProject");
const addProjectBtn = document.getElementById("addProjectBtn");
const projectTable = document.getElementById("projectTable");

const baseUrl = "http://localhost:3001/";

// SHOW EMPLOYEES
async function showEmployees() {
  const res = await fetch(baseUrl + "employees", {
    method: "GET",
  });
  const data = await res.json();
  employeeTable.innerHTML = "";
  employeeTable.innerHTML = `<thead>
          <th>ID</th>
          <th>Name</th>
          </thead>
          <tbody>
            ${data.map((employee) => `<tr onclick="employeeProperties(${employee.id})"><td>${employee.id}</td><td>${employee.name}</td></tr>`).join("")}
          </tbody>
          `;
}
employeeTable.onload = showEmployees();

// ADD EMPLOYEES
addEmployeeBtn.addEventListener("click", addEmployee);
async function addEmployee(e) {
  e.preventDefault();
  const name = employeeInput.value;
  await fetch(baseUrl + "employees", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      name,
    }),
  });
  employeeInput.value = "";
  showEmployees();
  resetInfo();
}

// SHOW PROJECTS
async function showProjects() {
  const projects = await fetch(baseUrl + "projects", {
    method: "GET",
  });
  const data = await projects.json();
  const names = [];
  for (let i = 0; i < data.length; i++) {
    const task = await showTask(data[i].id);
    names.push(task);
  }
  projectTable.innerHTML = "";
  projectTable.innerHTML = `<thead>
          <th>ID</th>
          <th>Name</th>
          <th>Assigned to</th>
          </thead>
          <tbody id="projectTableBody">
          </tbody>
          `;

  for (let i = 0; i < data.length; i++) {
    const project = data[i];
    const projectTableBody = document.getElementById("projectTableBody");

    projectTableBody.innerHTML += `<tr id=projectTableRow onclick="projectProperties(${project.id})"><td>${project.id}</td><td>${project.name}</td><td id="projectTableTasks">${names[i]}</td></tr>`;
  }
}
projectTable.onload = showProjects();

// SHOW TASKS
async function showTask(id) {
  const tasks = await fetch(baseUrl + "tasks/project/" + id, {
    method: "GET",
  });

  const taskJson = await tasks.json();
  const names = [];
  for (let i = 0; i < taskJson.length; i++) {
    const employeeData = await fetch(
      baseUrl + "employees/" + taskJson[i].employee_id,
      {
        method: "GET",
      },
    );

    const employeeJson = await employeeData.json();
    names.push(employeeJson.name);
  }

  return names;
}

// ADD PROJECTS
addProjectBtn.addEventListener("click", addProject);
async function addProject(e) {
  e.preventDefault();
  const name = projectInput.value;
  await fetch(baseUrl + "projects", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      name,
    }),
  });
  projectInput.value = "";
  showProjects();
  resetInfo();
}

// EMPLOYEE INFO
async function employeeProperties(id) {
  const res = await fetch(baseUrl + "employees/" + id, {
    method: "GET",
  });
  const data = await res.json();

  // get not assigned tasks information
  const notAssignedTask = await fetch(baseUrl + `tasks/employee/${id}/not`, {
    method: "GET",
  });

  const notAssignedTaskJson = await notAssignedTask.json();
  let notAssignedTaskInfo = [];
  for (let i = 0; i < notAssignedTaskJson.length; i++) {
    const notAssignedTaskData = await fetch(
      baseUrl + "projects/" + notAssignedTaskJson[i].project_id,
      {
        method: "GET",
      },
    );
    let notAssignedJson = await notAssignedTaskData.json();
    notAssignedTaskInfo.push(notAssignedJson);
  }

  const projects = await fetch(baseUrl + "projects", {
    method: "GET",
  });
  const projectsJson = await projects.json();

  let projectInfo = [];
  for (let i = 0; i < projectsJson.length; i++) {
    projectInfo.push(projectsJson[i]);
  }

  // Get assigned tasks information
  const assignedTask = await fetch(baseUrl + "tasks/employee/" + id, {
    method: "GET",
  });
  const assignedTaskJson = await assignedTask.json();

  let taskInfo = [];
  for (let i = 0; i < assignedTaskJson.length; i++) {
    const taskData = await fetch(
      baseUrl + "projects/" + assignedTaskJson[i].project_id,
      {
        method: "GET",
      },
    );
    taskJson = await taskData.json();
    taskInfo.push(taskJson);
  }

  employeeInfo.innerHTML = "";
  employeeInfo.innerHTML = `
          <h2>Employee Info</h2>
          <form>
            <label class="form-label" for="employeeName"><b>Name</b></label>
            <input
              type="text"
              class="form-control"
              id="employeeName"
              placeholder="Employee Name"
              value = "${data.name}"
            />

            <label class="form-label mt-3" for="assignTask"><b>Assign Project (ID)</b></label>
            <select id="assignEmployeeTask" class="form-select" name="Assign Task" >
              <option selected>None</option>
              ${projectInfo.map((task) => `<option>${task.id}</option>`).join("")}
            </select>

            <label class="form-label mt-3" for="removeTask"><b>Remove Project (ID)</b></label>
            <select id="removeEmployeeTask" class="form-select" name="Remove Task" >
              <option selected>None</option>
              ${taskInfo.map((task) => `<option>${task.id}</option>`).join("")}
            </select>

            <button type="button" id="updateEmployeeBtn" class="btn btn-success mt-3" onclick="updateEmployee(${data.id})">
              Update
            </button>

            <button type="button" id="deleteEmployeeBtn" class="btn btn-danger mt-3" onclick="deleteEmployee(${data.id})">
              Delete
            </button>
          </form>
        `;
}

// UPDATE EMPLOYEE
async function updateEmployee(id) {
  const name = document.getElementById("employeeName").value;
  await fetch(baseUrl + "employees/" + id, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      name,
    }),
  });

  const assignTask = document.getElementById("assignEmployeeTask").value;
  const taskValue = parseInt(assignTask);
  if (assignTask !== "None") {
    await fetch(baseUrl + "tasks", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        project_id: taskValue,
        employee_id: id,
      }),
    });
  }

  const removeTask = document.getElementById("removeEmployeeTask").value;
  if (removeTask !== "None") {
    await fetch(baseUrl + `tasks/employee/${id}/project/` + removeTask, {
      method: "DELETE",
    });
  }

  showEmployees();
  showProjects();
  employeeProperties(id);
}

// PROJECT INFO
async function projectProperties(id) {
  const res = await fetch(baseUrl + "projects/" + id, {
    method: "GET",
  });
  const data = await res.json();

  projectInfo.innerHTML = "";
  projectInfo.innerHTML = `
          <h2>Project Info</h2>
          <form>
            <label class="form-label" for="projectName"><b>Name</b></label>
            <input
              type="text"
              class="form-control"
              id="projectName"
              placeholder="Employee Name"
              value = "${data.name}"
            />
            <button type="button" id="updateProjectBtn" class="btn btn-success mt-3" onclick="updateProject(${data.id})">
              Update
            </button>
            <button type="button" id="deleteProjectBtn" class="btn btn-danger mt-3" onclick="deleteProject(${data.id})">
              Delete
            </button>
          </form>
        `;
}

// UPDATE PROJECT
async function updateProject(id) {
  const name = document.getElementById("projectName").value;
  await fetch(baseUrl + "projects/" + id, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      name,
    }),
  });
  showProjects();
}

// DELETE EMPLOYEE
async function deleteEmployee(id) {
  await fetch(baseUrl + "employees/" + id, {
    method: "DELETE",
  });
  showEmployees();
  showProjects();
  employeeInfo.innerHTML = "";
}

// DELETE PROJECT
async function deleteProject(id) {
  await fetch(baseUrl + "projects/" + id, {
    method: "DELETE",
  });
  showProjects();
  projectInfo.innerHTML = "";
}

// RESET Info
function resetInfo() {
  employeeInfo.innerHTML = "";
  projectInfo.innerHTML = "";
}
