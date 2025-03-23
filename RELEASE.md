# Release Process Guide

This guide explains the process of releasing a new version of s-screen, including GitHub tags and AUR package updates.

## 1. Update Version in Cargo.toml

First, update the version in `Cargo.toml`:

```toml
[package]
version = "0.1.3"  # Update this version
```

## 2. Create GitHub Release

1. Commit your changes:

   ```bash
   git add Cargo.toml
   git commit -m "chore: bump version to 0.1.3"
   ```

2. Create and push a new tag:

   ```bash
   git tag -a v0.1.3 -m "Release v0.1.3"
   git push origin v0.1.3
   ```

3. Go to GitHub repository and create a new release:
   - Visit https://github.com/kolarski/s/releases
   - Click "Create a new release"
   - Choose the tag you just created (v0.1.3)
   - Fill in the release notes
   - Publish the release

## 3. Update AUR Package

1. Clone the AUR repository (if not already cloned):

   ```bash
   git clone ssh://aur@aur.archlinux.org/s-screen.git
   cd s-screen
   ```

2. Download the new release and calculate SHA256:

   ```bash
   curl -L https://github.com/kolarski/s/archive/v0.1.3.tar.gz -o s-screen-0.1.3.tar.gz
   sha256sum s-screen-0.1.3.tar.gz
   ```

3. Update the PKGBUILD:

   - Update `pkgver` to "0.1.3"
   - Update `sha256sums` with the new hash
   - Update any other relevant information (dependencies, etc.)

4. Generate new .SRCINFO:

   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

5. Commit and push changes:

   ```bash
   git add PKGBUILD .SRCINFO
   git commit -m "chore: bump version to 0.1.3"
   git push
   ```

6. Verify the update:
   ```bash
   yay -S s-screen
   ```

## 4. Update Homebrew Package

1. Clone the Homebrew tap repository (if not already cloned):

   ```bash
   git clone https://github.com/kolarski/homebrew-s-screen.git
   cd homebrew-s-screen
   ```

2. Download the new release and calculate SHA256:

   ```bash
   curl -L https://github.com/kolarski/s/archive/v0.1.3.tar.gz -o s-screen-0.1.3.tar.gz
   sha256sum s-screen-0.1.3.tar.gz
   ```

3. Update the Formula file (`s-screen.rb`):

   - Update the version number
   - Update the SHA256 hash
   - Update any dependencies if needed

   Example:

   ```ruby
   class SScreen < Formula
     desc "Terminal Sessions Made Ridiculously Simple"
     homepage "https://kolarski.github.io/s/"
     url "https://github.com/kolarski/s/archive/v0.1.3.tar.gz"
     sha256 "your-new-sha256-hash"
     version "0.1.3"

     depends_on "rust" => :build
     depends_on "screen"

     def install
       system "cargo", "install", "--root", prefix, "--path", "."
     end
   end
   ```

4. Test the formula locally:

   ```bash
   brew install --build-from-source ./s-screen.rb
   ```

5. Commit and push changes:

   ```bash
   git add s-screen.rb
   git commit -m "chore: bump version to 0.1.3"
   git push
   ```

6. Verify the update:
   ```bash
   brew update
   brew upgrade s-screen
   s --version
   ```

## 5. Verify the Updates

1. Wait a few minutes for both AUR and Homebrew to process the changes

2. Test both package installations:

   ```bash
   # AUR
   yay -S s-screen

   # Homebrew
   brew upgrade s-screen
   ```

3. Verify the installation:
   ```bash
   s --version
   ```

## 6. Troubleshooting

If you encounter issues:

1. Check AUR package status:

   - Visit https://aur.archlinux.org/packages/s-screen
   - Verify the package is up to date
   - Check for any error messages

2. Verify package build:

   ```bash
   cd s-screen
   makepkg -s
   ```

3. Check package dependencies:
   ```bash
   makepkg -i
   ```

## 7. Common Issues

1. **AUR not showing updates**: Wait 5-10 minutes for AUR to process changes

2. **SHA256 mismatch**:

   - Double-check the downloaded tarball
   - Verify the version number in the URL
   - Make sure you're using the correct release tag

3. **Build failures**:
   - Check if all dependencies are listed in PKGBUILD
   - Verify the build process works locally
   - Check for any new dependencies in Cargo.toml

## 8. Best Practices

1. Always test the package locally before pushing to AUR
2. Keep a changelog of your changes
3. Update documentation if needed
4. Test the package on a clean system
5. Follow semantic versioning (MAJOR.MINOR.PATCH)

## 9. Useful Commands

```bash
# Update AUR database
yay -Syu

# Check package status
yay -Si s-screen

# Build package locally
makepkg -s

# Install package locally
makepkg -i

# Clean build files
makepkg -c
```
