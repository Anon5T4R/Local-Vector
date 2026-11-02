//! Help → community and project links (the community's Discord, this app's website, its GitHub
//! repository and where to report an issue). The commands return the URL; the frontend opens it.
//! The URLs follow the suite pattern (`github.com/<owner>/<app>`), as the other Local apps do.

use serde_json::{Value, json};

use super::{CommandSpec, Result, always, cmd};

/// The community's Discord: this repository's discussions, as in the other Local apps.
pub const DISCORD_URL: &str = "https://github.com/Anon5T4R/Local-Vector/discussions";
/// This app's website: its repository page.
pub const WEBSITE_URL: &str = "https://github.com/Anon5T4R/Local-Vector";
/// This app's source repository.
pub const GITHUB_URL: &str = "https://github.com/Anon5T4R/Local-Vector";
/// Where bugs and feature requests are reported.
pub const ISSUES_URL: &str = "https://github.com/Anon5T4R/Local-Vector/issues";

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.discord", "Join Our Discord", ["Help"], None, "{} → {url} the community Discord", always, |_, _| url(DISCORD_URL.into())),
        cmd!(query "help.website", "LocalVector Website", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(
            query "help.github",
            "LocalVector on GitHub",
            ["Help"],
            None,
            "{} → {url} source code, issues and releases",
            always,
            |_, _| url(GITHUB_URL.into())
        ),
        cmd!(
            query "help.reportIssue",
            "Report an Issue",
            ["Help"],
            None,
            "{} → {url} bugs and feature requests",
            always,
            |_, _| url(ISSUES_URL.into())
        ),
        cmd!(query "help.links", "Links", [], None, "{} → {discord, website, github, reportIssue}", always, |_, _| {
            Ok(json!({ "discord": DISCORD_URL, "website": WEBSITE_URL, "github": GITHUB_URL, "reportIssue": ISSUES_URL }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.discord", &json!({})).unwrap()["url"], "https://github.com/Anon5T4R/Local-Vector/discussions");
        assert_eq!(s.execute("help.website", &json!({})).unwrap()["url"], "https://github.com/Anon5T4R/Local-Vector");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], "https://github.com/Anon5T4R/Local-Vector");
        assert_eq!(s.execute("help.reportIssue", &json!({})).unwrap()["url"], "https://github.com/Anon5T4R/Local-Vector/issues");
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], "https://github.com/Anon5T4R/Local-Vector");
    }
}
