use vecli::*;

fn main() {
    App::new("rzkfold")
        .name("rzkfold")
        .description("A project scaffolding CLI")
        .version("0.1.0")
        .print_help_if_no_args(true)
        .run();
}	
