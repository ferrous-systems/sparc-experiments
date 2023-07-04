FROM rust:bookworm

# Update repo list and installed packages
RUN apt-get update -y && apt-get upgrade -y

# Install-time dependencies
RUN apt-get -y install curl xz-utils

# Install a SPARC C compiler
RUN curl --proto '=https' -tlsv1.2 -LsSf https://www.gaisler.com/anonftp/bcc2/bin/bcc-2.2.3-llvm-linux64.tar.xz > /tmp/bcc-2.2.3-llvm-linux64.tar.xz && \
    echo "3beea1cc6a74fb0b09fc9533650df6b8  /tmp/bcc-2.2.3-llvm-linux64.tar.xz" | md5sum -c && \
    tar xvf /tmp/bcc-2.2.3-llvm-linux64.tar.xz -C /opt && \
    rm -f /tmp/bcc-2.2.3-llvm-linux64.tar.xz

# Install a LEON3 simulator (SPARCV8)
RUN curl --proto '=https' -tlsv1.2 -LsSf https://www.gaisler.com/tsim3/tsim-eval/tsim-eval-3.1.9.tar.gz > /tmp/tsim-eval-3.1.9.tar.gz && \
    echo "6d9a65c6dcb44ea025963bd6aa9156ae025d459240b453afbd3086654d6153ee  /tmp/tsim-eval-3.1.9.tar.gz" | sha256sum -c && \
    tar xvf /tmp/tsim-eval-3.1.9.tar.gz -C /opt && \
    rm -f /tmp/tsim-eval-3.1.9.tar.gz

# Run-time dependencies
RUN apt-get -y install libnspr4 libtinfo5 make gcc

RUN mkdir /work
WORKDIR /work

# Set system PATH
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/tsim-eval/tsim/linux-x64:/opt/bcc-2.2.3-llvm/bin:/root/.cargo/bin

# We want Rust to work in nightly-mode, because we need to build libcore
ENV RUSTC_BOOTSTRAP=1
