FROM ubuntu:22.04

RUN apt-get -y update && apt-get -y upgrade

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

# Install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh && \
    bash /tmp/rustup.sh -y && \
    rm /tmp/rustup.sh && \
    /root/.cargo/bin/rustup component add rust-src

# Set system PATH
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/tsim-eval/tsim/linux-x64:/opt/bcc-2.2.3-llvm/bin:/root/.cargo/bin

# Set up passwordless sudo
RUN echo 'ALL            ALL = (ALL) NOPASSWD: ALL' >> /etc/sudoers
