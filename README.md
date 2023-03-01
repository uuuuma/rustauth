# rustauth
## Dependencies
- bash
- rustc
- cargo
- docker compose v2
- make

## Preparation
### Seed postgres
1. compose up postgres container
    ```bash
    docker compose up
    ```
2. seed data
    ```bash
    make seed
    ```
