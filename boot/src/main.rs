use bootloader::BootConfig;

pub fn main() {
    let mut config = BootConfig::default();
    config.frame_buffer.minimum_framebuffer_height = Some(720);

    let kernel = include_bytes!(env!("CARGO_BIN_FILE_AURA_KERNEL"));
    let mut binding = bootloader::UefiBoot::new(kernel);
    let uefi_builder = binding.set_boot_config(&config);

    let out_path = std::env::args().nth(1).expect("output path argument");
    uefi_builder.create_disk_image(out_path.as_ref()).unwrap();
    println!("Created bootable disk image at {}", out_path);
}
