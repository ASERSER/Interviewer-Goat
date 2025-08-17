fn end_call_summary(meeting_id: i32) {
    let chunks = storage::get_transcript_chunks(meeting_id);
    let context = rag::query_memory("recap");
    let note = notes::build_notes(&chunks);
    storage::save_note(meeting_id, note);
    hud::show_export_ready();
}