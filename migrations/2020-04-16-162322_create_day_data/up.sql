CREATE TABLE moods (
    id SERIAL PRIMARY KEY,
    string TEXT NOT NULL
);

Create TABLE day_data (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL,
    mood_id INT,
    FOREIGN KEY (mood_id) REFERENCES moods(id)
);

INSERT INTO moods(id, string) VALUES (1, 'Terrible');
INSERT INTO moods(id, string) VALUES (2, 'Bad');
INSERT INTO moods(id, string) VALUES (3, 'Normal');
INSERT INTO moods(id, string) VALUES (4, 'Good');
INSERT INTO moods(id, string) VALUES (5, 'Marvelous');