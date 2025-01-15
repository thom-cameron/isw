{
  lib,
  fetchFromGitLab,
  rustPlatform,
}:
rustPlatform.buildRustPackage rec {
  pname = "isw";
  version = "0.3.3";

  src = fetchFromGitLab {
    owner = "thom-cameron";
    repo = pname;
    rev = version;
    hash = "sha256-jw6uoMNdU7sm0CF5199xyQtFJB98vP0TlzDjWzw+TPw=";
  };

  cargoHash = "sha256-Qrs+7GIlL/ODu1kBnJO7IQYkaxbdlWJ9/TPIDYfE1bI=";

  meta = {
    description = "a simple terminal stopwatch application";
    homepage = "https://gitlab.com/thom-cameron/isw";
    license = lib.licenses.gpl3Only;
    maintainers = with lib.maintainers; [thom-cameron];
    mainProgram = "isw";
  };
}
