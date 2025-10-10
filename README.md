# Terrier

## Getting started

See [SETUP.md](./SETUP.md) for instructions on how to host Terrier in production.

## Development

After opening the repository in VS Code, press "Open Workspace" in the `terrier.code-workspace` file.

This project uses `just`, a make-like command runner. After [installing it](https://github.com/casey/just?tab=readme-ov-file#packages), you can use the following command to start the application in development mode:

```bash
just dev
```

The first time you run this, it will also run the setup script to initialize the `.env` file. You will need to fill in the OIDC credentials and admin emails in this file before starting the dev server again.

Afterward, it will start the supporting services (PostgreSQL, MinIO, pgAdmin, and nginx) and launch the frontend and backend applications. The frontend will be available at [http://localhost:8080](http://localhost:8080) and the backend at [http://localhost:8080/api](http://localhost:8080/api).
