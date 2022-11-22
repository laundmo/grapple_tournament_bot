CREATE TABLE regions (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "short" VARCHAR(2) NOT NULL
);

CREATE TABLE playercounts (
    "time" TIMESTAMPTZ NOT NULL,
    country int references regions(id),
    amount int NOT NULL,
    PRIMARY KEY ("time", country)
);

SELECT create_hypertable('playercounts', 'time');