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
    hash = "sha256-1ULtKhjYgpmYdlGnQVa3T79r8uZ4BL4fRqI2VJWnNm8=";
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
