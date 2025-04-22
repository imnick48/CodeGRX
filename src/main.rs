use std::io;
mod fetch_code;
mod print_gx;
mod request_api;
mod utils;
const MAIN_PROMPT: &str = r#"You are a Software developer that responds in strict JSON format. Only include JSON. Your job is to suggest minimal, necessary code updates and types are insert and replace and if you want to delete a line replace that with space. Respond with this structure. also give a message in the end:
    {{
    "folder_paths": ["..."],    // new folders to create
    "file_paths": ["..."],      // new files to create
    "read_files": ["..."],      // files to read for context
    "file_changes": [           // precise diffs only
        {{
        "file_path": "./some/file.py",
        "changes": [
            {{ "line_no": 5, "type": "insert", "content": "new code here" }}
        ]
        }}
    ]
    "message": "Your message here"
    }}
    Current project structure:
    Also the file content of that are requested in previous prompt are in below "#;
static mut FULL_PROMPT: String = String::new();

#[tokio::main]
async fn main() {
    let mut api_key=String::new();
    let mut model_name=String::new();
    unsafe {
        FULL_PROMPT = MAIN_PROMPT.to_string();
    }
    println!("Enter Your OpenRouter API Key: ");
    io::stdin()
        .read_line(&mut api_key)
        .expect("Error in recording input");
    println!("Enter Model Name: ");
    io::stdin()
        .read_line(&mut model_name)
        .expect("Error in recording input");
    print_gx::car_gx();
    let mut inp = String::new();
    loop {
        println!(">> Enter your query: ");
        inp.clear();
        io::stdin()
            .read_line(&mut inp)
            .expect("Error in recording input");
        let prompt = unsafe { FULL_PROMPT.clone() };
        match request_api::request_api(&mut inp,&api_key.trim(),&prompt,&model_name).await {
            Ok(json_value) => {
                if let Some(paths) = json_value.as_array() {
                    unsafe {
                        FULL_PROMPT = MAIN_PROMPT.to_string();
                    }
                    for path in paths {
                        if let Some(p) = path.as_str() {
                            let content = utils::read_file(p);
                            unsafe {
                                let mut new_prompt = FULL_PROMPT.clone();
                                new_prompt.push_str(&format!("\npath:{}\n{}", p, content));
                                FULL_PROMPT = new_prompt;
                            }

                        }
                    }
                }
            },
            Err(e) => eprintln!("API request failed: {}", e),
        }
    }
}