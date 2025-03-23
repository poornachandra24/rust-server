# Rust-Server

This project is a REST API server built using Rust and the Axum framework. It includes endpoints for health checks and predictions, and it connects to a PostgreSQL database for data storage and retrieval.

## Features

1. [x] Build a "Hello World" REST API with Axum.
2. [x] Add a second endpoint called `/predictions`.
3. [x] Add a local PostgreSQL database to the project.
4. [x] Connect the `/predictions` endpoint to the PostgreSQL database to return predictions.

## Endpoints

### Health Check
- **Endpoint:** `/health`
- **Method:** `GET`
- **Description:** Returns a simple message indicating the server is running and healthy.

### Predictions
- **Endpoint:** `/predictions`
- **Method:** `GET`
- **Query Parameters:**
  - `product_id` (String): The ID of the product for which predictions are requested.
- **Description:** Fetches prediction data for a given product from the PostgreSQL database.

## Setup Instructions

1. Clone the repository.
2. Ensure you have Rust installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/).
3. Start a PostgreSQL database using Docker:
    ```sh
    make postgres-up
    ```
4. Set up the .env file with the database connection string:
    ```bash
    DATABASE_URL=postgresql://postgres:password@localhost/db
    ```
5. Run the server in development mode:
    ```bash
    make run
    ```

### Running in Release Mode
To run the server in release mode for optimized performance:

1. Build the release version:
    ```bash
    cargo build --release
    ```
2. Run the release binary:
    ```bash
    ./target/release/rust-rest-server
    ```

## Testing the Endpoints

### Health Check:
```bash
curl http://localhost:3001/health
```
### Predictions
To fetch predictions, use the following command:

```bash
make predictions product_id=<product_id>
```
#### Example Predictions
For product_id=XRP/EUR:
```
{"product_id":"XRP/EUR","value":0.5}
```
For product_id=ETH/EUR:
```
{"product_id":"ETH/EUR","value":3000.0}
```


## Troubleshooting
If you receive an "Empty reply from server" error, ensure:
- The DATABASE_URL in the .env file is correctly configured.
- The predictions table in the PostgreSQL database contains the requested product_id.
- The server is running and accessible at http://localhost:3001.


## Acknowledgments
Special thanks to [@Paulescu](https://github.com/Paulescu) for his contributions and insights that inspired this project. 
