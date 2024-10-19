FROM ubuntu:22.04

ARG DEBIAN_FRONTEND=noninteractive

RUN --mount=type=cache,target=/var/cache/apt \
    apt update && \
    apt install -y bash \
                   build-essential \
                   git \
                   curl \
                   wget \
                   zsh \
                   ca-certificates \
                   unzip \        
                   python3.10 \
                   python3-pip \
                   python3.10-venv && \
    ln -s /usr/bin/python3.10 /usr/bin/python
RUN python -m pip install --no-cache-dir --upgrade pip 

# pip install
COPY .devcontainer/requirements.txt /tmp/requirements.txt
RUN  --mount=type=cache,target=/root/.cache/pip python -m pip install -r /tmp/requirements.txt

# zsh plugin
ENV SHELL /bin/zsh
COPY .devcontainer/zsh-plugins/.p10k.zsh .devcontainer/zsh-plugins/ohmyzsh.sh /root/
RUN chmod +x /root/ohmyzsh.sh && sh -c /root/ohmyzsh.sh

RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y
