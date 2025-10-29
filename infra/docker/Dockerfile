# DEX-OS Development Environment

FROM rust:1.80

# Install system dependencies
RUN apt-get update && apt-get install -y \
    clang \
    cmake \
    protobuf-compiler \
    libprotobuf-dev \
    librocksdb-dev \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Android NDK for Android builds
ENV ANDROID_NDK_HOME=/opt/android-ndk
ENV ANDROID_NDK_VERSION=r25b
RUN cd /opt \
    && curl -O https://dl.google.com/android/repository/android-ndk-${ANDROID_NDK_VERSION}-linux.zip \
    && unzip android-ndk-${ANDROID_NDK_VERSION}-linux.zip \
    && mv android-ndk-${ANDROID_NDK_VERSION} ${ANDROID_NDK_HOME} \
    && rm android-ndk-${ANDROID_NDK_VERSION}-linux.zip

# Install Android tools
ENV ANDROID_HOME=/opt/android-sdk
RUN mkdir -p ${ANDROID_HOME} \
    && cd ${ANDROID_HOME} \
    && curl -O https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip \
    && unzip commandlinetools-linux-9477386_latest.zip \
    && rm commandlinetools-linux-9477386_latest.zip

# Set up environment variables
ENV PATH=$PATH:${ANDROID_NDK_HOME}:${ANDROID_HOME}/cmdline-tools/bin
ENV RUST_BACKTRACE=1

# Create working directory
WORKDIR /dexos

# Copy project files
COPY . .

# Build the project
RUN cargo build

# Expose ports
EXPOSE 8545 8546 5432 6379

# Default command
CMD ["cargo", "run"]