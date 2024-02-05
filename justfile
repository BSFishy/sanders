set dotenv-load := true

debug := 'true'

# the architecture to build for
arch := arch()

# the name of the rust target to build
target := "sanders-" + arch

native_target := `rustc -vV | sed -n 's|host: ||p'`

#native_target := arch + '-' + if os() == 'linux' { 'unknown' } else if os() == 'windows' { '' } else { '' }

# the target file
target_file := join(justfile_directory(), target + '.json')

cargo_debug_arg := if debug == 'true' { '' } else { '--release' }
cargo_target_dir := join(justfile_directory(), 'target')
cargo_manifest_path := join(justfile_directory(), 'Cargo.toml')

cargo_args := cargo_debug_arg + ' --target ' + target + ' --target-dir ' + cargo_target_dir + ' --manifest-path ' + cargo_manifest_path
sconfig_cargo_args := cargo_debug_arg + ' --target ' + native_target + ' --target-dir ' + cargo_target_dir + ' --manifest-path ' + join(justfile_directory(), 'tools', 'sconfig', 'Cargo.toml')

export RUST_TARGET_PATH := justfile_directory()

# TODO(BSFishy): document this
default: bootimage

# TODO(BSFishy): document this
@build:
	cargo build {{cargo_args}}

# TODO(BSFishy): document this
@bootimage:
	cargo bootimage {{cargo_args}}

# TODO(BSFishy): document this
@clean:
	cargo clean {{cargo_args}}

# TODO(BSFishy): document this
@run:
	cargo run {{cargo_args}}
