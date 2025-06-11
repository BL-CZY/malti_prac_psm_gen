use hound::{WavReader, WavSpec, WavWriter};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct ResultElement {
    pub sentence: String,
    pub audio_stop: f64,
}

/// Combines segments of clips together alternately with 1-second gaps
///
/// # Arguments
/// * `clips1` - First list of paths to .wav files
/// * `clips2` - Second list of paths to .wav files (must be same length as clips1)
/// * `entries1` - First list of sentence strings corresponding to clips1
/// * `entries2` - Second list of sentence strings corresponding to clips2
/// * `output_path` - Path where the combined file will be saved
///
/// # Returns
/// * `Result<(PathBuf, Vec<ResultElement>), Box<dyn std::error::Error>>` - The output path and result elements
pub fn combine_clips_alternately(
    clips1: &[PathBuf],
    clips2: &[PathBuf],
    entries1: &[String],
    entries2: &[String],
    output_path: &Path,
    output_path_json: &Path,
) -> Result<(PathBuf, Vec<ResultElement>), Box<dyn std::error::Error>> {
    if clips1.len() != clips2.len()
        || clips1.len() != entries1.len()
        || clips2.len() != entries2.len()
    {
        return Err("All input lists must have the same length".into());
    }

    if clips1.is_empty() {
        return Err("Input lists cannot be empty".into());
    }

    // Clean existing file if it exists
    if output_path.exists() {
        fs::remove_file(output_path)?;
    }

    // Read the first file to get the audio specification
    let first_reader = WavReader::open(&clips1[0])?;
    let spec = first_reader.spec();

    // Validate that all files have the same specification
    for (i, clip_path) in clips1.iter().chain(clips2.iter()).enumerate() {
        let reader = WavReader::open(clip_path)?;
        if reader.spec() != spec {
            return Err(format!("File {} has different audio specification", i).into());
        }
    }

    // Create output writer
    let mut writer = WavWriter::create(output_path, spec)?;

    let sample_rate = spec.sample_rate as f64;
    let channels = spec.channels as usize;
    let gap_samples = (sample_rate * channels as f64) as usize; // 1 second gap

    let mut current_time = 0.0;
    let mut result_elements = Vec::new();

    // Process clips alternately
    for i in 0..clips1.len() {
        // Process clip from first list
        let duration1 = write_clip_to_output(&mut writer, &clips1[i], &spec)?;
        current_time += duration1;

        result_elements.push(ResultElement {
            sentence: entries1[i].clone(),
            audio_stop: current_time,
        });

        // Add 1-second gap
        write_silence(&mut writer, gap_samples)?;
        current_time += 1.0;

        // Process clip from second list
        let duration2 = write_clip_to_output(&mut writer, &clips2[i], &spec)?;
        current_time += duration2;

        result_elements.push(ResultElement {
            sentence: entries2[i].clone(),
            audio_stop: current_time,
        });

        // Add 1-second gap after each pair (except the last one)
        if i < clips1.len() - 1 {
            write_silence(&mut writer, gap_samples)?;
            current_time += 1.0;
        }
    }

    writer.finalize()?;

    std::fs::write(
        output_path_json,
        serde_json::to_string_pretty(&result_elements).unwrap(),
    )
    .unwrap();

    Ok((output_path.to_path_buf(), result_elements))
}

/// Writes a single clip to the output writer and returns its duration in seconds
fn write_clip_to_output(
    writer: &mut WavWriter<std::io::BufWriter<std::fs::File>>,
    clip_path: &Path,
    expected_spec: &WavSpec,
) -> Result<f64, Box<dyn std::error::Error>> {
    let mut reader = WavReader::open(clip_path)?;

    let sample_count = reader.len() as f64;
    let duration =
        sample_count / (expected_spec.sample_rate as f64 * expected_spec.channels as f64);

    // Write samples based on the bit depth
    match expected_spec.bits_per_sample {
        16 => {
            for sample in reader.samples::<i16>() {
                writer.write_sample(sample?)?;
            }
        }
        24 => {
            for sample in reader.samples::<i32>() {
                writer.write_sample(sample?)?;
            }
        }
        32 => {
            if expected_spec.sample_format == hound::SampleFormat::Float {
                for sample in reader.samples::<f32>() {
                    writer.write_sample(sample?)?;
                }
            } else {
                for sample in reader.samples::<i32>() {
                    writer.write_sample(sample?)?;
                }
            }
        }
        _ => return Err("Unsupported bit depth".into()),
    }

    Ok(duration)
}

/// Writes silence (zeros) for the specified number of samples
fn write_silence(
    writer: &mut WavWriter<std::io::BufWriter<std::fs::File>>,
    sample_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..sample_count {
        writer.write_sample(0i16)?; // Writing zero samples as silence
    }
    Ok(())
}
