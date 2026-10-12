// Dropped files are copied into %LOCALAPPDATA%\Coucou\inbox so the original is
// never touched and the copy survives the drag source going away.
// The inbox is swept of anything older than a week, as on macOS.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use serde::Serialize;

use crate::settings;

const KEEP_FOR: Duration = Duration::from_secs(7 * 24 * 60 * 60);

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DroppedFile {
    pub name: String,
    pub path: String,
    pub size: u64,
}

pub fn inbox_dir() -> PathBuf {
    settings::local_dir().join("inbox")
}

// ── Paths a real drop delivered ─────────────────────────────────────────────
//
// `ingest_file` is callable from the page, so on its own it would copy any file
// the user can read (a script injected into the page could pull in ~/.ssh).
// Only paths the OS just delivered through a real drop are accepted: WebView2's
// drop objects on Windows (webview_drop.rs), Tauri's drag-drop window event
// elsewhere (lib.rs). Each is good once, for a couple of minutes.

const DROP_VALID_FOR: Duration = Duration::from_secs(120);
const DROP_MAX_PENDING: usize = 64;

static DROPPED: std::sync::Mutex<Vec<(String, std::time::Instant)>> = std::sync::Mutex::new(Vec::new());

/// Records paths that came from a real drop.
pub fn allow_dropped<I: IntoIterator<Item = String>>(paths: I) {
    let mut list = DROPPED.lock().unwrap_or_else(|e| e.into_inner());
    let now = std::time::Instant::now();
    list.retain(|(_, at)| now.duration_since(*at) < DROP_VALID_FOR);
    for p in paths {
        list.push((p, now));
    }
    let excess = list.len().saturating_sub(DROP_MAX_PENDING);
    list.drain(..excess);
}

/// True (once) when `path` was delivered by a drop in the last couple of minutes.
fn take_dropped(path: &str) -> bool {
    let mut list = DROPPED.lock().unwrap_or_else(|e| e.into_inner());
    let now = std::time::Instant::now();
    list.retain(|(_, at)| now.duration_since(*at) < DROP_VALID_FOR);
    match list.iter().position(|(p, _)| p == path) {
        Some(i) => {
            list.remove(i);
            true
        }
        None => false,
    }
}

pub fn ingest(source: &str) -> Result<DroppedFile, String> {
    if !take_dropped(source) {
        return Err(crate::i18n::t("Only files dropped on the island can be added."));
    }
    let src = Path::new(source);
    let meta = std::fs::metadata(src)
        .map_err(|e| crate::i18n::tf("Cannot read {path}: {error}", &[("path", source), ("error", &e.to_string())]))?;
    if meta.is_dir() {
        return Err(crate::i18n::t("Folders can't be dropped yet."));
    }

    let dir = inbox_dir();
    crate::platform::ensure_private_dir(&settings::local_dir()).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let name = src
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "file".into());

    let mut dest = dir.join(&name);
    if dest.exists() {
        let stem = src.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let ext = src.extension().map(|s| format!(".{}", s.to_string_lossy())).unwrap_or_default();
        for i in 2..1000 {
            let candidate = dir.join(format!("{stem} ({i}){ext}"));
            if !candidate.exists() {
                dest = candidate;
                break;
            }
        }
    }

    std::fs::copy(src, &dest).map_err(|e| crate::i18n::tf("Cannot copy: {error}", &[("error", &e.to_string())]))?;
    // CopyFileEx carries the source's timestamps across, so a file last edited
    // three years ago would arrive already older than the sweep window and be
    // deleted on the spot. The inbox ages from when *we* copied it.
    if let Ok(file) = std::fs::File::options().write(true).open(&dest) {
        let _ = file.set_modified(SystemTime::now());
    }
    sweep(&dir);

    Ok(DroppedFile {
        name,
        path: dest.to_string_lossy().to_string(),
        size: meta.len(),
    })
}

// ── 개인용 개조: 채팅창에 붙여 넣은 이미지 ─────────────────────────────────
//
// 페이지가 부르는 명령이라 아무 바이트나 올 수 있다: 파일 머리 바이트로 이미지인지
// 확인하고, 크기를 제한하고, 이름은 우리가 정한다 (경로를 페이지에서 받지 않는다).

const MAX_PASTE: usize = 20 * 1024 * 1024;

fn image_ext(b: &[u8]) -> Option<&'static str> {
    if b.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("png")
    } else if b.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some("jpg")
    } else if b.starts_with(b"GIF8") {
        Some("gif")
    } else if b.len() > 12 && &b[..4] == b"RIFF" && &b[8..12] == b"WEBP" {
        Some("webp")
    } else {
        None
    }
}

pub fn save_pasted_image(bytes: &[u8]) -> Result<DroppedFile, String> {
    let ext = image_ext(bytes).ok_or("이미지만 붙여 넣을 수 있어요.")?;
    if bytes.len() > MAX_PASTE {
        return Err("이미지가 너무 커요 (최대 20MB).".into());
    }
    let dir = inbox_dir();
    crate::platform::ensure_private_dir(&settings::local_dir()).map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let stamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
    let dest = dir.join(format!("pasted-{stamp}.{ext}"));
    std::fs::write(&dest, bytes).map_err(|e| e.to_string())?;
    sweep(&dir);
    Ok(DroppedFile {
        name: format!("붙여 넣은 이미지.{ext}"),
        path: dest.to_string_lossy().to_string(),
        size: bytes.len() as u64,
    })
}

/// Drops anything copied here more than a week ago. `ingest` stamps every copy
/// with the time it landed, so this really is the age of the copy and not the
/// age of whatever the user happened to drag in.
fn sweep(dir: &Path) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let now = SystemTime::now();
    for entry in entries.flatten() {
        let Ok(meta) = entry.metadata() else { continue };
        let Ok(copied) = meta.modified() else { continue };
        if now.duration_since(copied).map(|age| age > KEEP_FOR).unwrap_or(false) {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_real_images_can_be_pasted() {
        assert_eq!(image_ext(b"\x89PNG\r\n\x1a\nrest"), Some("png"));
        assert_eq!(image_ext(&[0xFF, 0xD8, 0xFF, 0xE0]), Some("jpg"));
        assert_eq!(image_ext(b"RIFF\0\0\0\0WEBPVP8 "), Some("webp"));
        assert_eq!(image_ext(b"MZ\x90\0 an exe"), None);
        assert!(save_pasted_image(b"#!/bin/sh").is_err());
    }

    #[test]
    fn ingest_copies_and_never_overwrites() {
        let tmp = std::env::temp_dir().join(format!("coucou-test-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let source = tmp.join("note.txt");
        std::fs::write(&source, b"hello").unwrap();

        let drop = |p: &Path| allow_dropped([p.to_string_lossy().to_string()]);
        drop(&source);
        let first = ingest(source.to_str().unwrap()).unwrap();
        assert_eq!(first.name, "note.txt");
        assert_eq!(std::fs::read(&first.path).unwrap(), b"hello");

        // A second drop of the same name must not clobber the first copy.
        std::fs::write(&source, b"second").unwrap();
        drop(&source);
        let second = ingest(source.to_str().unwrap()).unwrap();
        assert_ne!(first.path, second.path);
        assert_eq!(std::fs::read(&first.path).unwrap(), b"hello");
        assert_eq!(std::fs::read(&second.path).unwrap(), b"second");

        // Folders are refused rather than silently ignored.
        drop(&tmp);
        assert!(ingest(tmp.to_str().unwrap()).is_err());

        // An ancient source must not arrive already older than the sweep window.
        let old_source = tmp.join("ancient.txt");
        std::fs::write(&old_source, b"old").unwrap();
        let long_ago = SystemTime::now() - KEEP_FOR - Duration::from_secs(60 * 60);
        std::fs::File::options()
            .write(true)
            .open(&old_source)
            .unwrap()
            .set_modified(long_ago)
            .unwrap();
        drop(&old_source);
        let aged = ingest(old_source.to_str().unwrap()).unwrap();
        assert!(
            Path::new(&aged.path).exists(),
            "a file copied just now was swept as if it were a week old"
        );
        let _ = std::fs::remove_file(&aged.path);

        let _ = std::fs::remove_file(&first.path);
        let _ = std::fs::remove_file(&second.path);
        let _ = std::fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod drop_tests {
    use super::*;

    // One test: the list is process-wide and tests run in parallel.
    #[test]
    fn only_a_dropped_path_is_ingested_once_and_the_list_stays_bounded() {
        let p = "/tmp/coucou-test-not-dropped.txt".to_string();
        assert!(ingest(&p).is_err(), "never dropped");
        allow_dropped([p.clone()]);
        assert!(take_dropped(&p));
        assert!(!take_dropped(&p), "good once");

        allow_dropped((0..200).map(|i| format!("/tmp/bounded-{i}")));
        assert!(DROPPED.lock().unwrap().len() <= DROP_MAX_PENDING);
        assert!(take_dropped("/tmp/bounded-199"));
        assert!(!take_dropped("/tmp/bounded-0"));
    }
}
