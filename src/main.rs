use std::fmt::Display;

use clap::{Parser, Subcommand, ValueEnum};

use sailfish::TemplateOnce;

// Templates

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


// Commands and Arguments

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

fn create_project<T: TemplateOnce>(ctx: T, path: String) {
    std::fs::write(path, ctx.render_once().unwrap()).unwrap();
}

fn main() {
    let args = Cli::parse();

    let dir = format!("./{}", args.project_name);

    // TODO: Handle error
    std::fs::create_dir_all(dir.clone()).unwrap();

    match args.command {
        Commands::C { make_type } => {
            match make_type {
                MakeType::MAKE => {
                    let ctx = CMakefileTemplate {
                        project_name: format!("{}", args.project_name),
                    };

                    let path = format!("{}/Makefile", dir.clone());

                    create_project(ctx, path);
                },
                MakeType::CMAKE => {
                    let ctx = CCMakeListsTemplate {
                        project_name: format!("{}", args.project_name),
                    };

                    let path = format!("{}/CMakeLists.txt", dir.clone());

                    create_project(ctx, path);
                }
            }
        },
        Commands::CPP { make_type } => {
            match make_type {
                MakeType::MAKE => {
                    let ctx = CppMakefileTemplate {
                        project_name: format!("{}", args.project_name),
                    };

                    let path = format!("{}/Makefile", dir.clone());

                    create_project(ctx, path);
                },
                MakeType::CMAKE => {
                    let ctx = CppCMakeListsTemplate {
                        project_name: format!("{}", args.project_name),
                    };

                    let path = format!("{}/CMakeLists.txt", dir.clone());

                    create_project(ctx, path);
                }
            }
        },
    }
}
