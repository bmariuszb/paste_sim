FROM artixlinux/runit
RUN pacman --noconfirm -Syu hyprland
ENV XDG_RUNTIME_DIR=/run/user/1000
CMD Hyprland
