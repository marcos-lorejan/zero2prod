# System Patterns

This document describes the system architecture and key technical decisions.

## System Architecture

- Client-server architecture.
- The frontend is a single-page application (SPA) built with React.
- The backend is a web service written in Rust using the Axum framework.
- The backend is modular, with separate modules for handlers, database interactions, and models.

## Key Technical Decisions

- Use of Axum for the web framework.
- Separation of concerns between handlers, database logic, and data models.

## Design Patterns

- The backend follows a layered architecture, with a clear separation between the API layer (handlers) and the data access layer (db).
