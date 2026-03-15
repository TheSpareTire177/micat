use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use libium::config::{self, structs::{Config, ModLoader, Profile}};

/// MICAT (Minecraft Instance Control and Automation Toolkit) CLI
#[derive(Parser, Debug)]
#[command(name = "micat", about = "MICAT – Minecraft Instance Control and Automation Toolkit on top of libium")]
struct MicatCli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialise or overwrite Micat's local config with two profiles
    Init {
        /// Permanent server data directory (bound into mc-permanent:/data)
        #[arg(long, default_value = "../data/mc-permanent")]
        permanent_dir: PathBuf,

        /// Seasonal server data directory (bound into mc-seasonal:/data)
        #[arg(long, default_value = "../data/mc-seasonal")]
        seasonal_dir: PathBuf,

        /// Minecraft game version to target (e.g. 1.20.1)
        #[arg(long, default_value = "1.20.1")]
        game_version: String,

        /// Mod loader to target (Forge/NeoForge/Fabric/Quilt)
        #[arg(long, value_enum, default_value_t = LoaderChoice::Forge)]
        loader: LoaderChoice,
    },
}

/// Wrapper so we don't expose libium's ModLoader directly on the CLI
#[derive(Copy, Clone, Debug, ValueEnum)]
enum LoaderChoice {
    Quilt,
    Fabric,
    Forge,
    Neoforge,
}

impl From<LoaderChoice> for ModLoader {
    fn from(from: LoaderChoice) -> Self {
        match from {
            LoaderChoice::Quilt => ModLoader::Quilt,
            LoaderChoice::Fabric => ModLoader::Fabric,
            LoaderChoice::Forge => ModLoader::Forge,
            LoaderChoice::Neoforge => ModLoader::NeoForge,
        }
    }
}

/// Micat entrypoint.
fn main() {
    let cli = MicatCli::parse();

    match cli.command {
        Command::Init {
            permanent_dir,
            seasonal_dir,
            game_version,
            loader,
        } => {
            if let Err(err) = init_profiles(permanent_dir, seasonal_dir, game_version, loader) {
                eprintln!("Micat init failed: {err}");
                std::process::exit(1);
            }
        }
    }
}

fn init_profiles(
    permanent_dir: PathBuf,
    seasonal_dir: PathBuf,
    game_version: String,
    loader_choice: LoaderChoice,
) -> std::io::Result<()> {
    let loader: ModLoader = loader_choice.into();

    // Local Micat-specific config living next to the binary workspace
    let config_path = PathBuf::from("micat-config.json");

    let mut config = Config::default();

    let permanent_profile = Profile::new(
        "Permanent Server".to_string(),
        permanent_dir,
        vec![game_version.clone()],
        loader.clone(),
    );

    let seasonal_profile = Profile::new(
        "Seasonal Server".to_string(),
        seasonal_dir,
        vec![game_version],
        loader,
    );

    config.profiles = vec![permanent_profile, seasonal_profile];
    // active_profile is 1-based in ferium/libium configs
    config.active_profile = 1;

    config::write_config(&config_path, &config)?;

    println!("Micat config initialised at {}", config_path.display());
    for (idx, profile) in config.profiles.iter().enumerate() {
        println!(
            "  [{}] {} -> {}",
            idx + 1,
            profile.name,
            profile.output_dir.display()
        );
    }

    Ok(())
}