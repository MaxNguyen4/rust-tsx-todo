INSERT INTO users (id, username, password)
VALUES (1, 'max', 'max');

INSERT INTO todos (id, user_id, todo, category, deadline)
VALUES 
    (1, 1, 'Beat up Tyler Gwin', 'Revenge', '2025-03-10'),
    (2, 1, 'Go gambling', 'Work', '2025-03-12'),
    (3, 1, 'Smoke cigarettes', 'Health', '2025-03-15');