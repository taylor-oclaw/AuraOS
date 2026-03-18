use std::path::PathBuf;

fn main() {
    let kernel_path = PathBuf::from(std::env::var("KERNEL_BIN").unwrap_or_else(|_| {
        // Default: look for the kernel binary in the kernel target dir
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let kernel_binary = manifest_dir
            .parent()
            .unwrap()
            .join("kernel/target/x86_64-unknown-none/debug/aura-kernel");
        kernel_binary.to_string_lossy().to_string()
    }));

    if !kernel_path.exists() {
        eprintln!("Kernel binary not found at: {}", kernel_path.display());
        eprintln!("Build the kernel first: cd kernel && cargo +nightly build");
        std::process::exit(1);
    }

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string()));

    // Create UEFI disk image
    let uefi_path = out_dir.join("aura-os-uefi.img");
    bootloader::UefiBoot::new(&kernel_path)
        .create_disk_image(&uefi_path)
        .expect("Failed to create UEFI disk image");
    println!("Created UEFI image: {}", uefi_path.display());

    // Create BIOS disk image
    let bios_path = out_dir.join("aura-os-bios.img");
    bootloader::BiosBoot::new(&kernel_path)
        .create_disk_image(&bios_path)
        .expect("Failed to create BIOS disk image");
    println!("Created BIOS image: {}", bios_path.display());
}
