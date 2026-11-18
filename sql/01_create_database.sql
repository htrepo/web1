-- Creates the rusty database and sa user
-- Run as the postgres superuser

CREATE USER sa WITH PASSWORD 'Password1!';

CREATE DATABASE rusty
    OWNER sa
    ENCODING 'UTF8';
