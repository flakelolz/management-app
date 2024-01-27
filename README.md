# How to open

Since this web application is just a backend made with Rust which serves an HTML, the whole application is just a binary executable.

Go to the Releases section of this repository to download the executable.
https://github.com/flakelolz/management-app/releases/tag/v0.0.2

Otherwise, you can build it from source if you have Rust and cargo (Rust's toolchain) installed. For that you'll need to:
- Go to Rust's main page - https://www.rust-lang.org/learn/get-started
- Download and install `Rustup` to get the Rust toolchain
- Download the source code of this repository
- In your terminal, navigate to the app folder, (Where the `cargo.toml` is)
- Run the `cargo run` command in your CLI

# How to use the app

This is an example for management app that deals with employees and projects for a company. The main page is located on `http://localhost:3001/`. 

On the main page you add employees or projects, click an employee or project on their respective lists and update information or delete them.

For the APIs there's the following endpoints:
- `http://localhost:3001/employees`
- `http://localhost:3001/projects`
- `http://localhost:3001/employees/:employee_id``
- `http://localhost:3001/projects/:project_id`
- `http://localhost:3001/tasks`
- `http://localhost:3001/tasks/:task_id`
- `http://localhost:3001/tasks/employee/:employee_id`
- `http://localhost:3001/tasks/project/:project_id`
- `http://localhost:3001/tasks/employee/employee_id/not`

With this endpoints you can perform CRUD operations from a tool that allows you to send request like https://hoppscotch.io/

# API

The following API requests are valid:
- `GET` to `/employees`, `/projects` or `/tasks` will return a list of all the respective items in the database.
- `GET` to `/employees/:id`, `/projects/:id` or `/tasks:id` will return the item that matches the `id` passed through the URL.
- `POST` to `/employees`, `/projects` or `/tasks` will create a new item when paired with a `JSON` payload.
	- For employees the JSON will need `name: string`
	- For projects the JSON will need `name: string`
	- For tasks the JSON will need `project_id: int` and `employee_id: int`
- `PUT` to `/employees/:id`, `/projects/:id` or `/tasks:id` will update the information of the item that matches the `id` given. Send a JSON payload with the new information.
	- For employees the JSON will need `name: string`
	- For projects the JSON will need `name: string`
	- For tasks the JSON will need `project_id: int` and `employee_id: int`
- `DELETE` to `/employees/:id`, `/projects/:id` or `/tasks:id` will delete the item that matches the `id` given.
- `GET` to `/tasks/employee/:employee_id` will return all the projects (tasks) assigned to the employee ID given.
- `GET` to `/tasks/project/project_id` will return all the employees (tasks) assigned to the project ID given.

# Database

I used SQLite in memory, which means that everything added from the API after the app is running is not gong to remain after the app is closed. 

If you wish to add data in a more permanent way, you can do so by writing the queries on the `sql` file inside the `migrations` folder.
