-- DEV ONLY - Brute force DROP DB
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
	usename = 'udesk' OR datname = 'udesk';

DROP DATABASE IF EXISTS udesk;
DROP USER IF EXISTS udesk;

-- DEV ONLY - Dev only password
CREATE USER udesk PASSWORD 'udesk';
CREATE DATABASE udesk owner udesk ENCODING = 'UTF-8';
