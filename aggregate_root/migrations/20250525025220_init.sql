CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    updated_datetime TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_datetime TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
CREATE TABLE IF NOT EXISTS departments (
    id INTEGER PRIMARY KEY,
    user_id INT,
    name VARCHAR(100) NOT NULL,
    updated_datetime TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    created_datetime TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);