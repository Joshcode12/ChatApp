# ChatApp

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Coverage](https://img.shields.io/badge/coverage-20%25-brightgreen.svg)](#)

A realtime, full‑stack chat application with user authentication, one‑to‑one and group messaging, message persistence, typing indicators, and presence. This repository contains the production-ready frontend and backend, with scripts for local development, and Docker deployment.

Features
- Realtime messaging using Axum.
- Authentication (JWT) with refresh tokens and secure password storage (bcrypt).
- One-to-one and group chats.
- Message persistence in database with pagination and delivery receipts.
- File attachments (images) with uploads to local storage or S3-compatible storage.
- Responsive web client (desktop + mobile).
- Unit and integration tests for backend and frontend.

Tech stack
- Backend: Rust
- Database: PostgreSQL
- Auth: JSON Web Tokens (JWT)
- Frontend: React
- File storage: S3
- Containerization: Docker, Docker Compose

Getting started (development)

Prerequisites
- Node.js >= 18
- npm or yarn
- PostgreSQL (local or connection string)
- Docker

Environment variables
Create a .env file in both backend and frontend (where applicable) based on the provided .env.example files.

Example for backend docker (.env):
```
POSTGRES_USER=admin
POSTGRES_PASSWORD=password
POSTGRES_DB=chat

MINIO_ROOT_USER=admin
MINIO_ROOT_PASSWORD=password
```

API
Base route: /chat

Authentication
- POST /chat/auth/register - register a new user
- POST /chat/auth/login - login, currently returns the id will later implement the JWT

Users
- DELETE /chat/users/deregister/id - delete a user
- POST /chat/users/update - update the user fields

Conversation
- POST /chat/conversation/create - create a new conversation
- DELETE /chat/conversation/delete/id - delete a conversation

Security & Best Practices
- Passwords hashed with bcrypt
- Input validation

Contributing
Thank you for wanting to contribute! A few guidelines:
- Open an issue to discuss major changes first.
- Fork -> branch named feature/desc or fix/desc -> open a pull request.
- Write tests for new behavior or bug fixes.
- Follow the existing code style; run linting before submitting PR.
- Keep commits focused and descriptive.

Code of conduct
This project follows a contributor code of conduct. Please be respectful and considerate.

License
This project is licensed under the MIT License - see the LICENSE file for details.

Contact
Project maintainer: Josh (GitHub: @Joshcode12) and Riker (GitHub: @zpitolava22350)
For questions or help, open an issue on the repository.

---
