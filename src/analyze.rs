use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{AppScreen, MyApp};

impl MyApp {
    // Handler for the analyze text button
    pub fn handle_analyze_text(&mut self) {
        self.analysis_data.is_processing = true;
        self.analysis_data.processing_status = "Starting audio processing...".to_string();

        // Convert text areas to line vectors
        self.analysis_data.text_entries_1 = self
            .text_area_1
            .split('\n')
            .map(|s| s.to_string())
            .collect();

        self.analysis_data.text_entries_2 = self
            .text_area_2
            .split('\n')
            .map(|s| s.to_string())
            .collect();

        // Process MP3 files if they exist
        if let Some(mp3_path) = &self.mp3_file_1 {
            match self.process_mp3_file(mp3_path, 1) {
                Ok(clips) => self.analysis_data.audio_clips_1 = clips,
                Err(e) => {
                    self.analysis_data.processing_status =
                        format!("Error processing MP3 file 1: {}", e);
                    self.analysis_data.is_processing = false;
                    return;
                }
            }
        }

        if let Some(mp3_path) = &self.mp3_file_2 {
            match self.process_mp3_file(mp3_path, 2) {
                Ok(clips) => self.analysis_data.audio_clips_2 = clips,
                Err(e) => {
                    self.analysis_data.processing_status =
                        format!("Error processing MP3 file 2: {}", e);
                    self.analysis_data.is_processing = false;
                    return;
                }
            }
        }

        self.analysis_data.is_processing = false;
        self.analysis_data.processing_status = "Processing completed!".to_string();
        self.current_screen = AppScreen::TextAnalyzer;
    }

    // Convert MP3 to WAV and split by 2-second gaps
    fn process_mp3_file(&self, mp3_path: &Path, file_id: u8) -> Result<Vec<PathBuf>, String> {
        let temp_dir = std::env::temp_dir().join(format!("audio_analysis_{}", file_id));
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temp directory: {}", e))?;

        // Step 1: Convert MP3 to WAV
        let wav_path = temp_dir.join("converted.wav");
        let mp3_to_wav_result = Command::new("ffmpeg")
            .args([
                "-i",
                mp3_path.to_str().unwrap(),
                "-acodec",
                "pcm_s16le",
                "-ar",
                "44100",
                "-ac",
                "2",
                wav_path.to_str().unwrap(),
                "-y", // Overwrite output file
            ])
            .output();

        match mp3_to_wav_result {
            Ok(output) => {
                if !output.status.success() {
                    return Err(format!(
                        "FFmpeg conversion failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
            Err(e) => {
                return Err(format!(
                    "Failed to run FFmpeg: {}. Make sure FFmpeg is installed.",
                    e
                ));
            }
        }

        // Step 2: Split WAV by 2-second gaps using silence detection
        let clips_dir = temp_dir.join("clips");
        std::fs::create_dir_all(&clips_dir)
            .map_err(|e| format!("Failed to create clips directory: {}", e))?;

        let split_result = Command::new("ffmpeg")
            .args([
                "-i",
                wav_path.to_str().unwrap(),
                "-af",
                "silencedetect=noise=-30dB:duration=2.0",
                "-f",
                "segment",
                "-segment_time",
                "30", // Max segment length as fallback
                "-reset_timestamps",
                "1",
                "-map",
                "0:a",
                "-c:a",
                "pcm_s16le",
                clips_dir.join("clip_%03d.wav").to_str().unwrap(),
            ])
            .output();

        match split_result {
            Ok(output) => {
                if !output.status.success() {
                    // Fallback: simple time-based splitting if silence detection fails
                    self.split_wav_by_time(&wav_path, &clips_dir)?;
                }
            }
            Err(_) => {
                // Fallback: simple time-based splitting
                self.split_wav_by_time(&wav_path, &clips_dir)?;
            }
        }

        // Collect all generated clip files
        let mut clips = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&clips_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("wav") {
                        clips.push(path);
                    }
                }
            }
        }

        clips.sort();
        Ok(clips)
    }

    // Fallback method: split WAV by fixed time intervals
    fn split_wav_by_time(&self, wav_path: &Path, output_dir: &Path) -> Result<(), String> {
        // Get audio duration first
        let duration_output = Command::new("ffprobe")
            .args([
                "-v",
                "quiet",
                "-show_entries",
                "format=duration",
                "-of",
                "default=noprint_wrappers=1:nokey=1",
                wav_path.to_str().unwrap(),
            ])
            .output()
            .map_err(|e| format!("Failed to get audio duration: {}", e))?;

        let duration_str = String::from_utf8_lossy(&duration_output.stdout);
        let total_duration: f64 = duration_str
            .trim()
            .parse()
            .map_err(|e| format!("Failed to parse duration: {}", e))?;

        // Split into 10-second chunks (adjustable)
        let chunk_duration = 10.0;
        let mut start_time = 0.0;
        let mut clip_index = 0;

        while start_time < total_duration {
            let output_file = output_dir.join(format!("clip_{:03}.wav", clip_index));

            let split_result = Command::new("ffmpeg")
                .args([
                    "-i",
                    wav_path.to_str().unwrap(),
                    "-ss",
                    &start_time.to_string(),
                    "-t",
                    &chunk_duration.to_string(),
                    "-c",
                    "copy",
                    output_file.to_str().unwrap(),
                    "-y",
                ])
                .output();

            match split_result {
                Ok(output) => {
                    if !output.status.success() {
                        eprintln!(
                            "Warning: Failed to create clip {}: {}",
                            clip_index,
                            String::from_utf8_lossy(&output.stderr)
                        );
                    }
                }
                Err(e) => eprintln!(
                    "Warning: Failed to run FFmpeg for clip {}: {}",
                    clip_index, e
                ),
            }

            start_time += chunk_duration;
            clip_index += 1;
        }

        Ok(())
    }

    // Modified analysis screen renderer
    pub fn render_text_analyzer_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("Interactive Text & Audio Analyzer");
        ui.separator();

        if self.analysis_data.is_processing {
            ui.label(&self.analysis_data.processing_status);
            ui.spinner();
            return;
        }

        if !self.analysis_data.processing_status.is_empty() {
            ui.label(&self.analysis_data.processing_status);
            ui.separator();
        }

        ui.columns(2, |columns| {
            // Left column - Text Area 1 with audio
            render_interactive_text_column(
                &mut columns[0],
                "Text Area 1",
                &mut self.analysis_data.text_entries_1,
                &self.analysis_data.audio_clips_1,
                1,
            );

            // Render second column
            render_interactive_text_column(
                &mut columns[1],
                "Text Area 2",
                &mut self.analysis_data.text_entries_2,
                &self.analysis_data.audio_clips_2,
                2,
            );
        });
    }
}

fn render_interactive_text_column(
    ui: &mut egui::Ui,
    title: &str,
    text_entries: &mut Vec<String>,
    audio_clips: &[PathBuf],
    _column_id: usize,
) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label(format!("{} - Interactive Entries:", title));
            ui.add_space(5.0);

            // Add new entry button
            if ui.button("+ Add Entry").clicked() {
                text_entries.push(String::new());
            }

            ui.add_space(5.0);

            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    let mut to_remove = None;
                    let mut to_move_up = None;
                    let mut to_move_down = None;

                    // Collect UI actions without mutating text_entries
                    for (i, entry) in text_entries.iter().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("{}:", i + 1));

                                // Move up button
                                if i > 0 && ui.small_button("↑").clicked() {
                                    to_move_up = Some(i);
                                }

                                // Move down button
                                if i < text_entries.len() - 1 && ui.small_button("↓").clicked() {
                                    to_move_down = Some(i);
                                }

                                // Delete button
                                if ui.small_button("🗑").clicked() {
                                    to_remove = Some(i);
                                }
                            });

                            // Text input (we can't mutate here yet)
                            ui.label(entry);

                            // Audio player
                            if let Some(audio_clip) = audio_clips.get(i) {
                                ui.horizontal(|ui| {
                                    ui.label("🔊");
                                    if ui.button("Play").clicked() {
                                        play_audio_clip(audio_clip);
                                    }
                                    ui.label(format!(
                                        "Clip: {}",
                                        audio_clip
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy()
                                    ));
                                });
                            } else {
                                ui.label("No audio clip");
                            }
                        });

                        ui.add_space(5.0);
                    }
                });

            ui.add_space(10.0);
            ui.separator();
            ui.label(format!("Total entries: {}", text_entries.len()));
            ui.label(format!("Audio clips: {}", audio_clips.len()));
        });
    });
}

fn play_audio_clip(clip_path: &Path) {
    // Platform-specific audio playback
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("powershell")
            .args([
                "-c",
                &format!(
                    "(New-Object Media.SoundPlayer '{}').PlaySync()",
                    clip_path.display()
                ),
            ])
            .spawn();
    }

    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("afplay")
            .arg(clip_path.to_str().unwrap())
            .spawn();
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("aplay")
            .arg(clip_path.to_str().unwrap())
            .spawn();
    }
}
