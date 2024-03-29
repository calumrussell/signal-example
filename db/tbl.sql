CREATE TABLE IF NOT EXISTS event (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS event_log (
    event_id INT NOT NULL,
    message JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS signal (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS signal_log (
    signal_id INT NOT NULL,
    message JSONB NOT NULL
);



