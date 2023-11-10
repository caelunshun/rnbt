# Print the title at the screen
Write-Host "=========================="
Write-Host "Building Rust Library"
Write-Host "=========================="

# Check for command line input
if ($args.Length -eq 0) {
    Write-Host "No build mode specified. 'debug' will be used by default." -ForegroundColor Yellow
    $buildMode = "debug"
}
else {
    # Assign the build mode from command line input
    $buildMode = $args[0].ToLower()
}

# Run cargo command based on the input
switch ($buildMode) {
    "debug" {
        Write-Host "Running cargo build (debug mode)..."
        cargo build
    }
    "release" {
        Write-Host "Running cargo build (release mode)..."
        cargo build --release
    }
    "clean" {
        Write-Host "Cleaning project..."
        cargo clean
    }
    default {
        Write-Host "Invalid input. Use 'debug', 'release', or 'clean'." -ForegroundColor Red
        exit
    }
}

Write-Host "=========================="
# Check if the build was successful
if ($LASTEXITCODE -eq 0) {
    Write-Host "Build completed successfully" -ForegroundColor Green
    Write-Host "=========================="
    Write-Host "Run integration tests"
    cargo test
} else {
    Write-Host "Build failed" -ForegroundColor Red
}
Write-Host "=========================="

