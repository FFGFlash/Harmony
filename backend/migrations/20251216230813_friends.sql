CREATE TYPE friendship_status AS ENUM (
  'pending',
  'accepted',
  'rejected',
  'blocked'
);

CREATE TABLE friendships (
  user_low UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  user_high UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

  sender_id UUID NOT NULL,
  status friendship_status NOT NULL DEFAULT 'pending',

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  CHECK (user_low < user_high),
  CHECK (sender_id IN (user_low, user_high)),

  PRIMARY KEY(user_low, user_high)
);

CREATE INDEX idx_friendships_user_low ON friendships(user_low);
CREATE INDEX idx_friendships_user_high ON friendships(user_high);