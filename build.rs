
use copy_to_output::copy_to_output;  
use {std::env,winres::WindowsResource};
fn main() {  
    // Re-runs script if any files in res are changed  
    println!("cargo:rerun-if-changed=res/*");  
    copy_to_output("itemfilter.yml", &env::var("PROFILE").unwrap()).expect("Could not copy");  
    copy_to_output("settings.toml", &env::var("PROFILE").unwrap()).expect("Could not copy");  
    copy_to_output("bin", &env::var("PROFILE").unwrap()).expect("Could not copy");  
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            .set_icon("primemh.ico")
            .compile()
            .unwrap();
    }
}