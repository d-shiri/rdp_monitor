CREATE TABLE Users (
    id VARCHAR(255) PRIMARY KEY,
    full_name VARCHAR(255),
    team VARCHAR (255),
    creation_date DATETIME,
    active TINYINT(1),
    admin TINYINT(1)
);
