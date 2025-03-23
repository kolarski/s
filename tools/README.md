# Development Tools

This directory contains various development and testing tools for the project.

## PKGBUILD Testing

The `test_pkgbuild.sh` script helps you test the PKGBUILD in a clean Arch Linux environment using Docker. This ensures that your package builds correctly and all dependencies are properly specified.

### Prerequisites

- Docker installed on your system
- Bash shell

### Usage

1. Make the script executable:

   ```bash
   chmod +x test_pkgbuild.sh
   ```

2. Run the test script from the `tools` directory:
   ```bash
   ./test_pkgbuild.sh
   ```

The script will:

1. Start a clean Arch Linux container
2. Install all required dependencies
3. Build the package using `makepkg`
4. Clean up the container after the test

### What it Tests

- Package dependencies
- Build process
- Package creation
- File installation

### Troubleshooting

If the build fails, check:

1. All dependencies are listed in the PKGBUILD
2. The source URL is correct
3. The SHA256 sum matches your source file
4. The build process completes successfully

## AUR Installation Testing

The `test_aur_install.sh` script helps you test the installation of your package from the AUR using an AUR helper (yay by default). This ensures that your package can be installed correctly by users through the AUR.

### Prerequisites

- Docker installed on your system
- Bash shell
- Your package must be submitted to the AUR

### Usage

1. Make the script executable:

   ```bash
   chmod +x test_aur_install.sh
   ```

2. Run the test script from the `tools` directory:
   ```bash
   ./test_aur_install.sh
   ```

The script will:

1. Start a clean Arch Linux container
2. Install yay (AUR helper)
3. Attempt to install your package from the AUR
4. Verify the installation by checking if the binary is available
5. Clean up the container after the test

### Configuration

You can modify the following variables in the script:

- `PACKAGE_NAME`: The name of your package in the AUR
- `AUR_HELPER`: The AUR helper to use (default: yay)

### What it Tests

- AUR package installation
- Binary availability
- Package version reporting
- Dependencies resolution

### Troubleshooting

If the installation fails, check:

1. Your package is properly submitted to the AUR
2. All dependencies are correctly specified in the PKGBUILD
3. The package name matches exactly in the AUR
4. The AUR helper can access the AUR repository
