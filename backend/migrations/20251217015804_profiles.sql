CREATE TYPE profile_status AS ENUM (
  'online',
  'away',
  'offline',
  'dnd'
);

CREATE TABLE profiles (
  user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,

  display_name VARCHAR(100),
  bio TEXT,
  avatar_url TEXT,
  banner_url TEXT,

  status profile_status NOT NULL DEFAULT 'online',
  custom_status TEXT,
  status_emoji VARCHAR(10),

  show_online_status BOOLEAN NOT NULL DEFAULT TRUE,
  allow_dms BOOLEAN NOT NULL DEFAULT TRUE,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_profiles_user_id ON profiles(user_id);

INSERT INTO profiles (user_id)
SELECT id FROM users
ON CONFLICT (user_id) DO NOTHING;

CREATE OR REPLACE FUNCTION create_profile()
RETURNS TRIGGER AS $$
BEGIN
  INSERT INTO profiles (user_id)
  VALUES (NEW.id)
  ON CONFLICT (user_id) DO NOTHING;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_create_profile
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION create_profile();