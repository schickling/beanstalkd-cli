class BeanstalkdCli < Formula
  homepage "https://github.com/schickling/beanstalkd-cli"
  url "https://github.com/schickling/beanstalkd-cli/archive/0.0.1.tar.gz"
  sha1 "7f2eae42055036749f775893d229586fa2e15df6"

  depends_on "cargo"

  def install
    system "cargo", "build"
    bin.install "target/beanstalkd-cli"
  end

  test do
    system "#{bin}/beanstalkd-cli", "-v"
  end
end
