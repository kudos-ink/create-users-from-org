use std::{clone, process};

mod config;
mod model;

use config::Configuration;
use octocrab::Octocrab;
use reqwest::Client;
use tokio::time::{self, sleep};

#[tokio::main]
async fn main() {
    let config = match envy::from_env::<Configuration>() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error loading configuration: {:#?}", error);
            process::exit(1);
        }
    };

    // get all the users
    let github_client = match Octocrab::builder()
        .personal_token(config.github_token.clone())
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            eprintln!("Error building Octocrab client: {:#?}", error);
            process::exit(2);
        }
    };

    let mut all_members = Vec::new();

    // Start with the first page
    let mut page = 1_u32;
    let mut results = github_client
        .orgs(&config.github_organization)
        .list_members()
        .per_page(100)
        .page(page)
        .send()
        .await;

    while let Ok(ref result) = results {
        if !result.items.is_empty() {
            all_members.extend(result.items.clone());

            page += 1_u32;

            results = github_client
                .orgs(&config.github_organization)
                .list_members()
                .per_page(100)
                .page(page)
                .send()
                .await;
            let _ = sleep(time::Duration::from_secs(1));
        } else {
            break;
        }
    }

    if results.is_err() {
        eprintln!(
            "Error getting members from GitHub: {:#?}",
            results.unwrap_err()
        );
        process::exit(3);
    }

    println!("Fetched {} members", all_members.len());
    if all_members.is_empty() {
        eprintln!("The organization has no members");
        process::exit(4);
    }
    let client = Client::new();
    let endpoint = config.issues_api + "/users";
    for member in all_members {
        println!("Creating member {}", member.login);

        let response = client
            .post(&endpoint)
            .header("Authorization", &config.issues_api_token)
            .json(&model::User {
                username: member.login.clone(),
            })
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                println!("Successfully posted data for user: {}", member.login);
            }
            Ok(resp) => match resp.text().await {
                Ok(msg) => {
                    if msg.contains("already exists") {
                        eprintln!("User already exists {}", member.login);
                    } else {
                        eprintln!("Failed to post data for member {}: {}", member.login, msg);
                    }
                }
                Err(error) => {
                    eprintln!(
                        "Error reading response data for member: {:?}. Error: {:#?}",
                        member, error
                    );
                    process::exit(6);
                }
            },
            Err(error) => {
                eprintln!(
                    "Error posting data for member: {:?}. Error: {:#?}",
                    member, error
                );
                process::exit(7);
            }
        }
    }
}
