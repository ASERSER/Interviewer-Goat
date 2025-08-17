// Audio → VAD → ASR → Router → LLM → HUD
loop {
    let frame = get_audio_frame();
    if vad::process_audio_frame(&frame).is_speech {
        let partial = asr::transcribe_stream(&frame);
        let context = ocr::capture_window();
        let intent = router::route(context);
        let suggestion = orchestrator::generate_suggestion(intent);
        send_to_hud(suggestion);
    }
}