# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Build the Project

First, install the dependencies by running:

```bash
npm install
```

or

```bash
yarn install
```

Then, to build the project, run the following command:

```bash
npm run build
```

This will create a production build of your application in the `dist` directory.

## Start the MySQL Server

To start the MySQL server, follow these steps:

1. Ensure that MySQL is installed on your machine. If not, download and install it from the [official MySQL website](https://dev.mysql.com/downloads/).
2. Start the MySQL server using the command:

```bash
mysql.server start
```

3. To stop the MySQL server, use the command:

```bash
mysql.server stop
```

## Using Docker to Start the MySQL Server

Alternatively, you can use Docker to start the MySQL server. Follow these steps:

1. Ensure that Docker is installed on your machine. If not, download and install it from the [official Docker website](https://www.docker.com/get-started).
2. Navigate to the directory containing the `dev-docker.yaml` file.
3. Start the MySQL container using Docker Compose:

```bash
docker-compose -f docker-compose/dev-docker.yaml up -d
```

4. To stop the MySQL container, use the command:

```bash
docker-compose -f docker-compose/dev-docker.yaml down
```
