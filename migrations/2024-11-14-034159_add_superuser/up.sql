-- Your SQL goes here
INSERT INTO users (id, username, hashed_password, full_name, created_at, updated_at)
VALUES (
    '7763abad-f33d-4308-b89d-8897e9037d16',
    'superuser',
    '$2b$12$pv3Bzt9t1zfwJA3nUskK1eYymNXspJjNZyn70a23VwURHqolapmKe',
    'Superuser',
    NOW(),
    NOW()
);
