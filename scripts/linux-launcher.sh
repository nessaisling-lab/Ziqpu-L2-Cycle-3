#!/usr/bin/env sh
# Ziqpu launcher for the Linux tarball.
#
# WHY THIS EXISTS
#
# The desktop app links WebKitGTK, which is a *system* library on Linux — it is not, and cannot
# reasonably be, bundled in the tarball. On a machine without it, running the binary directly gives:
#
#   ./ziqpu-ui: error while loading shared libraries: libwebkit2gtk-4.1.so.0:
#   cannot open shared object file: No such file or directory
#
# The fix is in README.txt beside the binary. That only helps someone who opens it AND connects a
# linker error to a packaging note. Everyone else has an app that "doesn't work".
#
# So the shipped entry point checks first and says the one thing worth saying: exactly which library
# is missing, and the exact command for the distribution actually in front of them. Same reasoning as
# the Windows build detecting WebView2 rather than letting a blank window be the message.
#
# It checks with `ldd`, so it reports ANY missing shared object rather than only the one we expected
# — a guess about which dependency will be absent is a guess, and the loader already knows.

set -eu

here=$(cd "$(dirname "$0")" && pwd)
bin="$here/ziqpu-ui"

if [ ! -x "$bin" ]; then
    echo "Ziqpu: cannot find the executable at $bin" >&2
    echo "Extract the whole archive and run ./ziqpu from inside it." >&2
    exit 1
fi

# Ask the loader what it cannot resolve. If `ldd` is itself unavailable, skip the check rather than
# refuse to start — a missing diagnostic tool is no reason to block a machine that may be fine.
if command -v ldd >/dev/null 2>&1; then
    missing=$(ldd "$bin" 2>/dev/null | awk '/not found/ { print $1 }' | sort -u)
else
    missing=""
fi

if [ -n "$missing" ]; then
    # Name the distribution from os-release, falling back to a generic message rather than guessing
    # a package manager that is not there.
    id=""
    like=""
    if [ -r /etc/os-release ]; then
        # shellcheck disable=SC1091
        . /etc/os-release
        id="${ID:-}"
        like="${ID_LIKE:-}"
    fi

    case "$id $like" in
        *debian* | *ubuntu*) fix="sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0" ;;
        *fedora* | *rhel* | *centos*) fix="sudo dnf install webkit2gtk4.1 gtk3" ;;
        *arch*) fix="sudo pacman -S webkit2gtk-4.1 gtk3" ;;
        *suse*) fix="sudo zypper install libwebkit2gtk-4_1-0 gtk3" ;;
        *) fix="" ;;
    esac

    echo "Ziqpu needs system libraries this machine does not have:" >&2
    echo "$missing" | sed 's/^/  - /' >&2
    echo "" >&2
    if [ -n "$fix" ]; then
        echo "On this system, install them with:" >&2
        echo "  $fix" >&2
    else
        echo "Install WebKitGTK 4.1 and GTK 3 with your package manager." >&2
        echo "Debian/Ubuntu:  sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0" >&2
        echo "Fedora:         sudo dnf install webkit2gtk4.1 gtk3" >&2
        echo "Arch:           sudo pacman -S webkit2gtk-4.1 gtk3" >&2
    fi
    echo "" >&2
    echo "Then run ./ziqpu again. Nothing was installed or changed by this check." >&2
    exit 1
fi

exec "$bin" "$@"
