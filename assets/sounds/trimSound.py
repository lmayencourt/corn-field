from pydub import AudioSegment

def trim_last_second(file_path, output_path):
    # Load the audio file
    audio = AudioSegment.from_file(file_path, format="ogg")
    
    # Calculate duration in milliseconds
    duration_ms = len(audio)
    
    # Trim the last second (1000 milliseconds)
    trimmed_audio = audio[:duration_ms - 550]
    trimmed_audio = trimmed_audio[550:]
    
    # Export the trimmed audio
    trimmed_audio.export(output_path, format="ogg")

# Example usage
trim_last_second("game.ogg", "gamenew.ogg")
