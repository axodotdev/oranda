/* Code modified from the blender website
 * https://www.blender.org/wp-content/themes/bthree/assets/js/get_os.js?x82196
 */

let options = {
  windows: {
    default: "windows",
    64: "windows-64",
    arm: "windows-arm",
  },
  mac: {
    default: "macos",
    ppc: "macos-PPC",
    32: "macos-32",
    silicon: "macos-apple-silicon",
  },
  linux: {
    default: "linux",
    ubuntu: "linux-ubuntu",
    debian: "linux-debian",
    mandriva: "linux-mandriva",
    redhat: "linux-redhat",
    fedora: "linux-fedora",
    suse: "linux-suse",
    gentoo: "linux-gentoo",
  },
  phone: {
    ios: "ios",
    android: "linux-android",
  },
  freebsd: "freebsd",
};

function isAppleSilicon() {
  try {
    var glcontext = document.createElement("canvas").getContext("webgl");
    var debugrenderer = glcontext
      ? glcontext.getExtension("WEBGL_debug_renderer_info")
      : null;
    var renderername =
      (debugrenderer &&
        glcontext.getParameter(debugrenderer.UNMASKED_RENDERER_WEBGL)) ||
      "";
    if (renderername.match(/Apple M/) || renderername.match(/Apple GPU/)) {
      return true;
    }

    return false;
  } catch (e) {}
}

function getOS() {
  var OS = options.windows.default;
  var userAgent = navigator.userAgent;
  var platform = navigator.platform;

  if (navigator.appVersion.includes("Win")) {
    if (
      !userAgent.includes("Windows NT 5.0") &&
      !userAgent.includes("Windows NT 5.1") &&
      (userAgent.indexOf("Win64") > -1 ||
        platform == "Win64" ||
        userAgent.indexOf("x86_64") > -1 ||
        userAgent.indexOf("x86_64") > -1 ||
        userAgent.indexOf("amd64") > -1 ||
        userAgent.indexOf("AMD64") > -1 ||
        userAgent.indexOf("WOW64") > -1)
    ) {
      OS = options.windows[64];
    } else {
      if (
        window.external &&
        window.external.getHostEnvironmentValue &&
        window.external
          .getHostEnvironmentValue("os-architecture")
          .includes("ARM64")
      ) {
        OS = options.windows.arm;
      } else {
        try {
          var canvas = document.createElement("canvas");
          var gl = canvas.getContext("webgl");

          var debugInfo = gl.getExtension("WEBGL_debug_renderer_info");
          var renderer = gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL);
          if (renderer.includes("Qualcomm")) OS = options.windows.arm;
        } catch (e) {}
      }
    }
  }

  //MacOS, MacOS X, macOS
  if (navigator.appVersion.includes("Mac")) {
    if (platform.includes("MacPPC") || platform.includes("PowerPC")) {
      OS = options.mac.ppc;
    } else if (
      navigator.userAgent.includes("OS X 10.5") ||
      navigator.userAgent.includes("OS X 10.6")
    ) {
      OS = options.mac[32];
    } else {
      OS = options.mac.default;

      const isSilicon = isAppleSilicon();
      if (isSilicon) {
        OS = options.mac.silicon;
      }
    }
  }

  // linux
  if (platform.includes("Linux")) {
    if (navigator.userAgent.toLocaleLowerCase().includes("ubuntu"))
      OS = options.linux.ubuntu;
    else if (userAgent.includes("Debian")) OS = options.linux.debian;
    else if (userAgent.includes("Android")) OS = options.phone.android;
    else if (userAgent.includes("Mandriva")) OS = options.linux.mandriva;
    else if (userAgent.includes("Red Hat")) OS = options.linux.redhat;
    else if (userAgent.includes("Fedora")) OS = options.linux.fedora;
    else if (userAgent.includes("SUSE")) OS = options.linux.suse;
    else if (userAgent.includes("Gentoo")) OS = options.linux.gentoo;
    else OS = options.linux.default;
  }

  if (
    userAgent.includes("iPad") ||
    userAgent.includes("iPhone") ||
    userAgent.includes("iPod")
  ) {
    OS = options.phone.ios;
  }
  if (platform.toLocaleLowerCase().includes("freebsd")) {
    OS = options.freebsd;
  }

  return OS;
}

window.os = getOS();
