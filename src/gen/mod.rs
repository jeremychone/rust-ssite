use crate::consts::FRAME;
use crate::site::Site;
use crate::Error;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::fs::{read_dir, remove_dir, remove_file};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;
use walkdir::WalkDir;

use self::processor::FileProcessor;

mod processor;

pub async fn gen(site: &Site, watch: bool) -> Result<(), Error> {
	// copy the content to site
	let dst_set = copy_content_to_site(&site).await?;

	// clean site dir (with thing that are not coming from content)
	clean_site_dir(&site, &dst_set).await?;

	if watch {
		watch_src_dir(&site).await?;
	}

	Ok(())
}

async fn copy_content_to_site(site: &Site) -> Result<HashSet<PathBuf>, Error> {
	// the dst file set
	let mut dst_set: HashSet<PathBuf> = HashSet::new();

	// copy and process the content files to _site/ dir
	for entry in site.content_entries() {
		if let Some(file_processor) = FileProcessor::from_src_file(site, entry.path().to_owned()) {
			if let Ok(Some(dst_file)) = file_processor.process(site) {
				dst_set.insert(dst_file);
			}
		}
	}

	Ok(dst_set)
}

async fn clean_site_dir(site: &Site, dst_set: &HashSet<PathBuf>) -> Result<(), Error> {
	let site_dir = site.dist_dir();

	for entry in WalkDir::new(&site_dir)
		.into_iter()
		.filter_map(|e| e.ok().filter(|f| f.path().is_file()))
	{
		let dst_file = entry.path();
		if !dst_set.contains(dst_file) {
			safer_remove_file_and_empty_parent(dst_file)?;
		}
	}

	Ok(())
}

async fn watch_src_dir(site: &Site) -> Result<(), Error> {
	let content_dir = site.content_dir();

	// Create a channel to receive the events.
	let (tx, rx) = channel();

	let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
	watcher.watch(content_dir, RecursiveMode::Recursive)?;

	// loop on rx
	loop {
		match rx.recv() {
			Ok(event) => match event {
				DebouncedEvent::NoticeWrite(src_file) => handle_src_file_event(site, src_file).await?,
				DebouncedEvent::NoticeRemove(src_file) => handle_src_file_event(site, src_file).await?,
				DebouncedEvent::Create(src_file) => handle_src_file_event(site, src_file).await?,
				// For now, set this to void, because, it comes after NoticeWrite and duplicate
				// Needs to see if we miss some events
				DebouncedEvent::Write(_src_file) => (),
				DebouncedEvent::Chmod(src_file) => handle_src_file_event(site, src_file).await?,
				DebouncedEvent::Remove(src_file) => handle_src_file_event(site, src_file).await?,
				DebouncedEvent::Rename(_, _) => (),
				DebouncedEvent::Rescan => (),
				DebouncedEvent::Error(_, _) => (),
			},
			Err(e) => println!("watch error: {:?}", e),
		}
	}
}

/// > Note: Unfortunately the Notify/FileSystem events are not really reliable, sometime get NotifyRemove or Remove when move, and no rename or even create.
///         So, we have to deal with this by looking if the source file exists or not and do the appropriate acction
async fn handle_src_file_event(site: &Site, src_file: PathBuf) -> Result<(), Error> {
	// guard - do nothing if src_file belong to dist_dir
	if src_file.starts_with(site.dist_dir()) {
		return Ok(());
	}

	// if frame change, then, udpate all sub files
	if src_file.ends_with(FRAME) {
		if let Some(dir) = src_file.parent() {
			for entry in WalkDir::new(&dir)
				.into_iter()
				.filter_map(|e| e.ok().filter(|f| f.path().is_file()))
			{
				let src_file = entry.path();
				if let Some(processor) = FileProcessor::from_src_file(site, src_file.to_path_buf()) {
					if processor.is_for_html_render() {
						// TODO: Handle error
						let _ = processor.process(site);
					}
				}
			}
		}
	}
	// otherwise, single file processing
	else if let Some(file_processor) = FileProcessor::from_src_file(site, src_file) {
		// TODO: Handle error
		let _ = file_processor.process(site);
	}

	Ok(())
}

// region:    Module Utils

fn safer_remove_file_and_empty_parent(file: &Path) -> Result<(), Error> {
	if file.exists() {
		println!("--- Removing {}", file.display());
		remove_file(file)?;
		// if parent dir exist and empty, remove too
		if let Some(parent) = file.parent() {
			if parent.exists() {
				let children = read_dir(parent)?;
				if children.count() == 0 {
					println!("--- Removing parent dir {}", parent.display());
					remove_dir(parent)?;
				}
			}
		}
	} else {
		println!("--- Removing SKIPPING NOT EXISTS {}", file.display());
	}
	Ok(())
}

// endregion: Module Utils
