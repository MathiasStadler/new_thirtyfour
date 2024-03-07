#!/usr/bin/bash
# /w sound
# docker run -it \
# --security-opt apparmor=unconfined \
# --cap-add SYS_ADMIN \
# --cap-add CAP_SYSLOG \
# --tmpfs /run \
# --volume /sys/fs/cgroup:/sys/fs/cgroup \
# --volume /dev/shm:/dev/shm \
# --device /dev/snd \
# --device /dev/dri \
# --volume /sys/fs/cgroup:/sys/fs/cgroup \
# --volume /dev/shm:/dev/shm \
# --volume ./workspace:/home/user/workspace:rw \
# --env PULSE_SERVER=unix:${XDG_RUNTIME_DIR}/pulse/native \
# --volume ${XDG_RUNTIME_DIR}/pulse/native:${XDG_RUNTIME_DIR}/pulse/native \
# --volume ~/.config/pulse/cookie:/root/.config/pulse/cookie \
# --group-add $(getent group audio | cut -d: -f3) \
# --entrypoint /usr/bin/bash \
# debian_lxde_chrome 


# docker run -it \
# --security-opt apparmor=unconfined \
# --cap-add SYS_ADMIN \
# --cap-add CAP_SYSLOG \
# --tmpfs /run \
# --volume /sys/fs/cgroup:/sys/fs/cgroup \
# --volume /dev/shm:/dev/shm \
# --device /dev/snd \
# --device /dev/dri \
# --volume /sys/fs/cgroup:/sys/fs/cgroup \
# --volume /dev/shm:/dev/shm \
# --volume ./workspace:/home/user/workspace:rw \
# --env PULSE_SERVER="unix:${XDG_RUNTIME_DIR}/pulse/native" \
# --volume "${XDG_RUNTIME_DIR}/pulse/native:${XDG_RUNTIME_DIR}/pulse/native" \
# --volume ~/.config/pulse/cookie:/root/.config/pulse/cookie \
# --group-add "$(getent group audio | cut -d: -f3)" \
# debian_lxde_chrome bash -c "id -u && id -g && apt update && apt install --yes x11-apps && sh +x /home/user/workspace/bootstrap.sh  && bash" 


# docker run -it \
# --security-opt apparmor=unconfined \
# --cap-add SYS_ADMIN \
# --cap-add CAP_SYSLOG \
# --tmpfs /run \
# --volume /sys/fs/cgroup:/sys/fs/cgroup \
# --volume /dev/shm:/dev/shm \
# --device /dev/snd \
# --device /dev/dri \
# --volume /sys/fs/cgroup:/sys/fs/cgroup \
# --volume /dev/shm:/dev/shm \
# --volume ./workspace:/home/user/workspace:rw \
# --env PULSE_SERVER=unix:${XDG_RUNTIME_DIR}/pulse/native \
# --volume ${XDG_RUNTIME_DIR}/pulse/native:${XDG_RUNTIME_DIR}/pulse/native \
# --volume ~/.config/pulse/cookie:/root/.config/pulse/cookie \
# --group-add $(getent group audio | cut -d: -f3) \
# debian_lxde_chrome_rust /usr/bin/bash  

# create ip6 network for docker container
# docker network create --ipv6 --subnet 2001:0DB8::/112 ip6net

# ip6 fro chromedriver
# https://docs.docker.com/config/daemon/ipv6/
#/w sound
#/w ip6
docker run -it \
--security-opt apparmor=unconfined \
--cap-add SYS_ADMIN \
--cap-add CAP_SYSLOG \
--tmpfs /run \
--volume /sys/fs/cgroup:/sys/fs/cgroup \
--volume /dev/shm:/dev/shm \
--device /dev/snd \
--device /dev/dri \
--volume /sys/fs/cgroup:/sys/fs/cgroup \
--volume /dev/shm:/dev/shm \
--volume ./workspace:/home/user/workspace:rw \
--env PULSE_SERVER=unix:${XDG_RUNTIME_DIR}/pulse/native \
--volume ${XDG_RUNTIME_DIR}/pulse/native:${XDG_RUNTIME_DIR}/pulse/native \
--volume ~/.config/pulse/cookie:/root/.config/pulse/cookie \
--group-add $(getent group audio | cut -d: -f3) \
--network ip6net \
debian_lxde_chrome_rust /usr/bin/bash


# docker rm  $(docker ps -a -q)
# start script
# sudo sh +x ./src/start_container_lxde_chrome_rust_entrypoint_bash.sh
# sudo sh +x ./start_container_lxde_chrome_rust_entrypoint_bash.sh

# ps -xaaue
# ps axww
# ps aaxwwue
# ps -efw

# apt install notification-daemon
