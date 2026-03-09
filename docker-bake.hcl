variable "VERSION" {
    default = "test"
}

variable "PLATFORMS" {
    # Drop 32-bit support
    # Work around buildx quirks
    default = [
        # "linux/386",
        "linux/amd64",
        # "linux/arm/v6",
        # "linux/arm/v7",
        "linux/arm64/v8",
        # "linux/ppc64le",
        # "linux/riscv64",
        # "linux/s390x",
    ]
}

variable "PRODUCTION" {
    default = [
        "chandler",
    ]
}

variable "TEST" {
    default = [
        "test-chandler",
    ]
}

group "production" {
    targets = PRODUCTION
}

group "test" {
    targets = TEST
}

group "all" {
    targets = concat(PRODUCTION, TEST)
}

target "chandler" {
    platforms = PLATFORMS
    tags = [
        "n4jm4/chandler:${VERSION}",
        "n4jm4/chandler",
    ]
}

target "test-chandler" {
    platforms = PLATFORMS
    tags = [
        "n4jm4/chandler:test",
    ]
}
