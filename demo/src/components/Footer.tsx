function Footer() {
  return (
    <footer className="mt-12 text-center text-gray-500 text-sm">
      <p>Powered by Rust 🦀 and WebAssembly ⚡</p>
      <p className="mt-1">
        <a
          href="https://github.com/csaf-poc/csaf-rust"
          target="_blank"
          rel="noopener noreferrer"
          className="text-blue-600 hover:text-blue-800"
        >
          View on GitHub
        </a>
      </p>
    </footer>
  );
}

export default Footer;
