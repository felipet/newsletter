-- We wrap the whole migration in a transaction to make sure
-- it succeeds or fails atomically.
BEGIN;
    -- Backfill `status` for historical entries.
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status IS NULL;
    -- Now, we're ready to set it to mandatory for new entries.
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;

