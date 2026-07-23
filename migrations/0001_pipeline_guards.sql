CREATE TABLE IF NOT EXISTS argus_message_receipts (message_id TEXT PRIMARY KEY, stream TEXT NOT NULL, payload_hash TEXT NOT NULL, received_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE TABLE IF NOT EXISTS argus_reorg_invalidations (message_id TEXT PRIMARY KEY, chain_id BIGINT NOT NULL, fork_block BIGINT NOT NULL, replay_epoch BIGINT NOT NULL, payload JSONB NOT NULL, created_at TIMESTAMPTZ NOT NULL DEFAULT now());
CREATE INDEX IF NOT EXISTS argus_message_receipts_stream_idx ON argus_message_receipts(stream, received_at DESC);
