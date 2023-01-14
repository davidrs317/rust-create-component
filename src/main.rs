fn main() {
    let mut component_name = String::new();
    
    println!("Please enter the name of your component");
    match read_input(&mut component_name) {
        Ok(_) => {
            println!("starting creation of {}", component_name);
        },
        Err(err) => {
            panic!("{}", err);
        }
    };

    let mut component_name = component_name.trim().to_string();

    match create_dir(&component_name) {
        Ok(_) => {},
        Err(_) => {
            panic!("could not create component directory");
        }
    }

    let tsx_file_name = format!("{}.tsx", component_name);
    let css_file_name = format!("{}.css", component_name);
    let file_vec = vec![&tsx_file_name, &css_file_name];

    match create_files(&file_vec) {
        Ok(_) => {},
        Err(_) => {
            panic!("failed to create component files");
        },
    };

    match write_to_tsx(&tsx_file_name, &mut component_name) {
        Ok(_) => {},
        Err(_) => {
            panic!("failed to write to tsx file");
        }
    }
}

fn read_input(component_name: &mut String) -> Result<i32, i32> {
    use std::io::stdin;
    match stdin().read_line(component_name) {
        Ok(_) => {},
        Err(_) => {
            return Err(0);
        }
    };
    Ok(1)
}

fn create_file(file_name: &String) -> Result<i32, i32> {
    use std::fs::File;
    match File::create(file_name) {
        Ok(_) => {},
        Err(_) => {
            return Err(0);
        }
    };

    Ok(1)
}

fn create_files(file_vec: &Vec<&String>) -> Result<i32, i32> {
    for file in file_vec {
        match create_file(file) {
            Ok(_) => {
                println!("created {}", file);
            },
            Err(_) => {
               return Err(0); 
            },
        };
    }

    Ok(1)
}

fn write_to_tsx(tsx_file_name: &String, component_name: &mut String) -> Result<i32, i32> {
    use std::fs::write;
    let file_string = format!("import React from 'react'; \n \ninterface {}Props {{ \n \n}} \n \nconst {} = (props: {}Props) => {{ \n \n}}", component_name, component_name, component_name);

    match write(&tsx_file_name, &file_string) {
        Ok(_) => {
            println!("wrote react code to tsx file");
        },
        Err(_) => {
            return Err(0);
        }
    };

    Ok(1)
}

fn create_dir(component_name: &mut String) -> Result<i32, i32> {
    use std::fs;
    match fs::create_dir(component_name) {
        Ok(_) => {
            println!("created {} directory", component_name);
        },
        Err(_) => {
            return Err(0);
        },
    };

    Ok(1);
}
