use std::fmt::Display;

use clap::{Parser, Subcommand, ValueEnum};

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "../templates/c/Makefile")]
struct CMakefileTemplate {
    project_name: String,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/c/CMakeLists.txt")]
struct CCMakeListsTemplate {
    project_name: String,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/cpp/Makefile")]
struct CppMakefileTemplate {
    project_name: String,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/cpp/CMakeLists.txt")]
struct CppCMakeListsTemplate {
    project_name: String,
}


const C_MAKEFILE_TEMPLATE: &str = include_str!("../templates/c/Makefile");
const C_CMAKE_TEMPLATE: &str = include_str!("../templates/c/CMakeLists.txt");

const CPP_MAKEFILE_TEMPLATE: &str = include_str!("../templates/cpp/Makefile");
const CPP_CMAKE_TEMPLATE: &str = include_str!("../templates/cpp/CMakeLists.txt");

#[derive(Parser, Debug)]
#[command(author, version, about = "Scaffold a project.", long_about = None)]
struct Cli {
    #[arg(short, long)]
    project_name: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    C {
        #[arg(value_enum, short, long, default_value_t=MakeType::MAKE)]
        make_type: MakeType,
    },

    #[command()]
    CPP {
        #[arg(value_enum, short, long, default_value_t=MakeType::CMAKE)]
        make_type: MakeType,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum MakeType {
    CMAKE,
    MAKE,
}

impl Display for MakeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

fn main() {
    let args = Cli::parse();

    println!("args: {:#?}", args);

    println!("Project Name: {}", args.project_name);
    println!("\n");

    match args.command {
        Commands::C { make_type } => {
            match make_type {
                MakeType::MAKE => {
                    println!("Makefile chosen.");
                    println!("Makefile:");
                    println!("----------------------------------------");

                    let ctx = CMakefileTemplate {
                        project_name: format!("{}", args.project_name),
                    };

                    println!("{}", ctx.render_once().unwrap());
                },
                MakeType::CMAKE => {
                    println!("CMakeLists.txt Chosen.");
                    println!("CMakeLists.txt:");
                    println!("----------------------------------------");

                    let ctx = CCMakeListsTemplate {
                        project_name: format!("{}", args.project_name),
                    };

                    println!("{}", ctx.render_once().unwrap());
                }
            }
        },
        Commands::CPP { make_type } => {
            match make_type {
                MakeType::MAKE => {
                    println!("Makefile chosen.");
                    println!("Makefile:");
                    println!("----------------------------------------");

                    let ctx = CppMakefileTemplate {
                        project_name: format!("{}", args.project_name),
                    };

                    println!("{}", ctx.render_once().unwrap());
                    print!("\n\n");
                },
                MakeType::CMAKE => {
                    println!("CMakeLists.txt Chosen.");
                    println!("CMakeLists.txt:");
                    println!("----------------------------------------");

                    let ctx = CppCMakeListsTemplate {
                        project_name: format!("{}", args.project_name),
                    };

                    println!("{}", ctx.render_once().unwrap());
                }
            }
        },
    }
}
