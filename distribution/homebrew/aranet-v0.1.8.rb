# Homebrew formula for Aranet CLI
# This file is automatically updated by the release workflow.
#
# Users will install with:
#   brew tap cameronrye/aranet
#   brew install aranet
#
# Or directly:
#   brew install cameronrye/aranet/aranet

class Aranet < Formula
  desc "CLI for Aranet environmental sensors (CO2, radon, temperature, humidity)"
  homepage "https://github.com/cameronrye/aranet"
  version "0.1.8"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/cameronrye/aranet/releases/download/v0.1.8/aranet-cli-aarch64-apple-darwin.tar.xz"
      sha256 "ee54d88ddd2de98bab1e27a468cd4e14fb758dd1fe75ba4216eac848c8bd8022"
    end
    on_intel do
      url "https://github.com/cameronrye/aranet/releases/download/v0.1.8/aranet-cli-x86_64-apple-darwin.tar.xz"
      sha256 "dedaea9286b28ed4a863197c49b17024dbcb14240a4353d9d42e625fc6915953"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/cameronrye/aranet/releases/download/v0.1.8/aranet-cli-x86_64-unknown-linux-gnu.tar.xz"
      sha256 "e122d59f9437c7522d263ce67aa9592e39b600f63603918b1c21c8f1f8e73397"
    end
  end

  def install
    bin.install "aranet"
  end

  def caveats
    <<~EOS
      Aranet CLI requires Bluetooth access.

      On macOS:
        Grant Bluetooth permission when prompted, or add Terminal/iTerm
        to System Preferences > Privacy & Security > Bluetooth.

      Quick start:
        aranet scan              # Find nearby devices
        aranet read              # Read current values
        aranet watch             # Continuous monitoring
    EOS
  end

  test do
    # Test that the binary runs and shows help
    assert_match "CLI for Aranet environmental sensors", shell_output("#{bin}/aranet --help")
    
    # Test version output
    assert_match version.to_s, shell_output("#{bin}/aranet --version")
  end
end

