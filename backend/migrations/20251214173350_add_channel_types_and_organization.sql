CREATE TYPE channel_type AS ENUM ('text', 'voice', 'dm', 'group_dm');

ALTER TABLE channels
ADD COLUMN channel_type channel_type NOT NULL DEFAULT 'text',
ADD COLUMN topic TEXT,
ADD COLUMN is_private BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE dm_channels (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  channel_id UUID NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(channel_id)
);

CREATE TABLE dm_participants (
  dm_channel_id UUID NOT NULL REFERENCES dm_channels(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (dm_channel_id, user_id)
);

CREATE INDEX idx_dm_participants_user_id ON dm_participants(user_id);
CREATE INDEX idx_dm_participants_dm_channel_id ON dm_participants(dm_channel_id);

CREATE TABLE server_folders (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name VARCHAR(100) NOT NULL,
  color VARCHAR(7),
  position INTEGER NOT NULL DEFAULT 0,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_server_folders_user_id ON server_folders(user_id);

CREATE TABLE server_organization (
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  server_id UUID NOT NULL REFERENCES servers(id) ON DELETE CASCADE,
  folder_id UUID REFERENCES server_folders(id) ON DELETE SET NULL,
  position INTEGER NOT NULL DEFAULT 0,
  is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, server_id)
);

CREATE INDEX idx_server_organization_user_id ON server_organization(user_id);
CREATE INDEX idx_server_organization_folder_id ON server_organization(folder_id);

CREATE OR REPLACE FUNCTION create_server_organization()
RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO server_organization (user_id, server_id, position)
  VALUES (
    NEW.user_id,
    NEW.server_id,
    COALESCE(
      (SELECT MAX(position) + 1 FROM server_organization WHERE user_id = NEW.user_id),
      0
    )
  )
  ON CONFLICT (user_id, server_id) DO NOTHING;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_create_server_organization
AFTER INSERT ON server_members
FOR EACH ROW
EXECUTE FUNCTION create_server_organization();

INSERT INTO server_organization (user_id, server_id, position)
SELECT
  user_id,
  server_id,
  ROW_NUMBER() OVER (PARTITION BY user_id ORDER BY joined_at) - 1 as position
FROM server_members
ON CONFLICT (user_id, server_id) DO NOTHING;

ALTER TABLE channels ALTER COLUMN server_id DROP NOT NULL;