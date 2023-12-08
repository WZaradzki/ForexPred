-- Your SQL goes here
CREATE TABLE currency_rates (
    id VARCHAR(255) PRIMARY KEY,
    from_currency_id VARCHAR(255) NOT NULL,
    to_currency_id VARCHAR(255) NOT NULL,
    rate DECIMAL(10, 2) NOT NULL,
    date DATE NOT NULL,
    FOREIGN KEY (from_currency_id) REFERENCES currencies(id),
    FOREIGN KEY (to_currency_id) REFERENCES currencies(id),
    INDEX (rate, date)
);