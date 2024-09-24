{
  lib,
  fetchFromGitLab,
  rustPlatform,
}:
rustPlatform.buildRustPackage rec {
  pname = "isw";
  version = "0.2.0";

  src = fetchFromGitLab {
    owner = "thom-cameron";
    repo = pname;
    rev = version;
    hash = "sha256-PrdF97bC7SyZC7ReN5hLREccTRbifXiatuzHkQbk04k=";
  };

  cargoHash = "sha256-TeCPgVC7iPKnUfZxMJFTKuOF8bLuh3NSpcmm0VZEr+g=";

  meta = with lib; {
    description = "a simple terminal stopwatch application";
    homepage = "https://gitlab.com/thom-cameron/isw";
    license = licenses.gpl3Only;
    maintainers = with maintainers; [thom-cameron];
  };
}
