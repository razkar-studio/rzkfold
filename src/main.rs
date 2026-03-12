
use vecli::*;

fn new_project(ctx: &CommandContext) {
    let name = ctx.positionals.first()
    	.map(String::as_str).unwrap_or("my-project");

    if std::path::Path::new(name).exists() {
        println!("> A directory with the name '{}' already exists.", name);
        if !Confirm::new("> Overwrite?").ask() {
            println!("! Aborted");
            return;
        }
    }

    let preset = Choice::new("> Pick a preset", &["rust", "python", "web"])
        .ask();

    match preset.as_str() {
        "rust" => {
        	let subpreset = Choice::new(
        		"> Pick an optional preset for Rust",
        		&["lib", "bin", "both"]
        	).default("bin").ask();
        	match subpreset.as_str() {
        		"lib" => scaffold_rust(name, false, true),
        		"bin" => scaffold_rust(name, true, false),
        		"both" => scaffold_rust(name, true, true),
        		_ => unreachable!(),
        	}
        },
        "python" => {
        	let create_pyproject = Confirm::new(
        		"> Create pyproject.toml?"
        	).default(true).ask();
        	
        	scaffold_py(name, create_pyproject);
        },
        "web" => scaffold_web(name),
        _ => println!("Preset not implemented yet."),
    }
}

fn scaffold_web(name: &str) {
    std::fs::create_dir_all(name).unwrap();
    std::fs::write(format!("{}/index.html", name), format!(
"<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>{}</title>
    <link rel=\"stylesheet\" href=\"style.css\">
</head>
<body>
    <h1>Hello, world!</h1>
    <script src=\"script.js\"></script>
</body>
</html>", name)).unwrap();

    std::fs::write(format!("{}/style.css", name),
"* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: sans-serif;
}
").unwrap();

    std::fs::write(format!("{}/script.js", name),
"console.log(\"Hello, world!\");
").unwrap();

    std::fs::write(format!("{}/.gitignore", name),
"node_modules/
").unwrap();

    std::process::Command::new("git")
        .arg("init")
        .arg(name)
        .arg("--quiet")
        .status()
        .unwrap();

    println!("Created web project: {}", name);
}

fn scaffold_py(name: &str, toml: bool) {
	std::fs::create_dir_all(name).unwrap();
	std::fs::write(
		format!("{}/main.py", name),
		"def main():\n    print(\"Hello, world!\")\n\nif __name__ == \"__main__\":\n    main()"
	).unwrap();
	std::fs::write(
		format!("{}/.gitignore", name),
		"__pycache__/\n*.pyc"
	).unwrap();

	if toml {
		std::fs::write(format!("{}/pyproject.toml", name), format!("[project]\nname = \"{}\"\nversion = \"0.1.0\"\n", name)).unwrap();
	}

	std::process::Command::new("git")
		.arg("init")
		.arg(name)
		.arg("--quiet")
		.status()
		.unwrap();

	println!("Created Python project: {}", name);
}

fn scaffold_rust(name: &str, bin: bool, lib: bool,) {
    std::fs::create_dir_all(format!("{}/src", name)).unwrap();
	if bin {
    	std::fs::write(format!("{}/src/main.rs", name), "fn main() {\n    println!(\"Hello, world!\");\n}\n").unwrap();
	}
	if lib {
		std::fs::write(format!("{}/src/lib.rs", name), "pub fn hello() {\n    println!(\"Hello, world!\")\n}\n").unwrap();
	}
    std::fs::write(format!("{}/Cargo.toml", name), format!(
"[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", name
    )).unwrap();
    std::fs::write(format!("{}/.gitignore", name), "/target\n").unwrap();

	std::process::Command::new("git")
		.arg("init")
		.arg(name)
		.arg("--quiet")
		.status()
		.unwrap();

    println!("Created Rust project: {}", name);
}

fn main() {
    App::new("rzkfold")
        .name("rzkfold")
        .description("A project scaffolding CLI")
        .version("0.1.0")
        .print_help_if_no_args(true)
        .add_command(
            Command::new("new", new_project)
                .description("Create a new project")
                .usage("<name>")
        )
        .run();
}
