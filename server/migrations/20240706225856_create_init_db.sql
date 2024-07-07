-- Add migration script here

CREATE TABLE users (
    id VARCHAR(36) NOT NULL,
    active BOOLEAN NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    date_of_birth VARCHAR(255) NOT NULL,
    cpf VARCHAR(255) NOT NULL,
    location_lat FLOAT NOT NULL,
    location_lng FLOAT NOT NULL,
    uniques_store VARCHAR(255),
    uniques_station VARCHAR(255),
    uniques_can_change BOOLEAN NOT NULL,
    role VARCHAR(255) NOT NULL,
    phone_number VARCHAR(255),
    PRIMARY KEY (id)
);


CREATE TABLE stores (
    id VARCHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    location_addr VARCHAR(255) NOT NULL,
    location_neighborhood VARCHAR(255) NOT NULL,
    location_postal_code VARCHAR(255) NOT NULL,
    location_city VARCHAR(255) NOT NULL,
    location_state VARCHAR(255) NOT NULL,
    location_lat FLOAT NOT NULL,
    location_lng FLOAT NOT NULL,
    PRIMARY KEY (id)
);


CREATE TABLE stations (
    id VARCHAR(36) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    location_addr VARCHAR(255) NOT NULL,
    location_neighborhood VARCHAR(255) NOT NULL,
    location_postal_code VARCHAR(255) NOT NULL,
    location_city VARCHAR(255) NOT NULL,
    location_state VARCHAR(255) NOT NULL,
    location_lat FLOAT NOT NULL,
    location_lng FLOAT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE sales (
    id VARCHAR(36) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    seller_id VARCHAR(36),
    buyer_id VARCHAR(36) NOT NULL,
    status VARCHAR(255) NOT NULL,
    info_currency VARCHAR(255) NOT NULL,
    info_place VARCHAR(255) NOT NULL,
    info_amount VARCHAR(255),
    info_price VARCHAR(255),
    PRIMARY KEY (id)
);