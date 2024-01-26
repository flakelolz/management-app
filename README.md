# How to open

Since this web application is just a backend made with Rust which serves an HTML, the whole application is just a binary executable.

Go to the `Releases` section of this repository to download the executable.

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
- `http://localhost:3001/employees/:id`
- `http://localhost:3001/projects/:id`

With this endpoints you can perform CRUD operations from a tool that allows you to send request like https://hoppscotch.io/

# API

The following API requests are valid:
- `GET` to `/employees` or `/projects` will return a list of all the respective items in the database.
- `GET` to `/employees/:id` or `/projects/:id` will return the item that matches the `id` passed through the URL.
- `POST` to `/employees` or `/projects` will create a new item when paired with a `JSON` payload.
	- For employees the JSON will need `name: string`
	- For projects the JSON will need `name: string`
- `PUT` to `/employees/:id` or `/projects/:id` will update the information of the item that matches the `id` given. Send a JSON payload with the new information.
	- For employees the JSON will need `name: string`
	- For projects the JSON will need `name: string`
- `DELETE` to `/employees/:id` or `/projects/:id` will delete the item that matches the `id` given.
