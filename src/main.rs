use std::process;

mod config;
mod model;

use config::Configuration;
use model::CreatedUser;
use octocrab::Octocrab;
use reqwest::Client;

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
        let user = &model::User {
            username: member.login.clone(),
            avatar: member.avatar_url.to_string(),
        };
        println!("user {:?}", user);
        let response = client
            .post(&endpoint)
            .header("Authorization", &config.issues_api_token)
            .json(user)
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                println!("Successfully posted data for user: {}", member.login);
            }
            Ok(resp) => match resp.text().await {
                Ok(msg) => {
                    if msg.contains("already exists") {
                        eprintln!("User already exists {}. Updating it", member.login);
                        match client
                            .get(endpoint.clone() + "/username/" + &member.login)
                            .header("Authorization", &config.issues_api_token)
                            .json(user)
                            .send()
                            .await
                        {
                            Ok(res) => {
                                let created_user: CreatedUser = res.json().await.expect("error");
                                let response = client
                                    .put(endpoint.clone() + "/" + &created_user.id.to_string())
                                    .header("Authorization", &config.issues_api_token)
                                    .json(user)
                                    .send()
                                    .await;
                                match response {
                                    Ok(resp) => {
                                        if resp.status().is_success() {
                                            println!(
                                                "Successfully updated data for user: {}",
                                                member.login
                                            );
                                        } else {
                                            println!("Error: {:?}", resp);
                                        }
                                    }
                                    Err(error) => {
                                        eprintln!(
                                            "Error updating data for member: {:?}. Error: {:#?}",
                                            member, error
                                        );
                                        process::exit(8);
                                    }
                                }
                            }
                            Err(error) => {
                                eprintln!(
                                    "Error updating data for member: {:?}. Error: {:#?}",
                                    member, error
                                );
                                process::exit(8);
                            }
                        }
                    } else {
                        eprintln!("Failed to post data for member {}: {}", member.login, msg);
                        process::exit(6);
                    }
                }
                Err(error) => {
                    eprintln!(
                        "Error reading response data for member: {:?}. Error: {:#?}",
                        member, error
                    );
                    process::exit(7);
                }
            },
            Err(error) => {
                eprintln!(
                    "Error posting data for member: {:?}. Error: {:#?}",
                    member, error
                );
                process::exit(8);
            }
        }
    }
}
