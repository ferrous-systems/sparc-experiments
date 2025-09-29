FROM rust:bookworm

# Update repo list and installed packages
RUN apt-get update -y && apt-get upgrade -y

# Install-time dependencies
RUN apt-get -y install curl xz-utils

# Install a SPARC C compiler
RUN curl -Lf https://download.gaisler.com/anonftp/bcc2/bin/bcc-2.3.1-llvm-sparc-linux64.tar.xz > /tmp/bcc-2.3.1-llvm-sparc-linux64.tar.xz && \
    echo "53c6945254ebace9517ce16b0e09ac94  /tmp/bcc-2.3.1-llvm-sparc-linux64.tar.xz" | md5sum -c && \
    tar xvf /tmp/bcc-2.3.1-llvm-sparc-linux64.tar.xz -C /opt && \
    rm -f /tmp/bcc-2.3.1-llvm-sparc-linux64.tar.xz

# Install a LEON3 simulator (SPARCV8)
RUN curl -Lf https://download.gaisler.com/products/TSIM3/bin/tsim-eval/tsim-eval-3.1.12.tar.gz > /tmp/tsim-eval-3.1.12.tar.gz && \
    echo "93152a34afe6abcc57f1333a6db885e6df00e0266156d9693f1fa930aef9ee29  /tmp/tsim-eval-3.1.12.tar.gz" | sha256sum -c && \
    tar xvf /tmp/tsim-eval-3.1.12.tar.gz -C /opt && \
    rm -f /tmp/tsim-eval-3.1.12.tar.gz

# Run-time dependencies
RUN apt-get -y install libnspr4 libtinfo5 make gcc

# Fetch nightly rustc (with bare-metal SPARC target support)
RUN rustup toolchain add nightly
RUN rustup component add rust-src --toolchain=nightly

RUN mkdir /work
WORKDIR /work

# Set system PATH
ENV PATH=$PATH:/bin:/opt/tsim-eval/tsim/linux-x64:/opt/sparc-bcc-2.3.1-llvm/bin
