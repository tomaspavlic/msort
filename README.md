# msort

msort is a powerful command-line tool designed to organize your movie and TV show files into a folder structure ideal for use with [Plex Media Server](https://www.plex.tv). It processes file metadata to ensure your media library is clean, consistent, and easy to navigate.

## Features
- Automatically detects and organizes movies and TV shows.
- Creates a Plex-friendly folder structure:
  - **Movies**: `Movies/<Movie Title> (<Year>)/<Movie File>`
  - **TV Shows**: `TV Shows/<Show Title>/Season <Number>/<Episode File>`
- Handles common naming conventions and formats.
- Supports additional metadata like release year, season, and episode numbers.
- Safe operation with a dry-run mode to preview changes.

## Installation

### Requirements
Rust programming language installed. Use the [Rust toolchain installed](https://rustup.rs).

### Steps
Install the application via `cargo` (Rust's package manager):
```bash
cargo install msort
```

## Usage

### Basic Command
```bash
msort --input <source_file_path> --base-dir <destination_directory> --api-key
```

### Options
- `--input` (required): Path to the media file
- `--base-dir` (required): Path to the directory where organized files will be saved.
- `--dry-run`: Preview changes without moving any files.
- `--verbose`: Enable detailed logging for debugging.

### Examples

#### Organize Media Files
```bash
msort --input ~/Downloads/Media/house.s01e02.mkv --base-dir ~/MediaLibrary
```

#### Enable Verbose Logging
```bash
msort --input ~/Downloads/Media/house.s01e02.mkv --base-dir ~/MediaLibrary -vv
```

## Folder Structure
The application creates a Plex-compatible folder structure:

### Movies
```
Movies/
  Inception (2010)/
    Inception (2010).mp4
  The Matrix (1999)/
    The Matrix (1999).mkv
```

### TV Shows
```
TV Shows/
  Breaking Bad/
    Season 01/
      Breaking Bad - S01E01 - Pilot.mp4
      Breaking Bad - S01E02 - Cat's in the Bag.mp4
    Season 02/
      Breaking Bad - S02E01 - Seven Thirty-Seven.mp4
```

## Contributing
Contributions are welcome! To contribute:
1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-name`.
3. Commit your changes: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature-name`.
5. Open a pull request.

## License
This project is licensed under the [MIT License](LICENSE).

## Acknowledgments
- Thanks to the [Plex](https://www.plex.tv) community for guidelines on media organization.
- Inspired by tools like [FileBot](https://www.filebot.net).

## Contact
For questions or support, open an issue on the [GitHub repository](https://github.com/tomaspavlic/msort).
