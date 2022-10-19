fn main() {
    use cmake::Config;

    cxx_build::bridge("src/lib.rs");
    let dst = Config::new("orb_slam3")
                    .build_target("orb_slam3")
                    .build();

    println!("cargo:rustc-link-search=native=orb_slam3/lib");
    println!("cargo:rustc-link-lib=static=orb_slam3");
}
