ALTER TABLE servers
ADD COLUMN main_channel_id UUID REFERENCES channels(id) ON DELETE SET NULL;

CREATE INDEX idx_servers_main_channel_id ON servers(main_channel_id);

UPDATE servers s
SET main_channel_id = (
  SELECT c.id
  FROM channels c
  WHERE c.server_id = s.id
  ORDER BY c.position ASC, c.created_at ASC
  LIMIT 1
)
WHERE EXISTS (
  SELECT 1 FROM channels WHERE server_id = s.id
);