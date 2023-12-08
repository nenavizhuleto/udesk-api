
INSERT INTO companies (name, description, slug, itn) VALUES
('company', 'description', 'company', '1234567890');

INSERT INTO departments (company_id, name, description, address, contacts) VALUES
(1000, 'department 1', 'description', 'address', 'contacts'),
(1000, 'department 2', 'description', 'address', 'contacts');

INSERT INTO users (username, password, first_name, last_name, role) VALUES
('username_1', 'password1', 'user_firstname', 'user_lastname', 'employee'),
('username_2', 'password2', 'user_firstname', 'user_lastname', 'executor');

INSERT INTO tickets (user_id, title, description) VALUES
(1000, 'Ticket 1', 'Description 1'),
(1000, 'Ticket 2', 'Description 2'),
(1001, 'Ticket 3', 'Description 3'),
(1001, 'Ticket 4', 'Description 4');

