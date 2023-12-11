CREATE TABLE currency_rates (
    id SERIAL PRIMARY KEY,
    from_currency_iso VARCHAR(3) NOT NULL,
    to_currency_iso VARCHAR(3) NOT NULL,
    rate DECIMAL(10, 5) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (from_currency_iso) REFERENCES currencies(iso),
    FOREIGN KEY (to_currency_iso) REFERENCES currencies(iso),
    INDEX (rate),
    INDEX (created_at)
);