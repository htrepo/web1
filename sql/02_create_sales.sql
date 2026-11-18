-- Sales table
-- Tracks individual sale transactions

DROP TABLE IF EXISTS sales;
CREATE TABLE sales (
    id         BIGINT PRIMARY KEY,
    product    VARCHAR(255) NOT NULL,
    quantity   INT          NOT NULL DEFAULT 1,
    price      FLOAT8       NOT NULL,
    sale_date  BIGINT       NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW()) * 1000)::BIGINT
);
