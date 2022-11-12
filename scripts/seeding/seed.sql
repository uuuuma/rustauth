DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email varchar(32),
    password varchar(16)
);

INSERT INTO users (email, password) VALUES (
    'user@example.com',
    'password'
);
