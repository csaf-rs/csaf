/**
 * Header component for the CSAF Validator application.
 */
function Header() {
  return (
    <header className="mb-8">
      <h1 className="text-4xl font-bold text-gray-900 mb-2">
        🛡️ CSAF Validator
      </h1>
      <p className="text-gray-600">
        Validate CSAF documents in your browser. All validation happens
        client-side (using WebAssembly) - your documents never leave your
        machine.
      </p>
    </header>
  );
}

export default Header;
