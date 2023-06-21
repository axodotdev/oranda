use std::collections::HashMap;

use axohtml::elements::{div, tr};
use axohtml::{html, text};

use crate::config::Config;
use crate::data::artifacts::inference::triple_to_display_name;
use crate::data::artifacts::InstallMethod;
use crate::data::Release;
use crate::errors::*;

/// Build a downloads table for this release
pub fn build(release: &Release, _config: &Config) -> Result<Box<div<String>>> {
    let mut table = vec![];

    // We only display files that were detected to be a downloadable archive
    // this kinda messy code is just gathering those up, deduplicating, and sorting
    let mut files = HashMap::new();
    for (_, installer) in release.artifacts.installers() {
        let InstallMethod::Download { file } = installer.method else {
            continue;
        };
        files.insert(
            file,
            (
                release.artifacts.file(file),
                installer
                    .targets
                    .keys()
                    .map(|s| triple_to_display_name(s))
                    .collect::<Vec<_>>(),
            ),
        );
    }
    let mut files: Vec<_> = files.into_iter().collect();
    files.sort_by_key(|(_, (f, _))| &f.name);

    if files.is_empty() {
        return Ok(html!(<div><h3>{text!("No Downloads")}</h3></div>));
    }

    // If any files have checksums, add a column for that
    let has_checksum_files = files.iter().any(|(_, (f, _))| f.checksum_file.is_some());

    // Add the headings
    {
        let mut row = vec![];
        row.push(html!(<th>{text!("File")}</th>));
        row.push(html!(<th>{text!("Platform")}</th>));
        if has_checksum_files {
            row.push(html!(<th>{text!("Checksum")}</th>));
        }
        table.push(html!(<tr>{row}</tr>));
    }

    // Now add the rows
    for (_, (file, platforms)) in files {
        let mut row = vec![];

        // Link the file
        let url = &file.download_url;
        let name = &file.name;
        row.push(html!(<td><a href=url>{text!(name)}</a></td>));

        // List platforms
        let mut platform_list = String::new();
        let mut multi_platform = false;
        for platform in platforms {
            let Some(platform) = platform else {
                continue;
            };
            if multi_platform {
                platform_list.push_str(", ");
            }
            platform_list.push_str(platform);
            multi_platform = true;
        }
        row.push(html!(<td>{text!(platform_list)}</td>));

        // Optionally include checksums
        if has_checksum_files {
            let checksum_entry = if let Some(checksum) = file.checksum_file {
                let checksum_url = &release.artifacts.file(checksum).download_url;
                html!(<td><a href=checksum_url>{text!("checksum")}</a></td>)
            } else {
                html!(<td></td>)
            };
            row.push(checksum_entry);
        }
        table.push(html!(<tr>{row}</tr>));
    }

    Ok(html(table))
}

// False positive duplicate allocation warning
// https://github.com/rust-lang/rust-clippy/issues?q=is%3Aissue+redundant_allocation+sort%3Aupdated-desc
#[allow(clippy::vec_box)]
fn html(table: Vec<Box<tr<String>>>) -> Box<div<String>> {
    html!(
    <div>
        <h3>{text!("Downloads")}</h3>
        <table>
            {table}
        </table>
    </div>
    )
}
