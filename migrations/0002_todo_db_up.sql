INSERT INTO task_status (name) VALUES ('created') ON CONFLICT DO NOTHING;
INSERT INTO task_status (name) VALUES ('deleted') ON CONFLICT DO NOTHING;
INSERT INTO task_status (name) VALUES ('open')    ON CONFLICT DO NOTHING;
INSERT INTO task_status (name) VALUES ('closed')  ON CONFLICT DO NOTHING;

INSERT INTO method (name) VALUES ('post')   ON CONFLICT DO NOTHING;
INSERT INTO method (name) VALUES ('put')    ON CONFLICT DO NOTHING;
INSERT INTO method (name) VALUES ('delete') ON CONFLICT DO NOTHING;
