CREATE TABLE prefs (
    user_id TEXT NOT NULL,
    pref_key TEXT NOT NULL,
    pref_value TEXT,
    FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (user_id, pref_key)
);