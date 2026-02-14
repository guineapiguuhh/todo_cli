use std::{
    env::current_dir,
    fs,
    io::{self},
};

use owo_colors::OwoColorize;

use clap::{Parser, Subcommand};
use todo_cli::{item::Item, save::Save};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Show,
    Check {
        #[arg()]
        index: usize,
    },
    New {
        #[arg()]
        name: String,

        #[arg(short, long)]
        desc: String,

        #[arg(short, long, default_value_t = false)]
        checked: bool,
    },
    DeleteAll,
    Delete {
        #[arg()]
        index: usize,
    },
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let mut file_path = current_dir()?;
    file_path.push("todo_save");
    file_path.set_extension("toml");

    let save = if !fs::exists(&file_path)? {
        let save = Save::new(file_path);
        save.flush()?;
        save
    } else {
        Save::load(&file_path)?
    };

    if let Some(command) = args.command {
        handle_command(save, command)?
    }
    Ok(())
}

fn handle_command(mut save: Save, command: Command) -> io::Result<()> {
    match command {
        Command::Show => {
            if save.items.is_empty() {
                println!("Unfortunately, you don't have any items.")
            }
            for (i, item) in save.items.iter().enumerate() {
                println!(
                    "{} {}\n",
                    format!("{}{}", i.to_string(), ".").bright_black(),
                    item
                )
            }
            Ok(())
        }
        Command::DeleteAll => {
            println!(
                "{} {}",
                "Items cleaned:".bright_black(),
                save.items.len().bright_black()
            );
            save.items.clear();
            save.flush()
        }
        Command::Delete { index } => {
            if index > 0 && index < save.items.len() {
                save.items.remove(index);
            } else {
                println!("{}", "Couldn't find this item.".bright_black())
            }
            save.flush()
        }
        Command::Check { index } => {
            if let Some(item) = save.items.get_mut(index) {
                item.checked = true;
            } else {
                println!("{}", "Couldn't find this item.".bright_black())
            }
            save.flush()
        }
        Command::New {
            name,
            desc,
            checked,
        } => {
            let item = Item {
                name,
                desc,
                checked,
            };
            println!("{} {} : {}", "Added".green().bold(), item.name, item.desc);
            save.items.push(item);
            save.flush()
        }
    }
}
