#ifndef CTDOZLITE_IOS_H
#define CTDOZLITE_IOS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Initialize the ctdozlite iOS Rust core with a sandboxed data directory path.
 * Supports multiple account slots (`0` = Primary Account, `1` = Secondary Duo Account).
 * Returns 0 on success, non-zero on error.
 */
int32_t ctdoz_ios_init(const char *data_dir_utf8);

/**
 * Free any UTF-8 C string returned by `ctdoz_ios_*` functions.
 */
void ctdoz_ios_free_string(char *ptr);

/**
 * Return the current core version string (e.g. "0.0.5").
 * Caller must free with `ctdoz_ios_free_string`.
 */
char *ctdoz_ios_version(void);

/**
 * Switch active account slot (`0` = Primary, `1` = Secondary Duo account) and return
 * the active account profile JSON.
 */
char *ctdoz_ios_switch_account(int32_t slot);

/**
 * Return JSON array of all threads for the given account slot (`0` or `1`),
 * filtered by `filter_mode`:
 *   0 = All visible (non-hidden)
 *   1 = Unread only
 *   2 = Pinned only
 *   3 = Groups only
 *   4 = Hidden threads
 * and optional `search_query` (NULL or empty for no search).
 * Guarantees strict `thread_id` separation (#63: duplicate display names never merge).
 */
char *ctdoz_ios_list_threads_json(int32_t slot, int32_t filter_mode, const char *search_query);

/**
 * Return JSON array of messages for `(slot, thread_id)`.
 */
char *ctdoz_ios_get_messages_json(int32_t slot, const char *thread_id);

/**
 * Send a message in `(slot, thread_id)` with optional `reply_to_msg_id` (NULL if none).
 * Also evaluates active auto-reply rules if simulated incoming or bot test is triggered.
 * Returns updated messages JSON array for that thread.
 */
char *ctdoz_ios_send_message_json(
    int32_t slot,
    const char *thread_id,
    const char *content,
    const char *reply_to_msg_id
);

/**
 * Simulate receiving an incoming message from the partner in `(slot, thread_id)`.
 * Automatically runs the Auto-Reply engine (#35) and appends an auto-reply if matched!
 * Returns updated messages JSON array for that thread.
 */
char *ctdoz_ios_receive_message_json(
    int32_t slot,
    const char *thread_id,
    const char *sender_name,
    const char *content
);

/**
 * Toggle or add a reaction emoji on `(slot, thread_id, msg_id, reaction_emoji)`.
 * Returns updated messages JSON array.
 */
char *ctdoz_ios_react_message_json(
    int32_t slot,
    const char *thread_id,
    const char *msg_id,
    const char *reaction_emoji
);

/**
 * Thread context actions (#5, #24):
 *   action = 1 -> Toggle Pin
 *   action = 2 -> Toggle Mute
 *   action = 3 -> Toggle Hidden
 *   action = 4 -> Clear Thread History
 *   action = 5 -> Mark Thread Read
 * Returns updated thread list JSON.
 */
char *ctdoz_ios_thread_action_json(int32_t slot, const char *thread_id, int32_t action);

/**
 * Schedule a message (#38, #24) for `(slot, thread_id, content, deliver_at_ms)`.
 * Returns JSON array of all scheduled messages.
 */
char *ctdoz_ios_schedule_message_json(
    int32_t slot,
    const char *thread_id,
    const char *content,
    int64_t deliver_at_ms
);

/**
 * List all scheduled messages for `slot` as a JSON array.
 */
char *ctdoz_ios_list_scheduled_json(int32_t slot);

/**
 * Cancel a scheduled message by `sched_id`. Returns updated scheduled messages JSON array.
 */
char *ctdoz_ios_cancel_scheduled_json(int32_t slot, int64_t sched_id);

/**
 * Poll and dispatch any due scheduled messages (`deliver_at_ms <= now_ms`) across all slots.
 * Returns a JSON array of dispatched items (empty `[]` if none due) so UIKit can show a toast
 * or local notification and refresh the active chat view.
 */
char *ctdoz_ios_poll_due_scheduled_json(int64_t now_ms);

/**
 * List Auto-Reply rules (#35) as a JSON array.
 */
char *ctdoz_ios_list_autoreply_json(void);

/**
 * Add or toggle or delete an Auto-Reply rule:
 *   op = 1 -> Add rule (`pattern`, `response`, `match_mode`: "contains" | "exact" | "all")
 *   op = 2 -> Toggle rule enabled by `rule_id` (`pattern` holds rule_id)
 *   op = 3 -> Delete rule by `rule_id` (`pattern` holds rule_id)
 * Returns updated Auto-Reply rules JSON array.
 */
char *ctdoz_ios_mutate_autoreply_json(
    int32_t op,
    const char *pattern_or_id,
    const char *response,
    const char *match_mode
);

/**
 * Build signed Zalo ParamCipher v2 request payload (`zpw_ver=671&zpw_type=30`, AES-CBC `zcrypto` + MD5 `signkey`)
 * and return a diagnostic JSON object containing `{ "encrypted_params", "signkey", "endpoint", "failover_hosts" }`.
 */
char *ctdoz_ios_build_signed_request_json(
    const char *service_kind,
    const char *method_name,
    const char *plain_json_params,
    const char *zpw_enk_utf8
);

/**
 * Parse a GitHub Releases API `/releases/latest` JSON payload, compare against current version,
 * and return structured JSON `{ "has_update", "current_version", "latest_version", "title", "published_at", "html_url", "bullets": [...] }`.
 */
char *ctdoz_ios_parse_github_release_json(const char *raw_github_json, const char *current_version_override);

/**
 * Import/migrate external history JSON array (#9) idempotently into `slot`.
 * Returns JSON `{ "imported_threads", "imported_messages", "skipped_duplicates" }`.
 */
char *ctdoz_ios_import_history_json(int32_t slot, const char *external_json);

/**
 * Export all threads and messages for `slot` as a portable JSON backup string (#9).
 */
char *ctdoz_ios_export_history_json(int32_t slot);

#ifdef __cplusplus
}
#endif

#endif /* CTDOZLITE_IOS_H */
