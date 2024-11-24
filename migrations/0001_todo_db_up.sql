CREATE TABLE IF NOT EXISTS tasks (
  id               BIGSERIAL PRIMARY KEY NOT NULL,
  created_at       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  name             VARCHAR NOT NULL,
  description      VARCHAR,
  start_date       TIMESTAMP WITH TIME ZONE,
  end_date         TIMESTAMP WITH TIME ZONE,
);

CREATE TABLE IF NOT EXISTS task_status (
  id         BIGSERIAL PRIMARY KEY NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  name       VARCHAR UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS tasks_task_status (
  id             BIGSERIAL PRIMARY KEY NOT NULL,
  created_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  task_id        BIGSERIAL NOT NULL REFERENCES tasks(id),
  task_status_id BIGSERIAL NOT NULL REFERENCES task_status(id)
  CONSTRAINT no_duplicate_task_status_association UNIQUE (task_id, task_status_id)
);

CREATE TABLE IF NOT EXISTS method (
    id         BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMPT WITH TIME ZONE NOT NULL DEFAULT now(),
    name       VARCHAR UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS task_history (
  id         BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  task_id    BIGSERIAL NOT NULL REFERENCES tasks(id),
  status_id  BIGSERIAL NOT NULL REFERENCES task_status(id),
  method_id  BIGSERIAL NOT NULL REFERENCES method(id)
);
