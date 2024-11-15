use crate::cli::check_upgrade;
use crate::cli::version_client::MapVersionClient;
use crate::err::Error;
use std::collections::BTreeMap;

#[test_log::test(tokio::test)]
pub async fn test_version_upgrade() {
	let mut client = MapVersionClient {
		fetch_mock: BTreeMap::new(),
	};
	client.fetch_mock.insert(
		"latest".to_string(),
		Box::new(|| -> Result<String, Error> { Ok("1.0.0".to_string()) }),
	);

	let test_cases = vec![("1.0.0", true), ("0.9.0", false), ("1.1.0", true)];

	for (version, should_succeed) in test_cases {
		let result = check_upgrade(&client, version).await;
		if should_succeed {
			assert!(result.is_ok(), "Test case for version {} failed", version);
		} else {
			assert!(result.is_err(), "Test case for version {} failed", version);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::CommandFactory;

	#[test]
	fn test_no_online_version_check_flag() {
		let cli =
			Cli::command().try_get_matches_from(vec!["test", "--no-online-version-check"]).unwrap();

		let cli: Cli = Cli::from_arg_matches(&cli).unwrap();
		assert!(
			!cli.online_version_check,
			"The online_version_check should be false when --no-online-version-check is passed"
		);
	}
}
