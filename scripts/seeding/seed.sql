DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id VARCHAR(64) PRIMARY KEY,
    first_name VARCHAR(16) NOT NULL,
    last_name VARCHAR(16) NOT NULL,
    email VARCHAR(32) NOT NULL,
    password VARCHAR(64) NOT NULL
);

INSERT INTO users VALUES (
    'id',
    'first_name',
    'last_name',
    'user@example.com',
    'password'
);
