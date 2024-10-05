# Lightning Network Nodes Data Server

This project is a Rust-based server that fetches and serves data about nodes in the Lightning Network. It collects node data from an external API, stores it in a PostgreSQL database, and exposes the data through a JSON API for external consumption.

## Build Tools & Versions Used
- **Rust** (latest stable version)
- **PostgreSQL**
- **Warp** (for HTTP server)
- **SQLx** (for database interaction)
- **Reqwest** (for API requests)
- **Serde** (for serialization/deserialization)
- **Tokio** (for asynchronous programming)
- **Chrono** (for date and time handling)

## Steps to Run the App
1. Clone the repository:
```
git clone https://github.com/KainanAugusto/Lightning-Network-Nodes-Data-Server
```
2. Install dependencies:
```
cargo build
```

3. Set up PostgreSQL and run the code to create the nodes table:
```

CREATE TABLE nodes (
    public_key VARCHAR(255) PRIMARY KEY,
    alias VARCHAR(255),
    capacity VARCHAR(255),
    first_seen VARCHAR(255)
);
```

4. Update the database connection settings in your .env file:
```
DATABASE_URL=postgres://user:password@localhost/database_name
```

5. Start the server:
```
cargo run
```

6. The server will be available to GET requests at http://127.0.0.1:3030/nodes

## Project Focus and Problem Solving
The goal was to create a simple and efficient server to collect, store, and expose data about Lightning Network nodes. This project addresses the need for real-time data about network nodes, allowing external services to access this information through a dedicated API.

## Time Investment
Approximately 5 hours were spent on this project, including architecture design, database setup, data fetching from the external API, and handling asynchronous operations using Rust's ecosystem.

## Trade-Offs Made
Due to time constraints, I focused on core functionality—data fetching, storage, and exposure. With additional time, I would implement:
- Enhanced error handling (e.g., more detailed user feedback for API failures).
- Unit and integration tests for various parts of the code.
- More robust API documentation.

## Weaknesses
The project’s weakest aspect may be the lack of robust error handling and testing. While basic logging is implemented, the error responses could be improved to be more user-friendly.